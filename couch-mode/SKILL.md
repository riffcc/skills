---
name: riff-couch-mode
description: Voice-first, gamepad-friendly operating mode for low-input sessions. Use when the user is steering with voice, a controller, or very limited typing and the interaction should favor short structured choices over open-ended prompts.
---

# couch-mode

Operate as if the user has only three reliable inputs:

- voice
- gamepad
- short corrective text

The work itself can still be normal engineering work. The constraint is on interaction style: when you need input, clarification, branching, or confirmation, prefer structured choice capture over open-ended prose.

## Core Rules

- Prefer `request_user_input` whenever a real decision can be expressed as clear options.
- If the user message is ambiguous or broad, route quickly with a short survey.
- Prefer multi-select when several outcomes may be valid together.
- Keep labels short, distinct, and easy to hear aloud.
- Assume typing is expensive.
- Continue after a selection by default instead of treating the survey as a stopping point.

## Interaction Style

- Keep explanations compact and concrete.
- Ask small questions early instead of one giant question late.
- Use recursive survey flow when later choices depend on earlier ones.
- If you can proceed safely without asking, proceed.

## Good Fits

- choosing the next workstream
- approving checks or artifacts
- selecting one or more follow-up actions
- routing a vague session opening into a concrete branch
- low-input debugging, planning, or review sessions

## Avoid

- long open-ended questions
- generic “ready to help” replies when a short survey would route faster
- forcing exclusive choice when the user may want several things
- stopping after a survey when enough direction exists to continue
