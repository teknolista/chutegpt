# Agent guidance

ChuteGPT is a Rust terminal application intended for Linux desktop terminals; Windows and macOS are deferred to a future stage, initially focused on Caixa's Dia de Sorte. The current development binary provides local-workbook historical analyses through an interactive terminal; consult the active change and verification evidence for implemented behavior and remaining integration or release gaps. Preserve the user's work and distinguish planned behavior from observed behavior.

The product goal is to analyze historical Dia de Sorte draws using numerical, combinatorial, and statistical methods to identify patterns and investigate ways to estimate the next draw as accurately as possible. Use the latest available official Caixa results and evaluate strategies against historical data. Keep future development aligned with this goal.

## Sources and workflow

- At the start of each local day when working on this repository, run `python3 scripts/atualizar_resultados.py` on Linux. Run it before work that depends on contest data and use the validated `docs/Dia de Sorte-atual.xlsx`. The script skips a repeat download after a successful update on the same local date. Use `--force` to check again after a draw. If the update fails, report the failure and identify any prior copy used as potentially stale.

- Start with `pentapro/process/index.md`, then read the affected records across `pentapro/`. `pentapro/` maintains the project specification; `openspec/` owns active changes, scenarios and tasks. Resolve disagreements explicitly.
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

From repository root: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo run`, and `cargo build --release`. Run checks appropriate to the change and report the platform and gaps. Tests of numerical behavior must cover independent expected results and meaningful boundary cases. Update this guidance when project workflows change.
