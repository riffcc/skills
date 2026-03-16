---
name: storage-systems
description: Expert in distributed storage systems, file systems, object storage, and data persistence trade-offs. Use when designing storage architecture, choosing backends, planning redundancy, or debugging storage performance.
---

# storage-systems

Use this skill when the work is about storage architecture rather than just one disk, one mount, or one benchmark.

## Core Abilities

- compare distributed file systems, object stores, and block storage
- match storage systems to workload shape
- reason about replication, erasure coding, and durability trade-offs
- design migration and cutover plans
- identify performance bottlenecks across disk, network, and caching layers
- turn vague storage goals into measurable requirements

## Working Pattern

1. Identify the workload:
   - sequential vs random
   - read-heavy vs write-heavy
   - large files vs many small files
2. Quantify requirements:
   - capacity
   - throughput
   - latency
   - durability
   - recovery goals
3. Choose the storage model:
   - distributed file system
   - object storage
   - block storage
4. Design redundancy, networking, and operations together.
5. Validate with benchmarks and failure thinking, not assumptions.

## Bias

- match storage to access patterns
- make network limits explicit
- prefer measurable claims over generic “fast” or “reliable”
- plan for failure from the start
- benchmark real workloads before calling the design done
