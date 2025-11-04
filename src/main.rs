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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            println!("{}", "Available Masks:".bold().green());
            println!();

            let masks = list_masks()?;

            if masks.is_empty() {
                println!("{}", "No masks found in masks/ directory".yellow());
                return Ok(());
            }

            let count = masks.len();
            for (specialty, model) in masks {
                println!("  {} {}", "●".blue(), format!("{}/{}", specialty, model).cyan());
            }

            println!();
            println!("{}", format!("Total: {} masks", count).dimmed());
        }

        Commands::Load { specialty, model } => {
            println!("{}", format!("Loading mask: {}/{}", specialty, model).bold().cyan());
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
            println!("{}", format!("Preview: {}/{}", specialty, model).bold().cyan());
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
            println!("{}", "Use 'to-skill' to create a Claude Skill from this mask".dimmed());
        }

        Commands::ToSkill { specialty, model, output } => {
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
            println!("  Or manually: {}", format!("Skill tool with command '{}'", skill_name).dimmed());
        }
    }

    Ok(())
}
