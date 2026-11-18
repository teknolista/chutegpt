# Claude Code instructions

@AGENTS.md

## Prompt logging (mandatory)

Every user prompt handled in this repository must be appended to the local `AGENTS.log` at the repository root before any other action. Claude Code has no automatic logger here, so append the entry yourself:

- Heading: `## <RFC 5322 date-time> | <provider/model> (agent <agent>)`.
- Then the prompt text unchanged, preserving spacing and line breaks, followed by one blank line.
- Append only; never rewrite, reorder or delete existing entries.
- Replace secrets and personal data with `[REDACTED]` before logging.
- If the append fails, stop the work, report it and fix the logging before continuing.

The log is local and not versioned. Full rule: `docs/agents-log-policy.md`.
