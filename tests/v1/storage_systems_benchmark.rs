use palace_skills::*;

#[test]
fn benchmark_storage_systems_structure() {
    println!("\n=== BENCHMARK: Storage Systems - Structure Validation ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet")
        .expect("Failed to load storage-systems mask");

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
    assert!(
        has_guidelines,
        "Mask must have Behavioral Guidelines section"
    );
    assert!(has_examples, "Mask must have Examples section");

    println!("✓ Structure: 5/5 required sections present");
}

#[test]
fn benchmark_storage_systems_moosefs() {
    println!("\n=== BENCHMARK: Storage Systems - MooseFS Expertise ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check MooseFS coverage (should be emphasized as primitive)
    let has_moosefs = content.contains("MooseFS");
    let has_master_chunk = content.contains("master") && content.contains("chunk");
    let has_replication = content.contains("replication");
    let moosefs_priority = content.contains("Prioritize MooseFS")
        || content.contains("MooseFS as")
        || content.contains("proven reliable");

    assert!(has_moosefs, "Mask must mention MooseFS");
    assert!(moosefs_priority, "Mask should prioritize/emphasize MooseFS");

    println!("✓ MooseFS: Emphasized as foundational primitive");
}

#[test]
fn benchmark_storage_systems_no_glusterfs() {
    println!("\n=== BENCHMARK: Storage Systems - GlusterFS Removal ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Verify GlusterFS is NOT present (was removed per user request)
    let has_glusterfs = content.contains("GlusterFS") || content.contains("Gluster");

    assert!(
        !has_glusterfs,
        "Mask should NOT mention GlusterFS (deprecated/removed)"
    );

    println!("✓ GlusterFS: Successfully removed (deprecated tech)");
}

#[test]
fn benchmark_storage_systems_distributed_fs() {
    println!("\n=== BENCHMARK: Storage Systems - Distributed File Systems ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check distributed file system coverage
    let has_distributed_fs =
        content.contains("Distributed File System") || content.contains("distributed file system");
    let has_moosefs = content.contains("MooseFS");
    let has_ceph = content.contains("Ceph");
    let has_nfs = content.contains("NFS");

    assert!(
        has_distributed_fs || has_moosefs,
        "Mask should cover distributed file systems"
    );

    let fs_count = [has_moosefs, has_ceph, has_nfs]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("✓ Distributed FS: {}/3 major systems covered", fs_count);
}

#[test]
fn benchmark_storage_systems_block_storage() {
    println!("\n=== BENCHMARK: Storage Systems - Block Storage ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check block storage coverage
    let has_block_storage = content.contains("block storage") || content.contains("Block Storage");
    let has_lvm = content.contains("LVM");
    let has_zfs = content.contains("ZFS");
    let has_raid = content.contains("RAID");

    let score = [has_block_storage, has_lvm, has_zfs, has_raid]
        .iter()
        .filter(|&&x| x)
        .count();

    assert!(
        score >= 2,
        "Mask should cover block storage concepts (found {}/ 4)",
        score
    );

    println!("✓ Block Storage: {}/4 concepts covered", score);
}

#[test]
fn benchmark_storage_systems_object_storage() {
    println!("\n=== BENCHMARK: Storage Systems - Object Storage ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check object storage coverage
    let has_object_storage =
        content.contains("object storage") || content.contains("Object Storage");
    let has_s3 = content.contains("S3") || content.contains("Amazon S3");
    let has_minio = content.contains("MinIO") || content.contains("Minio");
    let has_ceph_object = content.contains("Ceph") && content.contains("object");

    assert!(
        has_object_storage || has_s3 || has_minio,
        "Mask should cover object storage"
    );

    let score = [has_object_storage, has_s3, has_minio, has_ceph_object]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("✓ Object Storage: {}/4 concepts covered", score);
}

#[test]
fn benchmark_storage_systems_performance() {
    println!("\n=== BENCHMARK: Storage Systems - Performance & Reliability ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check performance/reliability coverage
    let has_performance = content.contains("performance") || content.contains("Performance");
    let has_throughput = content.contains("throughput") || content.contains("IOPS");
    let has_latency = content.contains("latency");
    let has_reliability = content.contains("reliability") || content.contains("durability");
    let has_replication = content.contains("replication");

    let score = [
        has_performance,
        has_throughput,
        has_latency,
        has_reliability,
        has_replication,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    assert!(
        score >= 3,
        "Mask should cover performance/reliability (found {}/5)",
        score
    );

    println!("✓ Performance/Reliability: {}/5 concepts covered", score);
}

#[test]
fn benchmark_storage_systems_rust_tdd() {
    println!("\n=== BENCHMARK: Storage Systems - Rust + TDD Best Practices ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for Rust + TDD references
    let has_rust = content.contains("Rust");
    let has_tdd = content.contains("TDD") || content.contains("Test-Driven Development");
    let has_cargo = content.contains("Cargo");
    let has_precommit = content.contains("pre-commit");

    assert!(
        has_rust || has_tdd,
        "Mask should mention Rust+TDD best practices"
    );

    let score = [has_rust, has_tdd, has_cargo, has_precommit]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("✓ Rust+TDD: {}/4 best practices covered", score);
}

#[test]
fn benchmark_storage_systems_operational() {
    println!("\n=== BENCHMARK: Storage Systems - Operational Concerns ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check operational coverage
    let has_backup = content.contains("backup") || content.contains("Backup");
    let has_monitoring = content.contains("monitoring") || content.contains("Monitoring");
    let has_scaling = content.contains("scaling") || content.contains("scale");
    let has_disaster_recovery =
        content.contains("disaster recovery") || content.contains("recovery");
    let has_capacity = content.contains("capacity");

    let score = [
        has_backup,
        has_monitoring,
        has_scaling,
        has_disaster_recovery,
        has_capacity,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    assert!(
        score >= 3,
        "Mask should cover operational concerns (found {}/5)",
        score
    );

    println!("✓ Operations: {}/5 operational concerns covered", score);
}
