// Benchmark tests for music-theory-composer mask v1
// These validate that the mask has the necessary expertise to analyze and compose music

use palace_skills::*;

#[test]
fn test_music_theory_composer_structure() {
    println!("\n=== V1 BENCHMARK: Music Theory Composer - Structure ===\n");

    let mask = load_mask_from_file("music-theory-composer", "sonnet")
        .expect("Failed to load music-theory-composer mask");

    let content = &mask.content;

    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_guidelines = content.contains("## Behavioral Guidelines");
    let has_examples = content.contains("## Examples");
    let has_improvements = content.contains("## Improvement Notes");

    println!("Structure validation:");
    println!("  ✓ Identity section: {}", if has_identity { "✅" } else { "❌" });
    println!("  ✓ Core Expertise: {}", if has_expertise { "✅" } else { "❌" });
    println!("  ✓ Mission section: {}", if has_mission { "✅" } else { "❌" });
    println!("  ✓ Behavioral Guidelines: {}", if has_guidelines { "✅" } else { "❌" });
    println!("  ✓ Examples: {}", if has_examples { "✅" } else { "❌" });
    println!("  ✓ Improvement Notes: {}", if has_improvements { "✅" } else { "❌" });

    let score = [has_identity, has_expertise, has_mission, has_guidelines, has_examples, has_improvements]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nScore: {}/6", score);

    assert_eq!(score, 6, "All structure sections should be present");
}

#[test]
fn test_music_theory_composer_harmony_expertise() {
    println!("\n=== V1 BENCHMARK: Music Theory Composer - Harmony Expertise ===\n");

    let mask = load_mask_from_file("music-theory-composer", "sonnet")
        .expect("Failed to load music-theory-composer mask");

    let content = &mask.content;

    // Core harmony concepts
    let has_chord_progressions = content.contains("chord progression") || content.contains("Chord construction");
    let has_roman_numerals = content.contains("Roman numeral") || content.contains("roman numeral");
    let has_voice_leading = content.contains("voice leading") || content.contains("Voice leading");
    let has_cadences = content.contains("cadence") || content.contains("Cadence");
    let has_functional_harmony = content.contains("functional harmony") || content.contains("Functional harmony");

    println!("Harmony Expertise:");
    println!("  ✓ Chord progressions: {}", if has_chord_progressions { "✅" } else { "❌" });
    println!("  ✓ Roman numeral analysis: {}", if has_roman_numerals { "✅" } else { "❌" });
    println!("  ✓ Voice leading: {}", if has_voice_leading { "✅" } else { "❌" });
    println!("  ✓ Cadences: {}", if has_cadences { "✅" } else { "❌" });
    println!("  ✓ Functional harmony: {}", if has_functional_harmony { "✅" } else { "❌" });

    let score = [
        has_chord_progressions,
        has_roman_numerals,
        has_voice_leading,
        has_cadences,
        has_functional_harmony,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert_eq!(score, 5, "Should cover all core harmony concepts");
}

#[test]
fn test_music_theory_composer_voice_leading_rules() {
    println!("\n=== V1 BENCHMARK: Music Theory Composer - Voice Leading Rules ===\n");

    let mask = load_mask_from_file("music-theory-composer", "sonnet")
        .expect("Failed to load music-theory-composer mask");

    let content = &mask.content;

    // Critical voice leading rules
    let has_parallel_fifths = content.contains("parallel fifth") || content.contains("Parallel fifth");
    let has_parallel_octaves = content.contains("parallel octave") || content.contains("Parallel octave");
    let has_doubling = content.contains("doubling") || content.contains("Doubling");
    let has_contrary_motion = content.contains("contrary motion") || content.contains("contrary");
    let has_oblique_motion = content.contains("oblique motion") || content.contains("oblique");

    println!("Voice Leading Rules:");
    println!("  ✓ Parallel fifths (forbidden): {}", if has_parallel_fifths { "✅" } else { "❌" });
    println!("  ✓ Parallel octaves (forbidden): {}", if has_parallel_octaves { "✅" } else { "❌" });
    println!("  ✓ Proper doubling: {}", if has_doubling { "✅" } else { "❌" });
    println!("  ✓ Contrary motion: {}", if has_contrary_motion { "✅" } else { "❌" });
    println!("  ✓ Oblique motion: {}", if has_oblique_motion { "✅" } else { "❌" });

    let score = [
        has_parallel_fifths,
        has_parallel_octaves,
        has_doubling,
        has_contrary_motion,
        has_oblique_motion,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert_eq!(score, 5, "Should cover essential voice leading rules");
}

#[test]
fn test_music_theory_composer_style_knowledge() {
    println!("\n=== V1 BENCHMARK: Music Theory Composer - Style Knowledge ===\n");

    let mask = load_mask_from_file("music-theory-composer", "sonnet")
        .expect("Failed to load music-theory-composer mask");

    let content = &mask.content;

    // Style-specific knowledge
    let has_baroque = content.contains("Baroque") || content.contains("baroque");
    let has_classical = content.contains("Classical") || content.contains("classical");
    let has_romantic = content.contains("Romantic") || content.contains("romantic");
    let has_jazz = content.contains("Jazz") || content.contains("jazz");
    let has_contemporary = content.contains("Contemporary") || content.contains("contemporary") || content.contains("modern");

    println!("Style Knowledge:");
    println!("  ✓ Baroque style: {}", if has_baroque { "✅" } else { "❌" });
    println!("  ✓ Classical style: {}", if has_classical { "✅" } else { "❌" });
    println!("  ✓ Romantic style: {}", if has_romantic { "✅" } else { "❌" });
    println!("  ✓ Jazz style: {}", if has_jazz { "✅" } else { "❌" });
    println!("  ✓ Contemporary style: {}", if has_contemporary { "✅" } else { "❌" });

    let score = [
        has_baroque,
        has_classical,
        has_romantic,
        has_jazz,
        has_contemporary,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert!(score >= 4, "Should cover at least 4 major style periods");
}

#[test]
fn test_music_theory_composer_counterpoint() {
    println!("\n=== V1 BENCHMARK: Music Theory Composer - Counterpoint ===\n");

    let mask = load_mask_from_file("music-theory-composer", "sonnet")
        .expect("Failed to load music-theory-composer mask");

    let content = &mask.content;

    // Counterpoint knowledge
    let has_species_counterpoint = content.contains("species counterpoint") || content.contains("Species counterpoint");
    let has_consonance_dissonance = content.contains("consonance") && content.contains("dissonance");
    let has_independent_lines = content.contains("independent") && (content.contains("line") || content.contains("voice") || content.contains("melodic"));
    let has_fugue_or_canon = content.contains("fugue") || content.contains("canon") || content.contains("Fugue") || content.contains("Canon");

    println!("Counterpoint Knowledge:");
    println!("  ✓ Species counterpoint: {}", if has_species_counterpoint { "✅" } else { "❌" });
    println!("  ✓ Consonance/dissonance: {}", if has_consonance_dissonance { "✅" } else { "❌" });
    println!("  ✓ Independent melodic lines: {}", if has_independent_lines { "✅" } else { "❌" });
    println!("  ✓ Fugue or canon: {}", if has_fugue_or_canon { "✅" } else { "❌" });

    let score = [
        has_species_counterpoint,
        has_consonance_dissonance,
        has_independent_lines,
        has_fugue_or_canon,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/4", score);

    assert!(score >= 3, "Should have solid counterpoint knowledge");
}

#[test]
fn test_music_theory_composer_form_analysis() {
    println!("\n=== V1 BENCHMARK: Music Theory Composer - Form Analysis ===\n");

    let mask = load_mask_from_file("music-theory-composer", "sonnet")
        .expect("Failed to load music-theory-composer mask");

    let content = &mask.content;

    // Musical form knowledge
    let has_binary_ternary = (content.contains("Binary") || content.contains("binary"))
        && (content.contains("Ternary") || content.contains("ternary") || content.contains("ABA"));
    let has_sonata_form = content.contains("sonata") || content.contains("Sonata");
    let has_rondo = content.contains("rondo") || content.contains("Rondo");
    let has_theme_variations = content.contains("variation") || content.contains("Variation");

    println!("Form Analysis Knowledge:");
    println!("  ✓ Binary/Ternary forms: {}", if has_binary_ternary { "✅" } else { "❌" });
    println!("  ✓ Sonata form: {}", if has_sonata_form { "✅" } else { "❌" });
    println!("  ✓ Rondo form: {}", if has_rondo { "✅" } else { "❌" });
    println!("  ✓ Theme and variations: {}", if has_theme_variations { "✅" } else { "❌" });

    let score = [
        has_binary_ternary,
        has_sonata_form,
        has_rondo,
        has_theme_variations,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/4", score);

    assert!(score >= 3, "Should cover major musical forms");
}

#[test]
fn test_music_theory_composer_notation_capability() {
    println!("\n=== V1 BENCHMARK: Music Theory Composer - Notation Capability ===\n");

    let mask = load_mask_from_file("music-theory-composer", "sonnet")
        .expect("Failed to load music-theory-composer mask");

    let content = &mask.content;

    // Notation knowledge
    let has_staff_notation = content.contains("staff notation") || content.contains("Staff notation");
    let has_clefs = content.contains("clef") || content.contains("Clef");
    let has_key_signature = content.contains("key signature") || content.contains("Key signature");
    let has_time_signature = content.contains("time signature") || content.contains("Time signature");
    let has_notation_format = content.contains("ABC") || content.contains("Lilypond") || content.contains("notation format");

    println!("Notation Capability:");
    println!("  ✓ Staff notation: {}", if has_staff_notation { "✅" } else { "❌" });
    println!("  ✓ Clefs: {}", if has_clefs { "✅" } else { "❌" });
    println!("  ✓ Key signatures: {}", if has_key_signature { "✅" } else { "❌" });
    println!("  ✓ Time signatures: {}", if has_time_signature { "✅" } else { "❌" });
    println!("  ✓ Notation formats (ABC/Lilypond): {}", if has_notation_format { "✅" } else { "❌" });

    let score = [
        has_staff_notation,
        has_clefs,
        has_key_signature,
        has_time_signature,
        has_notation_format,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert!(score >= 4, "Should have comprehensive notation knowledge");
}

#[test]
fn test_music_theory_composer_example_quality() {
    println!("\n=== V1 BENCHMARK: Music Theory Composer - Example Quality ===\n");

    let mask = load_mask_from_file("music-theory-composer", "sonnet")
        .expect("Failed to load music-theory-composer mask");

    let content = &mask.content;

    // Check for example quality
    let example_count = content.matches("### Example").count();
    let has_bach_example = content.contains("Bach") || content.contains("chorale");
    let has_analysis_example = content.contains("Analysis:") || content.contains("**Analysis:**");
    let has_composition_example = content.contains("Generate") || content.contains("Compose") || content.contains("**Generated");
    let has_jazz_example = content.contains("Jazz") && content.contains("reharmoni");
    let has_notation_in_examples = content.matches("```").count() >= 4;

    println!("Example Quality:");
    println!("  ✓ Number of examples: {}", example_count);
    println!("  ✓ Bach/chorale example: {}", if has_bach_example { "✅" } else { "❌" });
    println!("  ✓ Analysis example: {}", if has_analysis_example { "✅" } else { "❌" });
    println!("  ✓ Composition example: {}", if has_composition_example { "✅" } else { "❌" });
    println!("  ✓ Jazz example: {}", if has_jazz_example { "✅" } else { "❌" });
    println!("  ✓ Notation in examples: {}", if has_notation_in_examples { "✅" } else { "❌" });

    let score = [
        has_bach_example,
        has_analysis_example,
        has_composition_example,
        has_jazz_example,
        has_notation_in_examples,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5 quality dimensions + {} examples", score, example_count);

    assert!(example_count >= 2, "Should have at least 2 detailed examples");
    assert!(score >= 4, "Examples should be high quality with analysis and composition");
}

#[test]
fn test_music_theory_composer_validation_strategy() {
    println!("\n=== V1 BENCHMARK: Music Theory Composer - Validation Strategy ===\n");

    let mask = load_mask_from_file("music-theory-composer", "sonnet")
        .expect("Failed to load music-theory-composer mask");

    let content = &mask.content;

    // Validation capabilities
    let has_validation_section = content.contains("## Validation Strategy") || content.contains("Validation");
    let has_voice_leading_check = content.contains("Voice Leading Check") || content.contains("voice leading") && content.contains("check");
    let has_harmonic_check = content.contains("Harmonic") && content.contains("Check");
    let has_style_check = content.contains("Style") && content.contains("Check");
    let has_performance_check = content.contains("Performance") || content.contains("Practicality") || content.contains("playable");

    println!("Validation Strategy:");
    println!("  ✓ Validation section: {}", if has_validation_section { "✅" } else { "❌" });
    println!("  ✓ Voice leading validation: {}", if has_voice_leading_check { "✅" } else { "❌" });
    println!("  ✓ Harmonic validation: {}", if has_harmonic_check { "✅" } else { "❌" });
    println!("  ✓ Style consistency check: {}", if has_style_check { "✅" } else { "❌" });
    println!("  ✓ Performance practicality: {}", if has_performance_check { "✅" } else { "❌" });

    let score = [
        has_validation_section,
        has_voice_leading_check,
        has_harmonic_check,
        has_style_check,
        has_performance_check,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert!(score >= 4, "Should have comprehensive validation strategy");
}
