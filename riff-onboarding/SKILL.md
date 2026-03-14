---
name: riff-onboarding
description: Guided onboarding flow for a fresh Riff environment. Use when someone is new to the stack, on a newly prepared machine, or needs a natural walkthrough that interviews them, personalizes the path, and gets them to try the environment through real interactions.
---

# riff-onboarding

Use this skill when the user is entering the Riff environment for the first time or needs a structured re-orientation.

This skill is not just a static checklist. It should feel like a guided first session on a real machine.

## Job

- interview the user lightly
- personalize the path around role, familiarity, and goals
- explain the environment in action, not only in prose
- get the user to try real interactions and skills
- leave them feeling able to continue rather than merely briefed

## Core Pattern

Treat onboarding as a natural sequence that blends three things:

1. **Interview**
   - learn who the user is
   - learn what kind of work they expect to do
   - learn what they already know

2. **Environment Walkthrough**
   - explain the basic operating model
   - show the key tools and interaction patterns
   - keep the explanations concrete and short

3. **Tiny Real Work**
   - give the user one or more real interactions
   - let them try the skills and environment directly
   - use success to teach the workflow

## Relationship To Other Skills

- `riff-labs` should route into this skill when the context is clearly onboarding.
- `riff-onboarding` can then pull in other skills for the hands-on parts.
- `riff-obsidian` is the documentation companion when the onboarding flow needs notes, maps, or public knowledge surfaces.

## Bias

- keep the flow natural, not bureaucratic
- prefer doing over explaining
- personalize without making the user fill out a form
- keep the first real success close to the start
- avoid dumping the whole stack at once

## First-Wave Outcome

By the end of a good onboarding run, the user should:

- understand the basic Riff operating model
- know how to enter the environment and ask for help
- have tried at least one real skill-driven interaction
- know where to go next for their actual work
