# `pal skill release` - Design Specification

**Purpose:** Automate the release process for Claude Skills from palace-skills repository

---

## Command Interface

```bash
# Release a specific skill
pal skill release <skill-name> [OPTIONS]

# Examples:
pal skill release competition-math-researcher
pal skill release mask-improver --major  # Bump major version
pal skill release playwright-tester --version 2.1.0  # Explicit version
```

### Options

- `--version <VERSION>` - Explicit version number (e.g., "1.2.0")
- `--major` - Bump major version (1.0.0 → 2.0.0)
- `--minor` - Bump minor version (1.0.0 → 1.1.0) [default]
- `--patch` - Bump patch version (1.0.0 → 1.0.1)
- `--draft` - Create draft release
- `--prerelease` - Mark as pre-release
- `--notes <FILE>` - Custom release notes file
- `--dry-run` - Show what would be released without doing it

---

## Workflow

### Step 1: Detect Current Version

```rust
// Read version from mask file's "Improvement Notes" section
fn get_current_version(skill_name: &str) -> Result<Version> {
    let mask_path = format!("masks/{}/sonnet.md", skill_name);
    let content = fs::read_to_string(mask_path)?;

    // Parse version from "### Version X.Y.Z" in Improvement Notes
    let version_regex = Regex::new(r"###\s+Version\s+(\d+)\.(\d+)\.?(\d*)")?;

    // Find latest version
    let mut versions: Vec<Version> = vec![];
    for cap in version_regex.captures_iter(&content) {
        versions.push(Version::new(
            cap[1].parse()?,
            cap[2].parse()?,
            cap.get(3).and_then(|m| m.as_str().parse().ok()).unwrap_or(0)
        ));
    }

    versions.sort();
    versions.last().cloned().ok_or("No version found")
}
```

### Step 2: Determine Next Version

```rust
fn determine_next_version(
    current: Version,
    bump: VersionBump,
    explicit: Option<Version>
) -> Version {
    if let Some(v) = explicit {
        return v;
    }

    match bump {
        VersionBump::Major => Version::new(current.major + 1, 0, 0),
        VersionBump::Minor => Version::new(current.major, current.minor + 1, 0),
        VersionBump::Patch => Version::new(current.major, current.minor, current.patch + 1),
    }
}
```

### Step 3: Package Skill

```rust
fn package_skill(skill_name: &str, version: &Version) -> Result<PathBuf> {
    let temp_dir = TempDir::new()?;
    let package_dir = temp_dir.path().join(format!("{}-v{}", skill_name, version));

    // 1. Create SKILL.md (Claude Skills format)
    create_skill_md(&package_dir, skill_name)?;

    // 2. Copy README.md (create from mask content)
    create_readme(&package_dir, skill_name, version)?;

    // 3. Copy source mask
    copy_mask(&package_dir, skill_name)?;

    // 4. Copy benchmarks (if exist)
    copy_benchmarks(&package_dir, skill_name)?;

    // 5. Copy test problems (if exist)
    copy_test_problems(&package_dir, skill_name)?;

    // 6. Create ZIP
    let zip_path = format!("releases/{}-v{}.zip", skill_name, version);
    create_zip(&package_dir, &zip_path)?;

    Ok(PathBuf::from(zip_path))
}
```

### Step 4: Generate Release Notes

```rust
fn generate_release_notes(
    skill_name: &str,
    version: &Version,
    custom_notes: Option<PathBuf>
) -> Result<String> {
    if let Some(notes_file) = custom_notes {
        return fs::read_to_string(notes_file);
    }

    // Auto-generate from Improvement Notes
    let mask_content = load_mask(skill_name)?;
    let version_section = extract_version_section(&mask_content, version)?;

    format!(
        "# {} v{}\n\n{}\n\n## Installation\n\n[...]\n\n## Links\n\n- Repository: https://github.com/riffcc/palace-skills\n",
        skill_name,
        version,
        version_section
    )
}
```

### Step 5: Create Git Tag

```rust
fn create_git_tag(skill_name: &str, version: &Version, notes: &str) -> Result<()> {
    let tag_name = format!("{}-v{}", skill_name, version);

    Command::new("git")
        .args(["tag", "-a", &tag_name, "-m", notes])
        .status()?;

    Command::new("git")
        .args(["push", "origin", &tag_name])
        .status()?;

    Ok(())
}
```

### Step 6: Create GitHub Release

```rust
fn create_github_release(
    skill_name: &str,
    version: &Version,
    zip_path: &Path,
    notes: &str,
    opts: &ReleaseOptions
) -> Result<()> {
    let tag_name = format!("{}-v{}", skill_name, version);

    // Try gh CLI first
    if let Ok(_) = which::which("gh") {
        let mut cmd = Command::new("gh");
        cmd.args([
            "release", "create", &tag_name,
            zip_path.to_str().unwrap(),
            "--title", &format!("{} v{}", skill_name, version),
            "--notes", notes,
        ]);

        if opts.draft {
            cmd.arg("--draft");
        }
        if opts.prerelease {
            cmd.arg("--prerelease");
        }

        return Ok(cmd.status()?.success().then_some(()).ok_or("gh failed")?);
    }

    // Fallback: GitHub API
    create_release_via_api(tag_name, zip_path, notes, opts)
}

fn create_release_via_api(
    tag_name: String,
    zip_path: &Path,
    notes: &str,
    opts: &ReleaseOptions
) -> Result<()> {
    // Use GitHub API v3
    let client = reqwest::blocking::Client::new();
    let token = std::env::var("GITHUB_TOKEN")?;

    // Create release
    let release_response = client
        .post("https://api.github.com/repos/riffcc/palace-skills/releases")
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "tag_name": tag_name,
            "name": format!("{} v{}", skill_name, version),
            "body": notes,
            "draft": opts.draft,
            "prerelease": opts.prerelease,
        }))
        .send()?;

    let release: serde_json::Value = release_response.json()?;
    let upload_url = release["upload_url"].as_str().unwrap();

    // Upload ZIP asset
    let zip_data = fs::read(zip_path)?;
    client
        .post(upload_url.replace("{?name,label}", &format!("?name={}", zip_path.file_name().unwrap().to_str().unwrap())))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/zip")
        .body(zip_data)
        .send()?;

    Ok(())
}
```

### Step 7: Commit Release Files

```rust
fn commit_release_files(
    skill_name: &str,
    version: &Version,
    zip_path: &Path,
    notes_path: &Path
) -> Result<()> {
    Command::new("git")
        .args(["add", zip_path.to_str().unwrap(), notes_path.to_str().unwrap()])
        .status()?;

    Command::new("git")
        .args([
            "commit",
            "-m",
            &format!("Release {} v{}", skill_name, version)
        ])
        .status()?;

    Command::new("git")
        .args(["push"])
        .status()?;

    Ok(())
}
```

---

## Complete Flow

```
pal skill release competition-math-researcher
  ↓
1. Detect current version from mask (v1.1)
  ↓
2. Determine next version (v1.2 - default minor bump)
  ↓
3. Package skill into ZIP with SKILL.md
  ↓
4. Generate release notes from Improvement Notes
  ↓
5. Create git tag (competition-math-researcher-v1.2)
  ↓
6. Create GitHub release with ZIP attached
  ↓
7. Commit release files to repo
  ↓
8. Push to remote
  ↓
Done! 🎉
```

---

## File Structure

```
releases/
├── competition-math-researcher-v1.1.zip
├── competition-math-researcher-v1.2.zip
├── RELEASE-NOTES-competition-math-researcher-v1.1.md
└── RELEASE-NOTES-competition-math-researcher-v1.2.md
```

---

## Dependencies

### Required Crates

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde_json = "1.0"
regex = "1"
semver = "1.0"
which = "4.0"
tempfile = "3.8"
zip = "0.6"
```

### External Tools (Optional)

- `gh` (GitHub CLI) - Preferred for releases
- `git` - Required for tagging and pushing

### Environment Variables

- `GITHUB_TOKEN` - Required if using API fallback (no `gh` CLI)

---

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum ReleaseError {
    #[error("Mask not found: {0}")]
    MaskNotFound(String),

    #[error("No version found in mask")]
    NoVersion,

    #[error("Git command failed: {0}")]
    GitFailed(String),

    #[error("GitHub release failed: {0}")]
    ReleaseFailed(String),

    #[error("GITHUB_TOKEN not set (required when gh CLI not available)")]
    NoGitHubToken,
}
```

---

## Testing

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_version_detection() {
        let version = get_current_version("mask-improver").unwrap();
        assert_eq!(version, Version::new(4, 0, 0));
    }

    #[test]
    fn test_version_bump() {
        let current = Version::new(1, 2, 3);
        assert_eq!(
            determine_next_version(current, VersionBump::Minor, None),
            Version::new(1, 3, 0)
        );
    }

    #[test]
    fn test_package_skill() {
        let zip_path = package_skill("mask-improver", &Version::new(4, 0, 0)).unwrap();
        assert!(zip_path.exists());

        // Verify ZIP contains SKILL.md at root
        let file = File::open(zip_path).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();
        assert!(archive.by_name("SKILL.md").is_ok());
    }
}
```

---

## Future Enhancements

1. **Auto-detection of breaking changes** - Parse Improvement Notes for "BREAKING" keyword → major bump
2. **Changelog generation** - Aggregate changes across versions
3. **Cross-repository releases** - Release masks from external repos
4. **Release analytics** - Track download counts, usage statistics
5. **Automated testing before release** - Run benchmarks, validate structure

---

## Implementation Plan

### Phase 1: Core Release (This Sprint)
- [x] Manual release process (current)
- [ ] Implement `pal skill release` basic functionality
- [ ] Version detection and bumping
- [ ] ZIP packaging with SKILL.md
- [ ] Git tagging

### Phase 2: GitHub Integration (Next Sprint)
- [ ] GitHub API integration
- [ ] Release notes generation
- [ ] Asset upload
- [ ] Error handling and retry logic

### Phase 3: Advanced Features (Future)
- [ ] Auto-detection of breaking changes
- [ ] Changelog aggregation
- [ ] Pre-release validation
- [ ] Analytics integration

---

**Built for The Forge** 🔥⚒️
