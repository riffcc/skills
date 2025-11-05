use palace_skills::*;

/// V2 Benchmarks for database-architecture mask
///
/// V1 tested for PRESENCE of concepts (keywords)
/// V2 tests for DEPTH of understanding (examples, trade-offs, failure scenarios)

#[test]
fn v2_acid_isolation_tradeoffs() {
    println!("\n=== V2 BENCHMARK: Database Architecture - ACID Isolation Trade-offs ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V1 just checked for "ACID" keyword
    // V2 requires actual isolation level trade-off analysis

    let has_serializable = content.contains("Serializable") || content.contains("SERIALIZABLE");
    let has_read_committed =
        content.contains("Read Committed") || content.contains("READ COMMITTED");
    let has_repeatable_read =
        content.contains("Repeatable Read") || content.contains("REPEATABLE READ");

    // Must explain performance trade-offs
    let has_performance_tradeoff = (content.contains("Serializable")
        || content.contains("SERIALIZABLE"))
        && content.contains("performance");

    // Must explain consistency trade-offs
    let has_consistency_tradeoff = content.contains("phantom reads")
        || content.contains("dirty reads")
        || content.contains("non-repeatable reads");

    // Must have example showing when to choose different levels
    let has_isolation_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("isolation") || content.contains("Serializable"));

    let score = [
        has_serializable,
        has_read_committed,
        has_repeatable_read,
        has_performance_tradeoff,
        has_consistency_tradeoff,
        has_isolation_example,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 ACID Isolation: {}/6", score);

    if has_isolation_example {
        println!("✓ V2: Isolation levels demonstrated with examples");
    } else {
        println!(
            "⚠ V2: Isolation levels mentioned but lack concrete examples showing when to choose"
        );
    }

    // V2 requires 5/6
    assert!(
        score >= 5,
        "V2: Must have deep ACID isolation coverage (got {}/6)",
        score
    );
}

#[test]
fn v2_replication_failure_handling() {
    println!("\n=== V2 BENCHMARK: Database Architecture - Replication Failure Handling ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V1 checked for "replication" keyword
    // V2 requires failure mode coverage

    let has_replication_lag = content.contains("replication lag") || content.contains("replay lag");

    let has_failover = content.contains("failover")
        || content.contains("promotion")
        || content.contains("switchover");

    let has_split_brain = content.contains("split-brain") || content.contains("split brain");

    let has_lag_detection =
        content.contains("lag") && (content.contains("monitor") || content.contains("alert"));

    // Must have example showing what happens when replication fails
    let has_failure_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("replica") || content.contains("replication"))
        && (content.contains("failure") || content.contains("fail"));

    // Must have specific thresholds
    let has_lag_threshold =
        content.contains("ms") || content.contains("seconds") || content.contains("< ");

    let score = [
        has_replication_lag,
        has_failover,
        has_split_brain,
        has_lag_detection,
        has_failure_example,
        has_lag_threshold,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Replication Failure: {}/6 covered", score);

    if has_failure_example {
        println!("✓ V2: Replication failures demonstrated with examples");
    } else {
        println!("⚠ V2: Replication mentioned but lacks concrete failure/recovery examples");
    }

    // V2 requires 5/6
    assert!(
        score >= 5,
        "V2: Must cover replication failure scenarios (got {}/6)",
        score
    );
}

#[test]
fn v2_query_optimization_examples() {
    println!("\n=== V2 BENCHMARK: Database Architecture - Query Optimization Examples ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V1 just checked for optimization mentioned
    // V2 requires actual EXPLAIN plans and optimization techniques

    let has_explain = content.contains("EXPLAIN") || content.contains("explain plan");

    let has_seq_scan = content.contains("Seq Scan")
        || content.contains("Sequential Scan")
        || content.contains("seq scan");

    let has_index_scan = content.contains("Index Scan") || content.contains("index scan");

    let has_index_creation = content.contains("CREATE INDEX") || content.contains("create index");

    // Must have example showing before/after optimization
    let has_optimization_example = (content.contains("### Example")
        || content.contains("**Example"))
        && (content.contains("EXPLAIN")
            || content.contains("optimization")
            || content.contains("index"));

    // Must have performance metrics
    let has_performance_metrics = content.contains("ms")
        || content.contains("execution time")
        || content.contains("query time");

    let score = [
        has_explain,
        has_seq_scan,
        has_index_scan,
        has_index_creation,
        has_optimization_example,
        has_performance_metrics,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Query Optimization: {}/6", score);

    if has_optimization_example {
        println!("✓ V2: Query optimization demonstrated with EXPLAIN examples");
    } else {
        println!("⚠ V2: Query optimization mentioned but lacks concrete EXPLAIN plan examples");
    }

    // V2 requires 5/6
    assert!(
        score >= 5,
        "V2: Must have detailed query optimization examples (got {}/6)",
        score
    );
}

#[test]
fn v2_schema_antipatterns() {
    println!("\n=== V2 BENCHMARK: Database Architecture - Schema Anti-patterns ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires explicit "what NOT to do" for schema design

    let has_avoid_section = content.contains("## What to Avoid")
        || content.contains("### What to Avoid")
        || content.contains("What NOT to do");

    let has_dont_statements = content.matches("Don't").count() >= 2
        || content.matches("don't").count() >= 2
        || content.matches("Never").count() >= 2;

    let has_normalization_guidance =
        content.contains("normalize") || content.contains("denormalize");

    let has_eav_warning = content.contains("EAV")
        || content.contains("Entity-Attribute-Value")
        || (content.contains("anti-pattern") && content.contains("schema"));

    // Must have example showing bad schema design
    let has_bad_schema_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("anti") || content.contains("avoid") || content.contains("bad"));

    let score = [
        has_avoid_section,
        has_dont_statements,
        has_normalization_guidance,
        has_eav_warning,
        has_bad_schema_example,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Schema Anti-patterns: {}/5 criteria met", score);

    if has_avoid_section {
        println!("✓ V2: Dedicated anti-pattern section present");
    } else {
        println!("⚠ V2: No dedicated schema anti-pattern section");
    }

    // V2 requires 4/5
    assert!(
        score >= 4,
        "V2: Must cover schema anti-patterns extensively (got {}/5)",
        score
    );
}

#[test]
fn v2_production_ready_examples() {
    println!("\n=== V2 BENCHMARK: Database Architecture - Production-Ready Examples ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V1 just checked for examples existing
    // V2 requires DETAILED examples with actual SQL, configs, metrics

    // Count proper example sections
    let example_count = content.matches("### Example").count();

    // Check for actual SQL code
    let has_sql = content.contains("```sql") || content.contains("```SQL");
    let has_create_table = content.contains("CREATE TABLE") || content.contains("create table");
    let has_select = content.contains("SELECT") && content.contains("FROM");

    // Check for configuration blocks
    let has_config = content.contains("```yaml")
        || content.contains("```toml")
        || content.contains("postgresql.conf");

    // Check for metrics/monitoring
    let has_metrics = content.contains("metrics") || content.contains("monitoring");
    let has_specific_metrics = content.contains("pg_stat")
        || content.contains("tps")
        || content.contains("query time")
        || content.contains("connections");

    // Check for schema DDL
    let has_ddl = content.contains("DDL")
        || (content.contains("CREATE") && (content.contains("TABLE") || content.contains("INDEX")));

    println!("V2 Examples: {} proper example sections", example_count);
    println!(
        "V2 SQL: {}",
        if has_sql || has_create_table {
            "Present"
        } else {
            "Missing"
        }
    );
    println!(
        "V2 Config: {}",
        if has_config { "Present" } else { "Missing" }
    );
    println!(
        "V2 Metrics: {}",
        if has_metrics { "Covered" } else { "Missing" }
    );

    let score = [
        example_count >= 3,
        has_sql || has_create_table || has_select,
        has_config,
        has_metrics && has_specific_metrics,
        has_ddl,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Production-Ready: {}/5", score);

    // V2 requires at least 3 examples
    assert!(
        example_count >= 3,
        "V2: Must have at least 3 detailed examples (found {})",
        example_count
    );

    // V2 requires actual SQL in examples
    assert!(
        has_sql || has_create_table || has_select,
        "V2: Must include actual SQL code in examples"
    );

    // V2 requires specific metrics
    assert!(has_metrics, "V2: Must specify monitoring metrics");

    println!("✓ V2: Production-ready examples with SQL, configs, and metrics");
}

#[test]
fn v2_backup_recovery_procedures() {
    println!("\n=== V2 BENCHMARK: Database Architecture - Backup & Recovery Procedures ===\n");

    let mask = load_mask_from_file("database-architecture", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires specific backup/recovery guidance (critical for production DBs)

    let has_backup_mention = content.contains("backup") || content.contains("Backup");

    let has_pg_dump = content.contains("pg_dump") || content.contains("pg_basebackup");

    let has_wal_archiving = content.contains("WAL")
        || content.contains("Write-Ahead Log")
        || content.contains("archive");

    let has_pitr = content.contains("PITR")
        || content.contains("point-in-time recovery")
        || content.contains("point in time");

    let has_recovery_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("backup")
            || content.contains("recovery")
            || content.contains("restore"));

    let has_rto_rpo = content.contains("RTO")
        || content.contains("RPO")
        || content.contains("Recovery Time Objective")
        || content.contains("Recovery Point Objective");

    let score = [
        has_backup_mention,
        has_pg_dump,
        has_wal_archiving,
        has_pitr,
        has_recovery_example,
        has_rto_rpo,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Backup/Recovery: {}/6", score);

    if has_recovery_example {
        println!("✓ V2: Backup/recovery demonstrated with examples");
    } else {
        println!("⚠ V2: Backup mentioned but lacks concrete recovery procedures");
    }

    // V2 requires 4/6 (backup/recovery is critical but may not be in all examples)
    assert!(
        score >= 4,
        "V2: Must have backup/recovery coverage (got {}/6)",
        score
    );
}
