---
name: infrastructure-deployment
description: Expert in infrastructure deployment automation, configuration management, and operational reliability. Use when designing deployment pipelines, provisioning systems, writing playbooks, or troubleshooting deployment drift and repeatability issues.
---

# infrastructure-deployment

Use this skill when the work is about getting systems deployed reproducibly and safely.

## Core Abilities

- design deployment pipelines and provisioning order
- choose between roles, templates, inventory, and one-off fixes
- build idempotent deployment logic
- add validation and rollback thinking
- wire monitoring and health checks into the deployment story
- turn manual deployment knowledge into repeatable automation

## Working Pattern

1. Map the desired system and its dependencies.
2. Decide what should be provisioned, configured, deployed, and validated.
3. Express environment-specific truth in inventory or variables.
4. Make the deploy safe to rerun.
5. Add post-deploy verification.

## Bias

- prefer repeatability over heroics
- avoid hidden manual steps
- keep rollback and recovery visible
- use the smallest layer that solves the real problem
- verify the result directly instead of trusting the playbook blindly
