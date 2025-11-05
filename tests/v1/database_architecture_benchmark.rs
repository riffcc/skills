use palace_skills::*;

#[test]
fn benchmark_database_architecture_structure() {
    println!("\n=== BENCHMARK: Database Architecture - Structure Validation ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet")
        .expect("Failed to load database-architecture mask");

    let content = &mask.content;

    // Core sections
    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_guidelines = content.contains("## Behavioral Guidelines");
    let has_examples = content.contains("## Examples");

    assert!(has_identity, "Mask must have Identity section");
    assert!(has_expertise, "Mask must have Core Expertise section");
    assert!(has_mission, "Mask must have Your Mission section");
    assert!(has_guidelines, "Mask must have Behavioral Guidelines section");
    assert!(has_examples, "Mask must have Examples section");

    println!("✓ Structure: 5/5 required sections present");
}

#[test]
fn benchmark_database_architecture_acid() {
    println!("\n=== BENCHMARK: Database Architecture - ACID Properties ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check ACID coverage
    let has_acid = content.contains("ACID");
    let has_atomicity = content.contains("tomicity") || content.contains("Atomicity");
    let has_consistency = content.contains("onsistency") || content.contains("Consistency");
    let has_isolation = content.contains("solation") || content.contains("Isolation");
    let has_durability = content.contains("urability") || content.contains("Durability");

    assert!(has_acid || (has_atomicity && has_consistency && has_isolation && has_durability),
            "Mask should cover ACID properties");

    println!("✓ ACID: Core transaction properties covered");
}

#[test]
fn benchmark_database_architecture_indexing() {
    println!("\n=== BENCHMARK: Database Architecture - Indexing Strategies ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check indexing coverage
    let has_btree = content.contains("B-tree") || content.contains("BTree") || content.contains("B+tree");
    let has_hash = content.contains("hash index") || content.contains("Hash index");
    let has_indexing = content.contains("index") || content.contains("Index");
    let has_query_performance = content.contains("query performance") || content.contains("query optimization");

    assert!(has_indexing, "Mask should discuss indexing");
    assert!(has_btree || has_hash || content.contains("GiST") || content.contains("GIN"),
            "Mask should mention specific index types");

    println!("✓ Indexing: Index strategies covered");
}

#[test]
fn benchmark_database_architecture_replication() {
    println!("\n=== BENCHMARK: Database Architecture - Replication Patterns ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check replication coverage
    let has_replication = content.contains("replication") || content.contains("Replication");
    let has_primary_replica = content.contains("primary-replica") ||
                              content.contains("primary replica") ||
                              content.contains("master-slave");
    let has_streaming = content.contains("streaming replication") ||
                        content.contains("streaming");
    let has_sync_async = content.contains("synchronous") || content.contains("asynchronous");

    assert!(has_replication, "Mask should discuss replication");
    assert!(has_primary_replica || has_sync_async, "Mask should discuss replication patterns");

    println!("✓ Replication: Replication patterns covered");
}

#[test]
fn benchmark_database_architecture_postgres() {
    println!("\n=== BENCHMARK: Database Architecture - PostgreSQL Expertise ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check PostgreSQL-specific coverage
    let has_postgres = content.contains("PostgreSQL") || content.contains("Postgres");
    let has_mvcc = content.contains("MVCC") || content.contains("Multi-Version Concurrency");
    let has_vacuum = content.contains("VACUUM") || content.contains("vacuum");
    let has_pg_specific = has_postgres && (has_mvcc || has_vacuum ||
                                           content.contains("pg_stat") ||
                                           content.contains("pgbench"));

    assert!(has_postgres, "Mask should mention PostgreSQL");

    if has_pg_specific {
        println!("✓ PostgreSQL: PostgreSQL-specific features covered");
    } else {
        println!("⚠ PostgreSQL: Mentioned but lacks PostgreSQL-specific details");
    }
}

#[test]
fn benchmark_database_architecture_normalization() {
    println!("\n=== BENCHMARK: Database Architecture - Data Modeling ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check data modeling coverage
    let has_normalization = content.contains("normalization") || content.contains("Normalization");
    let has_schema_design = content.contains("schema") || content.contains("Schema");
    let has_modeling = content.contains("data model") || content.contains("Data model");
    let has_relations = content.contains("relationship") || content.contains("foreign key");

    assert!(has_schema_design || has_modeling, "Mask should discuss data modeling");

    let score = [has_normalization, has_schema_design, has_modeling, has_relations]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("✓ Data Modeling: {}/4 concepts covered", score);
}

#[test]
fn benchmark_database_architecture_performance() {
    println!("\n=== BENCHMARK: Database Architecture - Performance Tuning ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check performance tuning coverage
    let has_performance = content.contains("performance") || content.contains("Performance");
    let has_query_optimization = content.contains("query optimization") ||
                                 content.contains("EXPLAIN");
    let has_caching = content.contains("cach") || content.contains("Cach");
    let has_connection_pooling = content.contains("connection pool");
    let has_monitoring = content.contains("monitoring") || content.contains("metrics");

    assert!(has_performance, "Mask should discuss performance");

    let score = [has_query_optimization, has_caching, has_connection_pooling, has_monitoring]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("✓ Performance: {}/4 tuning aspects covered", score);
}

#[test]
fn benchmark_database_architecture_backup_recovery() {
    println!("\n=== BENCHMARK: Database Architecture - Backup & Recovery ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check backup/recovery coverage
    let has_backup = content.contains("backup") || content.contains("Backup");
    let has_recovery = content.contains("recovery") || content.contains("Recovery");
    let has_wal = content.contains("WAL") || content.contains("Write-Ahead Log");
    let has_pitr = content.contains("PITR") || content.contains("point-in-time");
    let has_disaster_recovery = content.contains("disaster recovery") || content.contains("DR");

    assert!(has_backup || has_recovery, "Mask should discuss backup/recovery");

    let score = [has_backup, has_recovery, has_wal, has_pitr, has_disaster_recovery]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("✓ Backup/Recovery: {}/5 concepts covered", score);
}
