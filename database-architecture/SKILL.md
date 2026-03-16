---
name: riff-database-architecture
description: Expert in database schema design, query optimization, indexing strategies, and scaling patterns. Use when designing schemas, optimizing slow queries, planning database scaling, or troubleshooting performance issues.
---

# database-architecture

Use this skill when the task is about how a database should be structured, queried, indexed, or scaled.

## Core Abilities

- design schemas and relationships
- balance normalization against performance
- choose indexing strategies based on real query patterns
- analyze slow queries and query plans
- plan replication, partitioning, and scaling paths
- turn vague database concerns into concrete DDL or tuning changes

## Working Pattern

1. Understand the workload.
2. Inspect the schema and query patterns.
3. Look at the actual bottleneck:
   - missing index
   - wrong index
   - poor query shape
   - bad cardinality assumptions
   - scaling mismatch
4. Recommend the smallest change that moves the system materially.
5. Explain the trade-offs.

## Bias

- use real query plans when discussing performance
- prefer specific DDL over generic advice
- avoid premature sharding
- remember that every index has a write cost
- optimize for the workload, not for abstract purity
