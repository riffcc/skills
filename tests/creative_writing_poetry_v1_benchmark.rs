// Benchmark tests for creative-writing-poetry v1
// Validates structure, domain coverage, and technical knowledge

use palace_skills::*;

#[test]
fn test_poetry_specialist_structure() {
    println!("\n=== BENCHMARK: Poetry Specialist - Structure Validation ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Standard 6-section structure
    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_guidelines = content.contains("## Behavioral Guidelines");
    let has_examples = content.contains("## Examples");
    let has_improvements = content.contains("## Improvement Notes");

    println!("Structure Validation:");
    println!("  ✓ Identity section: {}", if has_identity { "✅" } else { "❌" });
    println!("  ✓ Core Expertise section: {}", if has_expertise { "✅" } else { "❌" });
    println!("  ✓ Your Mission section: {}", if has_mission { "✅" } else { "❌" });
    println!("  ✓ Behavioral Guidelines section: {}", if has_guidelines { "✅" } else { "❌" });
    println!("  ✓ Examples section: {}", if has_examples { "✅" } else { "❌" });
    println!("  ✓ Improvement Notes section: {}", if has_improvements { "✅" } else { "❌" });

    let score = [
        has_identity,
        has_expertise,
        has_mission,
        has_guidelines,
        has_examples,
        has_improvements,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nStructure Score: {}/6", score);

    assert_eq!(score, 6, "Should have all 6 required sections");
}

#[test]
fn test_poetry_specialist_poetic_forms() {
    println!("\n=== BENCHMARK: Poetry Specialist - Poetic Forms Knowledge ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Test for knowledge of key poetic forms
    let has_sonnet = content.contains("Sonnet") || content.contains("sonnet");
    let has_haiku = content.contains("Haiku") || content.contains("haiku");
    let has_villanelle = content.contains("villanelle") || content.contains("Villanelle");
    let has_free_verse = content.contains("free verse") || content.contains("Free verse");
    let has_blank_verse = content.contains("blank verse") || content.contains("Blank verse");

    println!("Poetic Forms Coverage:");
    println!("  ✓ Sonnet: {}", if has_sonnet { "✅" } else { "❌" });
    println!("  ✓ Haiku: {}", if has_haiku { "✅" } else { "❌" });
    println!("  ✓ Villanelle: {}", if has_villanelle { "✅" } else { "❌" });
    println!("  ✓ Free verse: {}", if has_free_verse { "✅" } else { "❌" });
    println!("  ✓ Blank verse: {}", if has_blank_verse { "✅" } else { "❌" });

    let score = [
        has_sonnet,
        has_haiku,
        has_villanelle,
        has_free_verse,
        has_blank_verse,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nForms Score: {}/5", score);

    assert!(score >= 4, "Should mention at least 4/5 major poetic forms");
}

#[test]
fn test_poetry_specialist_prosody() {
    println!("\n=== BENCHMARK: Poetry Specialist - Prosody & Meter Knowledge ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Test for prosody concepts
    let has_meter = content.contains("meter") || content.contains("Meter");
    let has_iambic = content.contains("iambic") || content.contains("Iambic");
    let has_pentameter = content.contains("pentameter") || content.contains("Pentameter");
    let has_rhythm = content.contains("rhythm") || content.contains("Rhythm");
    let has_scansion = content.contains("scansion") || content.contains("Scansion");

    println!("Prosody & Meter:");
    println!("  ✓ Meter concept: {}", if has_meter { "✅" } else { "❌" });
    println!("  ✓ Iambic: {}", if has_iambic { "✅" } else { "❌" });
    println!("  ✓ Pentameter: {}", if has_pentameter { "✅" } else { "❌" });
    println!("  ✓ Rhythm: {}", if has_rhythm { "✅" } else { "❌" });
    println!("  ✓ Scansion: {}", if has_scansion { "✅" } else { "❌" });

    let score = [
        has_meter,
        has_iambic,
        has_pentameter,
        has_rhythm,
        has_scansion,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nProsody Score: {}/5", score);

    assert!(score >= 4, "Should have solid prosody knowledge (at least 4/5)");
}

#[test]
fn test_poetry_specialist_sound_devices() {
    println!("\n=== BENCHMARK: Poetry Specialist - Sound Devices ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Test for sound device knowledge
    let has_rhyme = content.contains("rhyme") || content.contains("Rhyme");
    let has_alliteration = content.contains("alliteration") || content.contains("Alliteration");
    let has_assonance = content.contains("assonance") || content.contains("Assonance");
    let has_consonance = content.contains("consonance") || content.contains("Consonance");

    println!("Sound Devices:");
    println!("  ✓ Rhyme: {}", if has_rhyme { "✅" } else { "❌" });
    println!("  ✓ Alliteration: {}", if has_alliteration { "✅" } else { "❌" });
    println!("  ✓ Assonance: {}", if has_assonance { "✅" } else { "❌" });
    println!("  ✓ Consonance: {}", if has_consonance { "✅" } else { "❌" });

    let score = [
        has_rhyme,
        has_alliteration,
        has_assonance,
        has_consonance,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nSound Devices Score: {}/4", score);

    assert!(score >= 3, "Should mention at least 3/4 major sound devices");
}

#[test]
fn test_poetry_specialist_imagery_figurative_language() {
    println!("\n=== BENCHMARK: Poetry Specialist - Imagery & Figurative Language ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Test for imagery and figurative language concepts
    let has_metaphor = content.contains("metaphor") || content.contains("Metaphor");
    let has_simile = content.contains("simile") || content.contains("Simile");
    let has_imagery = content.contains("imagery") || content.contains("Imagery");
    let has_symbol = content.contains("symbol") || content.contains("Symbol");
    let has_personification = content.contains("personification") || content.contains("Personification");

    println!("Imagery & Figurative Language:");
    println!("  ✓ Metaphor: {}", if has_metaphor { "✅" } else { "❌" });
    println!("  ✓ Simile: {}", if has_simile { "✅" } else { "❌" });
    println!("  ✓ Imagery: {}", if has_imagery { "✅" } else { "❌" });
    println!("  ✓ Symbolism: {}", if has_symbol { "✅" } else { "❌" });
    println!("  ✓ Personification: {}", if has_personification { "✅" } else { "❌" });

    let score = [
        has_metaphor,
        has_simile,
        has_imagery,
        has_symbol,
        has_personification,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nImagery Score: {}/5", score);

    assert!(score >= 4, "Should have strong imagery/figurative language coverage (at least 4/5)");
}

#[test]
fn test_poetry_specialist_voice_emotion() {
    println!("\n=== BENCHMARK: Poetry Specialist - Voice & Emotional Concepts ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Test for voice and emotional resonance concepts
    let has_voice = content.contains("voice") || content.contains("Voice");
    let has_tone = content.contains("tone") || content.contains("Tone");
    let has_emotion = content.contains("emotion") || content.contains("Emotion");
    let has_authenticity = content.contains("authentic") || content.contains("Authentic");
    let has_perspective = content.contains("perspective") || content.contains("Perspective");

    println!("Voice & Emotional Concepts:");
    println!("  ✓ Voice: {}", if has_voice { "✅" } else { "❌" });
    println!("  ✓ Tone: {}", if has_tone { "✅" } else { "❌" });
    println!("  ✓ Emotion: {}", if has_emotion { "✅" } else { "❌" });
    println!("  ✓ Authenticity: {}", if has_authenticity { "✅" } else { "❌" });
    println!("  ✓ Perspective: {}", if has_perspective { "✅" } else { "❌" });

    let score = [
        has_voice,
        has_tone,
        has_emotion,
        has_authenticity,
        has_perspective,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nVoice & Emotion Score: {}/5", score);

    assert!(score >= 4, "Should address voice and emotional concepts (at least 4/5)");
}

#[test]
fn test_poetry_specialist_revision_craft() {
    println!("\n=== BENCHMARK: Poetry Specialist - Revision & Craft ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Test for revision and craft concepts
    let has_revision = content.contains("revision") || content.contains("Revision");
    let has_line_break = content.contains("line break") || content.contains("Line break");
    let has_enjambment = content.contains("enjambment") || content.contains("Enjambment");
    let has_compression = content.contains("compression") || content.contains("Compression")
        || content.contains("tighten");
    let has_cliche = content.contains("cliché") || content.contains("cliche");

    println!("Revision & Craft:");
    println!("  ✓ Revision: {}", if has_revision { "✅" } else { "❌" });
    println!("  ✓ Line breaks: {}", if has_line_break { "✅" } else { "❌" });
    println!("  ✓ Enjambment: {}", if has_enjambment { "✅" } else { "❌" });
    println!("  ✓ Compression: {}", if has_compression { "✅" } else { "❌" });
    println!("  ✓ Avoiding clichés: {}", if has_cliche { "✅" } else { "❌" });

    let score = [
        has_revision,
        has_line_break,
        has_enjambment,
        has_compression,
        has_cliche,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nRevision & Craft Score: {}/5", score);

    assert!(score >= 4, "Should have strong revision/craft guidance (at least 4/5)");
}

#[test]
fn test_poetry_specialist_literary_context() {
    println!("\n=== BENCHMARK: Poetry Specialist - Literary Context & Tradition ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Test for awareness of literary tradition
    let has_classical = content.contains("Classical") || content.contains("classical");
    let has_romantic = content.contains("Romantic") || content.contains("romantic");
    let has_modernist = content.contains("Modernist") || content.contains("modernist");
    let has_contemporary = content.contains("Contemporary") || content.contains("contemporary");
    let has_poet_examples = (content.contains("Keats") || content.contains("Dickinson")
        || content.contains("Frost") || content.contains("Bishop")
        || content.contains("Eliot") || content.contains("Plath"));

    println!("Literary Context:");
    println!("  ✓ Classical tradition: {}", if has_classical { "✅" } else { "❌" });
    println!("  ✓ Romantic period: {}", if has_romantic { "✅" } else { "❌" });
    println!("  ✓ Modernist movement: {}", if has_modernist { "✅" } else { "❌" });
    println!("  ✓ Contemporary poetry: {}", if has_contemporary { "✅" } else { "❌" });
    println!("  ✓ Named poets as examples: {}", if has_poet_examples { "✅" } else { "❌" });

    let score = [
        has_classical,
        has_romantic,
        has_modernist,
        has_contemporary,
        has_poet_examples,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nLiterary Context Score: {}/5", score);

    assert!(score >= 3, "Should show awareness of literary tradition (at least 3/5)");
}

#[test]
fn test_poetry_specialist_example_quality() {
    println!("\n=== BENCHMARK: Poetry Specialist - Example Quality ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Test for quality examples
    let example_count = content.matches("### Example").count();
    let has_concrete_poems = content.contains("```") || content.contains("poem");
    let has_before_after = content.contains("Before:") || content.contains("Original");
    let has_analysis = content.contains("Analysis:") || content.contains("What Improved");
    let has_specific_techniques = content.contains("line break") && content.contains("enjamb");

    println!("Example Quality:");
    println!("  ✓ Number of examples: {} (expect >= 2)", example_count);
    println!("  ✓ Concrete poems/verse: {}", if has_concrete_poems { "✅" } else { "❌" });
    println!("  ✓ Before/after comparisons: {}", if has_before_after { "✅" } else { "❌" });
    println!("  ✓ Detailed analysis: {}", if has_analysis { "✅" } else { "❌" });
    println!("  ✓ Specific techniques shown: {}", if has_specific_techniques { "✅" } else { "❌" });

    let score = [
        example_count >= 2,
        has_concrete_poems,
        has_before_after,
        has_analysis,
        has_specific_techniques,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nExample Quality Score: {}/5", score);

    assert!(score >= 4, "Should have high-quality concrete examples (at least 4/5)");
}

#[test]
fn test_poetry_specialist_scoring_framework() {
    println!("\n=== BENCHMARK: Poetry Specialist - Evaluation/Scoring Framework ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Test for structured scoring approach
    let has_evaluation = content.contains("Evaluate") || content.contains("Score");
    let has_dimensions = content.contains("Technical") && content.contains("Imagery");
    let has_percentages = content.contains("%") || content.contains("weighted");
    let has_subjectivity_note = content.contains("subjective") || content.contains("Subjective");

    println!("Scoring Framework:");
    println!("  ✓ Evaluation section: {}", if has_evaluation { "✅" } else { "❌" });
    println!("  ✓ Multiple dimensions: {}", if has_dimensions { "✅" } else { "❌" });
    println!("  ✓ Weighted scoring: {}", if has_percentages { "✅" } else { "❌" });
    println!("  ✓ Acknowledges subjectivity: {}", if has_subjectivity_note { "✅" } else { "❌" });

    let score = [
        has_evaluation,
        has_dimensions,
        has_percentages,
        has_subjectivity_note,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScoring Framework Score: {}/4", score);

    assert!(score >= 3, "Should have structured but subjective-aware evaluation (at least 3/4)");
}

#[test]
fn test_poetry_specialist_subjective_awareness() {
    println!("\n=== BENCHMARK: Poetry Specialist - Subjective Domain Awareness ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Test that mask acknowledges poetry is subjective/artistic
    let has_subjective = content.contains("subjective") || content.contains("Subjective");
    let has_multiple_interpretations = content.contains("multiple") && content.contains("interpretation");
    let has_no_single_correct = content.contains("no single") || content.contains("not just one");
    let has_context_matters = content.contains("context") || content.contains("Context");
    let respects_voice = content.contains("their voice") || content.contains("authentic voice")
        || content.contains("THEIR voice");

    println!("Subjective Awareness:");
    println!("  ✓ Acknowledges subjectivity: {}", if has_subjective { "✅" } else { "❌" });
    println!("  ✓ Multiple interpretations: {}", if has_multiple_interpretations { "✅" } else { "❌" });
    println!("  ✓ No single 'correct' answer: {}", if has_no_single_correct { "✅" } else { "❌" });
    println!("  ✓ Context matters: {}", if has_context_matters { "✅" } else { "❌" });
    println!("  ✓ Respects individual voice: {}", if respects_voice { "✅" } else { "❌" });

    let score = [
        has_subjective,
        has_multiple_interpretations,
        has_no_single_correct,
        has_context_matters,
        respects_voice,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nSubjective Awareness Score: {}/5", score);

    assert!(score >= 3, "Should acknowledge subjective nature of poetry (at least 3/5)");
}

#[test]
fn test_poetry_specialist_comprehensive_coverage() {
    println!("\n=== COMPREHENSIVE BENCHMARK: Poetry Specialist - Overall Domain Coverage ===\n");

    let mask = load_mask_from_file("creative-writing-poetry", "sonnet")
        .expect("Failed to load creative-writing-poetry mask");

    let content = &mask.content;

    // Aggregate test across all major areas
    let areas = vec![
        ("Forms", content.contains("sonnet") && content.contains("haiku")),
        ("Prosody", content.contains("meter") && content.contains("iambic")),
        ("Sound", content.contains("rhyme") && content.contains("alliteration")),
        ("Imagery", content.contains("metaphor") && content.contains("imagery")),
        ("Voice", content.contains("voice") && content.contains("tone")),
        ("Revision", content.contains("revision") && content.contains("line break")),
        ("Literary Context", content.contains("Romantic") || content.contains("Modernist")),
        ("Examples", content.matches("### Example").count() >= 2),
        ("Scoring", content.contains("Score") || content.contains("Evaluate")),
        ("Subjectivity", content.contains("subjective")),
    ];

    println!("Comprehensive Domain Coverage:");
    for (area, present) in &areas {
        println!("  ✓ {}: {}", area, if *present { "✅" } else { "❌" });
    }

    let total_score = areas.iter().filter(|(_, present)| *present).count();
    println!("\nOverall Coverage Score: {}/10", total_score);

    assert!(total_score >= 8, "Should have comprehensive poetry domain coverage (at least 8/10)");
}
