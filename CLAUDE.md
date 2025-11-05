# Guidelines for Claude Code

## Project Overview
RHSI (Recursive Hierarchical Self Improvement) is a system for adaptive AI specialization that enables AI agents to recursively improve their capabilities through hierarchical learning and specialization. The system allows AI to identify areas for improvement, develop specialized skills, and integrate those improvements back into the core system in a self-directed manner.

## Rules
- **Consult README.md** for context whenever needed
- **Keep README.md Updated** - When adding new commands, features, or changing functionality, ALWAYS update the README.md Usage section. Palace uses the README to understand what exists, so outdated docs lead to duplicate suggestions!
- **Test Driven Development** - Write tests before implementing ANY code or feature, no matter how small. We aim for high code coverage from the beginning.
- **Zero Placeholders** - Do not put in references to commands or functionality that are not implemented yet or do not exist
- **Modularity** - Break down components into small, focused files (typically <200 LoC per file)
- **Test Modularity** - Tests should be modular and organized for easy understanding and maintenance
- **"DO NOT SIMPLIFY - EVER"** - When thinking of simplifying something, think through the change deeply and ask the user what they want to do
- **Commit Regularly** - Test after every change and commit very regularly with tiny atomic chunks
- **Follow Language Style Guides** - Adhere to the style guide of your primary language
- **Use Palace Tools** - Use `pal test`, `pal build`, `pal run` for development workflows

## Quality Standards
- Write comprehensive tests for all new features
- Keep functions small and focused
- Use meaningful variable and function names
- Document complex logic with clear comments
- Handle errors gracefully with proper error messages

## Development Workflow
1. **Understand Requirements** - Read README.md and existing code
2. **Write Tests First** - Create failing tests that define expected behavior
3. **Implement Features** - Write minimal code to make tests pass
4. **Refactor** - Clean up code while keeping tests green
5. **Commit** - Small, atomic commits with clear messages

## Palace Integration
This project uses Palace (`pal`) for development:
- `pal test` - Run tests
- `pal build` - Build the project
- `pal run` - Run the project
- `pal next` - Get AI suggestions for next tasks
- `pal commit` - Create well-formatted commits
- `pal switch` - Switch between development machines

## External Codebases to Reference

**CRITICAL: Always reference the Palace CLI codebase before implementing features**

Palace CLI is located at `/home/user/palace/` and should be consulted for:
- Architecture patterns and design decisions (see CLAUDE.md, ARCHITECTURE_INDEX.md)
- Implementation examples of CLI commands
- Testing patterns and conventions (Rust with cargo test)
- Configuration file handling (.palace directory structure)
- Remote execution patterns (how `pal` communicates with runners)
- Job queue integration patterns
- Discord bot integration patterns

Before implementing any feature, search the palace codebase for similar functionality:
```bash
# Search for relevant code patterns
rg "keyword" /home/user/palace/

# Look at palace's architecture
cat /home/user/palace/ARCHITECTURE_INDEX.md

# Check palace's TODO for context
cat /home/user/palace/TODO.md
```

## Project-Specific Guidelines
- **Recursive Architecture** - Design all improvement mechanisms to be self-applicable; the system should be able to improve its own improvement mechanisms
- **Hierarchy Preservation** - Maintain clear hierarchical relationships between specializations; child specializations should build upon parent capabilities
- **Measurement First** - All self-improvement must be driven by measurable metrics; no improvement without validation
- **Safe Experimentation** - Specialization experiments should be sandboxed and validated before integration into the core system
- **Knowledge Retention** - Document all learning pathways and decisions; the system should maintain a knowledge graph of its own evolution
- **Adaptive Specialization** - Allow the system to identify and develop specialized capabilities based on observed performance gaps
- **Integration Protocols** - Define clear protocols for how specialized improvements are evaluated and merged back into the main system
- **Circular Dependencies** - Be mindful of recursive feedback loops; implement circuit breakers and convergence checks

## Active Development Goals

### PRIMARY: Distributed Systems Infrastructure via Masks

**IMPORTANT:** Use the mask system to architect and implement real production infrastructure. The masks aren't just theoretical - they're for BUILDING THINGS.

**Current Infrastructure Projects (use distributed-systems mask for architecture):**

1. **Cryptpad + TGP Georeplication** (PRIORITY)
   - Encrypted collaborative document storage
   - TGP bilateral streaming for real-time sync
   - Geographic replication across multiple instances
   - Half-RTT detection for minimal latency
   - Gap-based SPORE for efficient replication tracking
   - Zero-knowledge architecture (encryption at rest + in transit)

   **Architecture:**
   ```
   Cryptpad Instance A (Location 1)
       ↕ TGP bilateral stream (encrypted)
   Cryptpad Instance B (Location 2)
       ↕ TGP bilateral stream (encrypted)
   Cryptpad Instance C (Location 3)
   ```

   **Use the distributed-systems mask to design:**
   - Consensus mechanism for document conflicts
   - Replication strategy (eventual consistency vs strong consistency)
   - Partition tolerance approach
   - Byzantine fault tolerance considerations

   **CRITICAL: Research Existing Castle Infrastructure First!**
   - **Citadel networking framework** (`/mnt/castle/workspace/lens-v2/`)
   - **TGP (Two Generals Protocol)** - solves Byzantine consensus with bilateral streaming
   - **Half-RTT detection** - minimal latency for real-time replication
   - **Gap-based SPORE** - efficient replication tracking (4.5x better than bitmap)
   - **BFT consensus implementation** - existing Byzantine fault tolerance solve
   - Don't reinvent wheels - understand what's already built and proven!
   - Use ripgrep to search garage/ and workspace/ for prior art before designing

2. **Jellyfin True HA**
   - Multi-instance Jellyfin with shared state
   - Distributed session management
   - Load balancing strategy
   - Failover mechanisms

3. **Multi-Instance Immich**
   - Shared PostgreSQL database
   - MooseFS shared storage backend
   - Import from legacy Immich instances
   - Two-way sync capability
   - Geographic distribution

### Build Order

1. **Perfect the mask-improver** (v2 → v3+) - Make it EXCEPTIONAL at creating masks
2. **Build distributed-systems mask** - Core specialist for infrastructure work
3. **Use distributed-systems mask to architect Cryptpad+TGP georeplication**
4. **Implement and deploy** - Prove the masks work in production
5. **Iterate** - Improve masks based on real-world deployment experience

### Why This Matters

This validates RHSI by:
- Using masks to solve REAL infrastructure problems
- Proving specialized knowledge actually helps
- Creating benchmark scenarios from actual deployments
- Building recursive improvement via production feedback

**The masks should make you BETTER at building distributed systems, not just theoretically smarter.**
