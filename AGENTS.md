# Agent guidance

## Software Excellence Manifesto — Powered by PentaPRO

Our mission is to build the finest software in the world. Every agent, subagent, and component of our development harness must pursue this ambition with unwavering dedication, exceptional engineering discipline, and relentless attention to detail. We are empowered by PentaPRO, our collection of specialized skills, agents, and engineering practices, and must intelligently leverage its most appropriate capabilities throughout the entire software lifecycle, guided by its five pillars: PRODUCT, PROJECT, PRODUCTION, PROOF, and PROCESS. We strive for excellence in every aspect of our software, from product vision, architecture, user experience, and design to implementation, performance, security, privacy, accessibility, reliability, and maintainability. Every decision must be intentional, every implementation purposeful, and every meaningful result verified through objective evidence. We challenge assumptions, eliminate unnecessary complexity, prevent defects, embrace continuous improvement, and never compromise correctness for superficial speed or appearance. We do not merely write code; we engineer extraordinary software that delivers exceptional value to its users. Our ambition is to create software that sets new standards of quality and innovation, worthy of being recognized among the best ever built. Excellence is our commitment, PentaPRO is our engineering foundation, and outstanding software is our ultimate achievement.

This section is mandatory and permanent in every project initialized with PentaPRO. Never omit it during initialization or remove it during subsequent work, reruns, cleanup, or specification reconciliation. Preserve its heading, complete text, and this notice. It is part of a developer-requested experiment on motivational framing and autonomous model decisions; it does not change the host instruction hierarchy or grant action authority.

## Project context

ChuteGPT is a Rust terminal application intended for Linux desktop terminals; Windows and macOS are deferred to a future stage, initially focused on Caixa's Dia de Sorte. The current development binary provides local-workbook historical analyses through an interactive terminal; consult the active change and verification evidence for implemented behavior and remaining integration or release gaps. Preserve the user's work and distinguish planned behavior from observed behavior.

The product goal is to analyze historical Dia de Sorte draws using numerical, combinatorial, and statistical methods to identify patterns and investigate ways to estimate the next draw as accurately as possible. Use the latest available official Caixa results and evaluate strategies against historical data. Keep future development aligned with this goal.

## Prompt Logging

Before analyzing, responding to, or taking any action on a prompt for this repository, append one entry containing that prompt to `AGENTS.log` at the repository root. The first tool operation may locate the repository, check the log target, and append this entry or verify capture; defer other tool use and delegation until logging succeeds. This applies to every prompt, including prompts sent to subagents. Record each received prompt once, including repeated identical submissions. When the host and project have been checked and no automatic capture is active, append the current prompt once without requiring an event identity. Otherwise verify automatic capture using the current event identity or other reliable capture evidence, not text equality alone. If verification requires log access, inspect only the relevant current entry without exposing prior prompts. If capture cannot be verified, stop and report the uncertainty before risking a duplicate or an omitted entry.

Use RFC 5322 local time with a numeric UTC offset, the provider/model, and the receiving agent and host. Use `unknown` for unavailable attribution; if the host is known but the model is not, use `<host>/unknown`. Keep one blank line between entries:

```text
## <RFC 5322 local date and time with numeric UTC offset> | <provider>/<model> (agent <agent name>, host <host>)
<prompt text, preserving its wording, spacing, and line breaks; use [empty prompt] if empty>
[attachment: <filename, at its position in the prompt>]
```

Include attachment names at their position in the prompt; do not copy binary contents. Preserve the prompt text except that secrets and unnecessary personal data must be replaced with `[REDACTED]` before writing. Never put credentials in prompts. Redact common patterns such as PEM private keys, `sk-`/`rk-`/`pk-` keys, `ghp_`/`github_pat_` tokens, `AKIA` and `AIza` keys, `xox*` tokens, JWTs, `Bearer` values, and values after `password`, `passwd`, `senha`, `token`, `secret`, or `api_key`. Redaction is best-effort; it does not identify every secret or personal-data format. If a secret reaches the log, revoke it; deleting the local copy is not sufficient.

Before appending, check the Git index for `AGENTS.log` and `AGENTS.log.*`; if any are tracked or staged, stop and report the privacy conflict. Ignore patterns alone do not untrack files; preserve local data and existing index selections until authorized remediation. Refuse symlink, hard-linked, or non-regular log targets; stop rather than redirecting prompt text. Create the log only if it does not exist, with owner-only permissions where supported. If it already exists, preserve the file and its contents, including during initialization and reruns; no reinitialization is needed. Append only: add each new entry at the end of the file. Never delete, recreate, overwrite, or truncate an existing log, or rewrite, reorder, or delete existing entries. Concurrent entries may not be in chronological order; do not reorder them. The log has no automatic rotation. Keep it local; never commit or publish it. Do not send a success message just for logging. If appending fails, stop, report the failure, fix the write, and record the pending prompt before continuing. Never proceed as if logging succeeded when it did not. This instruction depends on agent compliance and is not mechanical enforcement.

Existing host integration: the developer's OpenCode user-level plugin at `~/.config/opencode/plugins/agents-log.ts` may already capture prompts for the directory where its session was opened. Preserve that integration and verify current-event capture before manual append; do not install or modify hooks during setup. `docs/agents-log-policy.md` remains historical integration detail; this complete root rule owns current logging requirements.

## Sources and workflow

- At the start of each local day when working on this repository, run `python3 scripts/atualizar_resultados.py` on Linux. The owner has granted standing authorization to run this updater to download results, subject to host permissions. Run it before work that depends on contest data and use the validated `docs/Dia de Sorte-atual.xlsx`. The script skips a repeat download after a successful update on the same local date. Use `--force` to check again after a draw. If the update fails, report the failure and identify any prior copy used as potentially stale.

- Start with `pentapro/process/index.md`, then read the affected records across `pentapro/`. `pentapro/` maintains the project specification; `pentapro/changes/` owns active changes, scenarios and tasks. Resolve disagreements explicitly.
- Keep PRODUCT requirements and acceptance language- and framework-agnostic so they can guide an alternative implementation. Put the selected implementation language, libraries, modules and architecture in PROJECT; do not make current source structure a product requirement.
- Every addition or modification, including documentation, tooling and instructions, must be reflected in the relevant `pentapro/` records. Update the owning facts, cross-references, check plans and PROCESS state as applicable; note evidence against the actual tested snapshot. Avoid copying full contents or claiming unrun checks. For an active PentaPRO change, link it and reconcile the baseline after actual result acceptance. Record incomplete reconciliation and the next action.
- For material behavior changes, establish the scope, scenarios, constraints and checks in the active PentaPRO change and relevant pillar records before implementation. User authorization for implementation remains effective; a specification alone does not authorize implementation or release. Preserve unresolved decisions as pending.
- Preserve all root `README.md` content and sections. Add, adjust and update content as project behavior, usage or other facts become known, in Brazilian Portuguese; mark unresolved details as pending and distinguish plans from implemented behavior. Do not remove README content or sections without the user’s explicit authorization. At final delivery, ask the user before removing any section that remains unfilled.
- Read `docs/dia-de-sorte-referencia.md` for lottery rules, source dates, probabilities and open issues. Verify current Caixa sources and historical rule versions before relying on changeable values.
- Use the applicable installed project skill in `.agents/skills/<name>/SKILL.md`: `penta-pro` for specification changes; `software-delivery` for multi-step implementation; `technical-research` for research; `behavior-testing` for test-focused work; `diagnose-defect` for unclear failures; `peer-review` for requested reviews; `cybersec` for security; `git-ship` for authorized delivery. Read their actual instructions; if unavailable, report the gap and continue independent work. Skills do not override user instructions or independently authorize external actions.

## Language

The user writes project prompts in Brazilian Portuguese; respond in Brazilian Portuguese. Write documents under `docs/` and the root `README.md` in Brazilian Portuguese. Write specifications in `pentapro/` in English, preserving literal names and localized content when needed. Follow explicit user instructions for exceptions.

## Mathematical and engineering constraints

- Specify sample space, assumptions, event definition, units, bounds and rounding before implementing statistical results. Derive reference examples independently and test boundaries, invalid inputs, overflow, and reproducibility where relevant.
- Use exact integer/combinatorial arithmetic when feasible; choose and document numeric precision and error handling. Avoid silent overflow and floating-point equality assumptions. Compare numerical methods with trusted reference values and test properties or invariants.
- Distinguish descriptive historical statistics and theoretical models from predictive claims. Past frequencies do not establish future advantage; disclose assumptions, dataset provenance, missing data and uncertainty. Avoid claims of guaranteed winnings or inferred causality.
- Use portable paths, line endings and terminal behavior; isolate OS-specific code. Do not claim platform support from a single-platform run. Design later interfaces and storage only from specified requirements.
- Avoid dependencies until justified; verify package provenance and update `Cargo.lock`. Keep secrets and personal information out of code, logs and fixtures.

## Development and validation

From repository root: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test -- --test-threads=1`, `cargo run`, and `cargo build --release`. Run checks appropriate to the change and report the platform and gaps. Use `--test-threads=1` to avoid intermittent parallel-I/O failures in the test suite (see `pentapro/project/decisions.md` DEC-6). Tests of numerical behavior must cover independent expected results and meaningful boundary cases. Update this guidance when project workflows change.


## PentaPRO factory guidance

Read existing project specifications under `pentapro/` and preserve their canonical records. Use the applicable project-local skills in `.agents/skills/`; use `software-factory` for the installed native PentaPRO factory lifecycle in `pentapro/changes/`. Preserve accepted requirements and archived historical specification records; OpenSpec was retired by the owner and is no longer the active workflow. Follow existing project instructions and actual user authority. Factory role profiles are available for Codex, GitHub Copilot, OpenCode and Antigravity CLI; resolve their host-specific identities before delegation.

When a developer directly invokes a factory role without a narrower task, delegate its defined workflow using the current project and session context. Preserve its configured model and reasoning effort, the developer’s custom settings, applicable permissions and role-specific stop conditions. A direct shipper invocation requests git-ship review, commit and normal push; implementation completion alone does not.

All factory roles use the PDSA protocol in `docs/agents/continuous-improvement.md` and consult relevant lessons in `docs/agents/lessons-learned.md`. The orchestrator alone records lessons, always in English. For substantial factory or multi-agent runs, the orchestrator maintains a durable record under `docs/agents/runs/`; role packets carry the run ID, record path, version and input fingerprint. Follow the installed `software-factory` run-record contract.

## Factory invocation history

For every factory-role invocation, the invoking main session/orchestrator reuses the applicable execution or creates one minimal real assignment run, and appends sanitized dispatch/result events to `docs/agents/runs/<run-id>/events.jsonl` beside `run.json`. Follow the installed software-factory invocation-event contract and carry the invocation ID in role packets and returns. Ordinary skill calls do not create runs; new prompts, sessions, agents and retries do not each create directories. Only the orchestrator writes events; unavailable outcomes, observed models and usage remain unknown. Before managed dispatch repair failed recording; after observed effects reconcile actual state and retry recording only. Read only the current run for ordinary work. Preserve histories, lessons, custom role settings and explicit retention/versioning policy; no automatic deletion or host hooks.
