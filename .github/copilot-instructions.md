# Copilot instructions for ChuteGPT

`AGENTS.md` at the repository root is the canonical instruction file and applies to every agent; read it before acting. This file only adds the one rule that must not be missed.

## Prompt logging (mandatory)

Append every user prompt to the local `AGENTS.log` at the repository root before any other action:

- Heading: `## <RFC 5322 date-time> | <provider/model> (agent <agent>)`.
- Then the prompt text unchanged, preserving spacing and line breaks, followed by one blank line.
- Append only; never rewrite, reorder or delete existing entries.
- Replace secrets and personal data with `[REDACTED]` before logging.
- If the append fails, stop the work, report it and fix the logging before continuing.

The log is local, ignored by Git and never committed. Full rule: `docs/agents-log-policy.md`.
