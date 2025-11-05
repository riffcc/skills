use crate::{Mask, ModelProvider};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn load_mask_from_file(specialty: &str, model: &str) -> Result<Mask> {
    let path = format!("masks/{}/{}.md", specialty, model);
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read mask file: {}", path))?;

    let model_provider = parse_model_provider(model)?;

    Ok(Mask::new(specialty.to_string(), model_provider, content))
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

pub fn list_masks() -> Result<Vec<(String, String)>> {
    let masks_dir = Path::new("masks");
    if !masks_dir.exists() {
        return Ok(Vec::new());
    }

    let mut masks = Vec::new();

    for specialty_entry in fs::read_dir(masks_dir)? {
        let specialty_entry = specialty_entry?;
        if !specialty_entry.file_type()?.is_dir() {
            continue;
        }

        let specialty = specialty_entry.file_name().to_string_lossy().to_string();

        for mask_entry in fs::read_dir(specialty_entry.path())? {
            let mask_entry = mask_entry?;
            if let Some(filename) = mask_entry.file_name().to_str() {
                if filename.ends_with(".md") {
                    let model = filename.strip_suffix(".md").unwrap().to_string();
                    masks.push((specialty.clone(), model));
                }
            }
        }
    }

    Ok(masks)
}
