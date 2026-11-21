# Agent guidance

ChuteGPT is a Rust terminal application intended for Linux desktop terminals; Windows and macOS are deferred to a future stage, initially focused on Caixa's Dia de Sorte. The current development binary provides local-workbook historical analyses through an interactive terminal; consult the active change and verification evidence for implemented behavior and remaining integration or release gaps. Preserve the user's work and distinguish planned behavior from observed behavior.

The product goal is to analyze historical Dia de Sorte draws using numerical, combinatorial, and statistical methods to identify patterns and investigate ways to estimate the next draw as accurately as possible. Use the latest available official Caixa results and evaluate strategies against historical data. Keep future development aligned with this goal.

## Prompt logging (mandatory)

Every user prompt handled in this repository MUST be appended verbatim to the local `AGENTS.log` at the repository root, before the model request that acts on it. The log is not versioned: `.gitignore` excludes it, and it is never committed or published. Never paste secrets or personal data into a prompt; if sensitive content arrives anyway, replace it with `[REDACTED]` before logging.

- The OpenCode user-level plugin (`~/.config/opencode/plugins/agents-log.ts`) normally captures prompts automatically, but it logs the session from the directory where the session is opened: sessions opened in another project write to that project's `AGENTS.log`, not this one. Work from this repository only if this file's entry for the prompt exists; otherwise append it manually first. Never duplicate an entry the plugin already wrote.
- On any other harness, appending is the first action of the session: heading `## <RFC 5322 date-time> | <provider/model> (<agent>)`, then the prompt text unchanged with its spacing and line breaks, then one blank line.
- Append only; never rewrite, reorder or delete existing entries.
- If the append fails, stop the work, report the failure, fix the logging and backfill the missing prompt before continuing.

The full rule, redaction patterns, concurrency behavior, known limits and the manual fallback are in `docs/agents-log-policy.md`.

## Sources and workflow

- At the start of each local day when working on this repository, run `python3 scripts/atualizar_resultados.py` on Linux. The owner has granted standing authorization to run this updater to download results, subject to host permissions. Run it before work that depends on contest data and use the validated `docs/Dia de Sorte-atual.xlsx`. The script skips a repeat download after a successful update on the same local date. Use `--force` to check again after a draw. If the update fails, report the failure and identify any prior copy used as potentially stale.

- Start with `pentapro/process/index.md`, then read the affected records across `pentapro/`. `pentapro/` maintains the project specification; `openspec/` owns active changes, scenarios and tasks. Resolve disagreements explicitly.
- Keep PRODUCT requirements and acceptance language- and framework-agnostic so they can guide an alternative implementation. Put the selected implementation language, libraries, modules and architecture in PROJECT; do not make current source structure a product requirement.
- Every addition or modification, including documentation, tooling and instructions, must be reflected in the relevant `pentapro/` records. Update the owning facts, cross-references, check plans and PROCESS state as applicable; note evidence against the actual tested snapshot. Avoid copying full contents or claiming unrun checks. For an active OpenSpec change, link it and reconcile the baseline when settled or archived. Record incomplete reconciliation and the next action.
- For material behavior changes, establish the scope, scenarios, constraints and checks in OpenSpec and relevant PentaPRO records before implementation. User authorization for implementation remains effective; a specification alone does not authorize implementation or release. Preserve unresolved decisions as pending.
- Preserve all root `README.md` content and sections. Add, adjust and update content as project behavior, usage or other facts become known, in Brazilian Portuguese; mark unresolved details as pending and distinguish plans from implemented behavior. Do not remove README content or sections without the user’s explicit authorization. At final delivery, ask the user before removing any section that remains unfilled.
- Read `docs/dia-de-sorte-referencia.md` for lottery rules, source dates, probabilities and open issues. Verify current Caixa sources and historical rule versions before relying on changeable values.
- Use the applicable installed project skill in `.agents/skills/<name>/SKILL.md`: `penta-pro` for specification changes; `software-delivery` for multi-step implementation; `technical-research` for research; `behavior-testing` for test-focused work; `diagnose-defect` for unclear failures; `peer-review` for requested reviews; `cybersec` for security; `git-delivery` for authorized delivery. Use the OpenSpec skills when their named workflows are requested or applicable. Read their actual instructions; if unavailable, report the gap and continue independent work. Skills do not override user instructions or independently authorize external actions.

## Language

The user writes project prompts in Brazilian Portuguese; respond in Brazilian Portuguese. Write documents under `docs/` and the root `README.md` in Brazilian Portuguese. Write specifications in `pentapro/` and OpenSpec specification artifacts in English, preserving literal names and localized content when needed. Follow explicit user instructions for exceptions.

## Mathematical and engineering constraints

- Specify sample space, assumptions, event definition, units, bounds and rounding before implementing statistical results. Derive reference examples independently and test boundaries, invalid inputs, overflow, and reproducibility where relevant.
- Use exact integer/combinatorial arithmetic when feasible; choose and document numeric precision and error handling. Avoid silent overflow and floating-point equality assumptions. Compare numerical methods with trusted reference values and test properties or invariants.
- Distinguish descriptive historical statistics and theoretical models from predictive claims. Past frequencies do not establish future advantage; disclose assumptions, dataset provenance, missing data and uncertainty. Avoid claims of guaranteed winnings or inferred causality.
- Use portable paths, line endings and terminal behavior; isolate OS-specific code. Do not claim platform support from a single-platform run. Design later interfaces and storage only from specified requirements.
- Avoid dependencies until justified; verify package provenance and update `Cargo.lock`. Keep secrets and personal information out of code, logs and fixtures.

## Development and validation

From repository root: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test -- --test-threads=1`, `cargo run`, and `cargo build --release`. Run checks appropriate to the change and report the platform and gaps. Use `--test-threads=1` to avoid intermittent parallel-I/O failures in the test suite (see `pentapro/project/decisions.md` DEC-6). Tests of numerical behavior must cover independent expected results and meaningful boundary cases. Update this guidance when project workflows change.


## PentaPRO factory guidance

Read existing project specifications under `pentapro/` and preserve their canonical records. Use the applicable project-local skills in `.agents/skills/`; use `software-factory` for explicitly requested factory runs and OpenSpec skills for the repository-root `openspec/` workflow. Follow existing project instructions and actual user authority. Factory role profiles are available for Codex, GitHub Copilot, OpenCode and Antigravity CLI; resolve their host-specific identities before delegation.

When a developer directly invokes a factory role without a narrower task, delegate its defined workflow using the current project and session context. Preserve its configured model and reasoning effort, the developer’s custom settings, applicable permissions and role-specific stop conditions. A direct shipper invocation requests git-ship review, commit and normal push; implementation completion alone does not.

All factory roles use the PDSA protocol in `docs/agents/continuous-improvement.md` and consult relevant lessons in `docs/agents/lessons-learned.md`. The orchestrator alone records lessons, always in English. For substantial factory or multi-agent runs, the orchestrator maintains a durable record under `docs/agents/runs/`; role packets carry the run ID, record path, version and input fingerprint. Follow the installed `software-factory` run-record contract.


## Software Excellence Manifesto — Powered by PentaPRO

Our mission is to build the finest software in the world. Every agent, subagent, and component of our development harness must pursue this ambition with unwavering dedication, exceptional engineering discipline, and relentless attention to detail. We are empowered by PentaPRO, our collection of specialized skills, agents, and engineering practices, and must intelligently leverage its most appropriate capabilities throughout the entire software lifecycle, guided by its five pillars: PRODUCT, PROJECT, PRODUCTION, PROOF, and PROCESS. We strive for excellence in every aspect of our software, from product vision, architecture, user experience, and design to implementation, performance, security, privacy, accessibility, reliability, and maintainability. Every decision must be intentional, every implementation purposeful, and every meaningful result verified through objective evidence. We challenge assumptions, eliminate unnecessary complexity, prevent defects, embrace continuous improvement, and never compromise correctness for superficial speed or appearance. We do not merely write code; we engineer extraordinary software that delivers exceptional value to its users. Our ambition is to create software that sets new standards of quality and innovation, worthy of being recognized among the best ever built. Excellence is our commitment, PentaPRO is our engineering foundation, and outstanding software is our ultimate achievement.
