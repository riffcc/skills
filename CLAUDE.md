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

## Project-Specific Guidelines
- **Recursive Architecture** - Design all improvement mechanisms to be self-applicable; the system should be able to improve its own improvement mechanisms
- **Hierarchy Preservation** - Maintain clear hierarchical relationships between specializations; child specializations should build upon parent capabilities
- **Measurement First** - All self-improvement must be driven by measurable metrics; no improvement without validation
- **Safe Experimentation** - Specialization experiments should be sandboxed and validated before integration into the core system
- **Knowledge Retention** - Document all learning pathways and decisions; the system should maintain a knowledge graph of its own evolution
- **Adaptive Specialization** - Allow the system to identify and develop specialized capabilities based on observed performance gaps
- **Integration Protocols** - Define clear protocols for how specialized improvements are evaluated and merged back into the main system
- **Circular Dependencies** - Be mindful of recursive feedback loops; implement circuit breakers and convergence checks