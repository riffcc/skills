use crate::{Mask, ModelProvider};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Get mask search paths in priority order:
/// 1. ~/.claude/masks/ (private, highest priority)
/// 2. ./masks/ (project repo masks)
fn get_mask_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // Priority 1: Private masks in ~/.claude/masks/
    if let Some(home) = std::env::var_os("HOME") {
        let private_masks = PathBuf::from(home).join(".claude/masks");
        paths.push(private_masks);
    }

    // Priority 2: Project repo masks
    paths.push(PathBuf::from("masks"));

    paths
}

pub fn load_mask_from_file(specialty: &str, model: &str) -> Result<Mask> {
    let search_paths = get_mask_search_paths();
    let mut tried_paths = Vec::new();

    // Try each search path in priority order
    for base_path in search_paths {
        let mask_path = base_path.join(specialty).join(format!("{}.md", model));
        tried_paths.push(mask_path.display().to_string());

        if mask_path.exists() {
            let content = fs::read_to_string(&mask_path)
                .with_context(|| format!("Failed to read mask file: {}", mask_path.display()))?;

            let model_provider = parse_model_provider(model)?;

            return Ok(Mask::new(specialty.to_string(), model_provider, content));
        }
    }

    anyhow::bail!(
        "Mask not found: {}/{}. Tried paths:\n  {}",
        specialty,
        model,
        tried_paths.join("\n  ")
    )
}

fn parse_model_provider(model: &str) -> Result<ModelProvider> {
    match model {
        "sonnet" => Ok(ModelProvider::ClaudeSonnet),
        "haiku" => Ok(ModelProvider::ClaudeHaiku),
        model if model.starts_with("openrouter/") => {
            let model_id = model.strip_prefix("openrouter/").unwrap().to_string();
            Ok(ModelProvider::OpenRouter { model_id })
        }
        model if model.starts_with("zai/") => {
            let model_id = model.strip_prefix("zai/").unwrap().to_string();
            Ok(ModelProvider::ZAi { model_id })
        }
        model if model.starts_with("codex/") => {
            let model_id = model.strip_prefix("codex/").unwrap().to_string();
            Ok(ModelProvider::Codex { model_id })
        }
        _ => anyhow::bail!("Unknown model provider: {}", model),
    }
}

pub fn list_masks() -> Result<Vec<(String, String, String)>> {
    let search_paths = get_mask_search_paths();
    let mut masks = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // Search all mask directories (private has precedence due to order)
    for base_path in search_paths {
        if !base_path.exists() {
            continue;
        }

        let source = if base_path.ends_with(".claude/masks") {
            "private"
        } else {
            "repo"
        };

        for specialty_entry in fs::read_dir(&base_path).unwrap_or_else(|_| fs::read_dir(".").unwrap()) {
            let specialty_entry = match specialty_entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if !specialty_entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                continue;
            }

            let specialty = specialty_entry.file_name().to_string_lossy().to_string();

            for mask_entry in fs::read_dir(specialty_entry.path()).unwrap_or_else(|_| fs::read_dir(".").unwrap()) {
                let mask_entry = match mask_entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                if let Some(filename) = mask_entry.file_name().to_str() {
                    if filename.ends_with(".md") {
                        let model = filename.strip_suffix(".md").unwrap().to_string();
                        let key = format!("{}/{}", specialty, model);

                        // Only add if not already seen (private masks override repo masks)
                        if seen.insert(key) {
                            masks.push((specialty.clone(), model, source.to_string()));
                        }
                    }
                }
            }
        }
    }

    Ok(masks)
}
