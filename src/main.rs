use clap::{Parser, Subcommand};
use colored::*;
use palace_skills::*;

#[derive(Parser)]
#[command(name = "palace-skills")]
#[command(about = "RHSI - Recursive Hierarchical Self Improvement", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available masks
    List,
    /// Load and display a mask
    Load {
        /// Specialty (e.g., "mask-improver", "distributed-systems")
        specialty: String,
        /// Model (e.g., "sonnet", "haiku", "openrouter/...")
        model: String,
    },
    /// Preview what a mask would do (pseudo-instruction, lightweight)
    Preview {
        /// Specialty
        specialty: String,
        /// Model
        model: String,
    },
    /// Convert mask to Claude Skill file
    ToSkill {
        /// Specialty
        specialty: String,
        /// Model
        model: String,
        /// Output path (defaults to ~/.claude/skills/{specialty}-{model}.md)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Create a new private mask
    New {
        /// Specialty name
        specialty: String,
        /// Model (defaults to "sonnet")
        #[arg(short, long, default_value = "sonnet")]
        model: String,
        /// Create in repo instead of private location
        #[arg(short, long)]
        repo: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            println!("{}", "Available Masks:".bold().green());
            println!();

            let masks = list_masks()?;

            if masks.is_empty() {
                println!("{}", "No masks found".yellow());
                println!("  Searched: ~/.claude/masks/ and ./masks/".dimmed());
                return Ok(());
            }

            let count = masks.len();
            for (specialty, model, source) in masks {
                let badge = if source == "private" {
                    "[private]".magenta()
                } else {
                    "[repo]".dimmed()
                };
                println!(
                    "  {} {} {}",
                    "●".blue(),
                    format!("{}/{}", specialty, model).cyan(),
                    badge
                );
            }

            println!();
            println!("{}", format!("Total: {} masks", count).dimmed());
        }

        Commands::Load { specialty, model } => {
            println!(
                "{}",
                format!("Loading mask: {}/{}", specialty, model)
                    .bold()
                    .cyan()
            );
            println!();

            let mask = load_mask_from_file(&specialty, &model)?;

            println!("{}", "=".repeat(80).dimmed());
            println!("{}", mask.content);
            println!("{}", "=".repeat(80).dimmed());
            println!();

            println!("{}", "Mask Details:".bold().green());
            println!("  {}: {}", "Specialty".dimmed(), specialty);
            println!("  {}: {:?}", "Model".dimmed(), mask.model);
            println!("  {}: v{}", "Version".dimmed(), mask.version);
            println!("  {}: {}", "ID".dimmed(), mask.id);
        }

        Commands::Preview { specialty, model } => {
            println!(
                "{}",
                format!("Preview: {}/{}", specialty, model).bold().cyan()
            );
            println!();

            let mask = load_mask_from_file(&specialty, &model)?;

            // Extract identity and core expertise from mask content
            println!("{}", "When loaded, you would become:".bold().green());
            println!();

            // Simple preview - extract first sections
            let lines: Vec<&str> = mask.content.lines().collect();
            let mut in_identity = false;
            let mut in_expertise = false;

            for line in lines {
                if line.starts_with("## Identity") {
                    in_identity = true;
                    in_expertise = false;
                    continue;
                } else if line.starts_with("## Core Expertise") {
                    in_identity = false;
                    in_expertise = true;
                    continue;
                } else if line.starts_with("##") {
                    in_identity = false;
                    in_expertise = false;
                }

                if in_identity && !line.is_empty() {
                    println!("{}", line.cyan());
                }
                if in_expertise && !line.is_empty() {
                    println!("{}", line.yellow());
                }
            }

            println!();
            println!(
                "{}",
                "Use 'to-skill' to create a Claude Skill from this mask".dimmed()
            );
        }

        Commands::ToSkill {
            specialty,
            model,
            output,
        } => {
            let mask = load_mask_from_file(&specialty, &model)?;

            let skill_name = specialty.clone();
            let skill_dir = output.unwrap_or_else(|| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                format!("{}/.claude/skills/{}", home, skill_name)
            });

            // Create skill directory
            std::fs::create_dir_all(&skill_dir)?;

            // Write SKILL.md inside the directory
            let skill_path = format!("{}/SKILL.md", skill_dir);
            std::fs::write(&skill_path, &mask.content)?;

            println!("{}", "✓ Claude Skill created!".bold().green());
            println!();
            println!("  {}: {}", "Skill directory".dimmed(), skill_dir.cyan());
            println!("  {}: {}", "Skill file".dimmed(), skill_path.cyan());
            println!("  {}: {}", "Specialty".dimmed(), specialty);
            println!("  {}: {:?}", "Model".dimmed(), mask.model);
            println!();
            println!("{}", "Usage:".bold());
            println!("  Claude will auto-invoke when relevant");
            println!(
                "  Or manually: {}",
                format!("Skill tool with command '{}'", skill_name).dimmed()
            );
        }

        Commands::New {
            specialty,
            model,
            repo,
        } => {
            let base_dir = if repo {
                "masks".to_string()
            } else {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                format!("{}/.claude/masks", home)
            };

            let mask_dir = format!("{}/{}", base_dir, specialty);
            std::fs::create_dir_all(&mask_dir)?;

            let mask_path = format!("{}/{}.md", mask_dir, model);

            if std::path::Path::new(&mask_path).exists() {
                anyhow::bail!("Mask already exists: {}", mask_path);
            }

            let template = create_mask_template(&specialty);
            std::fs::write(&mask_path, template)?;

            println!("{}", "✓ New mask created!".bold().green());
            println!();
            println!("  {}: {}", "Location".dimmed(), mask_path.cyan());
            println!(
                "  {}: {}",
                "Type".dimmed(),
                if repo {
                    "repo (will be committed)".yellow()
                } else {
                    "private (never committed)".magenta()
                }
            );
            println!();
            println!("{}", "Next steps:".bold());
            println!("  1. Edit the mask: {}", format!("vim {}", mask_path).cyan());
            println!("  2. Preview it: {}", format!("palace-skills preview {} {}", specialty, model).cyan());
            println!("  3. Install as skill: {}", format!("palace-skills to-skill {} {}", specialty, model).cyan());
        }
    }

    Ok(())
}

fn create_mask_template(specialty: &str) -> String {
    format!(
        r#"---
name: {}
description: Expert in <FILL IN>. Use when <FILL IN>.
---

# {} - Claude Sonnet

## Identity

You are the **{}**, a specialist in <FILL IN YOUR DOMAIN>.

Your role is to <FILL IN YOUR MISSION>.

## Core Expertise

- **<Domain 1>:** <Description>
- **<Domain 2>:** <Description>
- **<Domain 3>:** <Description>

## Your Mission

When given a task related to {}, you:

1. **<Step 1>**
   - <Details>
   - <Specific actions>

2. **<Step 2>**
   - <Details>
   - <Specific actions>

3. **<Step 3>**
   - <Details>
   - <Specific actions>

## Behavioral Guidelines

- **Be Specific:** <Example of good vs bad>
- **Be <Quality>:** <Explanation>
- **Be <Quality>:** <Explanation>

## Examples

**Example 1:**
- **Input:** <What comes in>
- **Output:** <What you produce>

**Example 2:**
- **Input:** <What comes in>
- **Output:** <What you produce>

## Improvement Notes

### Version 1 (YYYY-MM-DD)
Initial creation. Core capabilities:
- <Capability 1>
- <Capability 2>
- <Capability 3>
"#,
        specialty,
        specialty,
        specialty.replace("-", " ").chars().enumerate().map(|(i, c)| {
            if i == 0 || specialty.chars().nth(i - 1) == Some('-') {
                c.to_uppercase().next().unwrap()
            } else {
                c
            }
        }).collect::<String>(),
        specialty
    )
}
