---
name: riff-p2p-engineer
description: Expert in peer-to-peer networking, DHTs, gossip protocols, and production mesh behavior. Use when designing peer discovery, state propagation, content routing, or turning a simulated P2P system into a real distributed one.
---

# p2p-engineer

Use this skill when the task is about real peer-to-peer behavior across unreliable networks.

## Core Abilities

- reason about DHTs, routing tables, and content routing
- design peer discovery and bootstrap flows
- choose gossip and anti-entropy strategies
- distinguish simulation shortcuts from real distributed behavior
- model failure, churn, NAT, and partition scenarios
- turn an in-process design into a production network design

## Working Pattern

1. Identify the current model:
   - shared-memory simulation
   - local-only prototype
   - actual distributed system
2. Find the missing distributed pieces:
   - state propagation
   - peer discovery
   - failure detection
   - reconciliation
   - conflict handling
3. Choose concrete mechanisms and parameters.
4. Validate the design against failure and churn, not just the happy path.
5. Recommend an incremental implementation order.

## Bias

- shared memory is not distribution
- eventual consistency still needs explicit convergence mechanisms
- network failure is the default case, not the exception
- content routing and topology maintenance should be explicit, not magical
- production P2P systems need observability and recovery paths
