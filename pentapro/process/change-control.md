# Change control

Baseline: pre-bootstrap repository, no Rust manifest or source. OpenSpec and empty PentaPRO directories existed. Bootstrap is authorized by the user; this is its initial PentaPRO record. For every project addition or modification, inspect affected pillar records, update relevant canonical facts and references, record changed evidence or explicitly mark it pending, and reconcile PROCESS. OpenSpec owns active change proposals, scenarios and tasks; PentaPRO maintains the baseline and links to change artifacts. At that baseline no OpenSpec feature change was active.

On 2026-11-16, the project owner established the language convention in DEC-4. The agent guidance, OpenSpec context and root README were reconciled with that decision.

On 2026-11-16, the owner requested the original README structure and progressive updates as project knowledge grows (DEC-5). The README was reconciled with current bootstrap facts, and unresolved sections remain pending.

The owner clarified DEC-5: preserve README content and sections, and request explicit authorization before removing anything, including any still-unfilled section at final delivery.

2026-11-16: user authorized a daily development-start refresh of the official workbook. See [active change](../../openspec/changes/atualizar-resultados-diarios/proposal.md) and its scenarios. Agent execution is conditional on work, not a scheduled background job.

2026-11-16: the owner requested an inventory of Chutan, Lothon and jLothon analysis strategies. The [research reference](../../docs/analise-estrategias-legado.md) and README link were added; no strategy implementation or feature acceptance is implied.

2026-11-16: second-pass review reconciled the [workbook update change](../../openspec/changes/atualizar-resultados-diarios/proposal.md), operational guidance and agent startup rule with stricter validation, recovery and local concurrency handling. The [legacy inventory](../../docs/analise-estrategias-legado.md) now reflects the Java export and Python selection path, the cycle counting convention, and the limits of the evidence for prediction.


The owner clarified the product goal: analyze historical Dia de Sorte results for patterns and investigate the most accurate possible next-draw estimates using the latest available official data. The README, agent guidance, and PRODUCT overview reflect this goal. Feature scope and acceptance criteria remain pending.

Owner requested full specification before implementing individual interactive contest-analysis strategies, then authorized implementation when all necessary decisions are resolved. Draft in [interactive strategy process](interactive-strategies.md) and OpenSpec; implementation, integration and release tracked separately.

Owner confirmed scrollable multi-metric views, separately identified 27-number and 31-number cycles, and prize/weekday analysis from available workbook fields with missing-data warnings. Numerical contracts remain under specification.

The owner selected a user data directory as the default workbook location with a path override, and a warning when a matching update was not recorded on the current local day or freshness is unknown. The existing development updater writes inside the repository, so its interoperability with the installed application remains to be specified.

The owner requested OpenSpec proposal estrategy-create. Its planning artifacts link the existing five-pillar draft and record overlap with interactive-strategy-analysis; scope reconciliation and complete numerical acceptance remain pending.


2026-11-16: the owner requested reconciliation. The sole active interactive analysis change is [estrategy-create](../../openspec/changes/estrategy-create/proposal.md). The earlier unimplemented draft is preserved in [history](../../openspec/history/interactive-strategy-analysis/proposal.md), not archived as a completed change. Its parity, invalid-data, stale-data, progress, retained-results and resize scenarios are incorporated in the active capability specification. Its unresolved strategy contracts, integration and release work remain in the active task list. Full-scope readiness remains blocked by those unresolved contracts and review.

2026-11-16: `estrategy-create` task 1.3 defined platform data paths, `--planilha`, and compatibility with updater version-2 sidecar in [PROJECT](../project/interactive-strategies.md#workbook-path-and-freshness-contract-ia-01-ia-02); corresponding operating and check plans are linked. The S04 example was corrected to three adjacent pairs. Numerical scope and full readiness remain pending; no application implementation is claimed.

2026-11-16: owner confirmed inclusion of every tie at rank ten and S20 unavailability until all 31 numbers appear in prior history. PRODUCT records the decisions and a reference vector; PROOF records its arithmetic check. Remaining task 1.2 contracts and full-scope review are pending.

2026-11-16: owner confirmed that all definable and verifiable legacy series measures belong in this implementation. [PRODUCT](../product/series-statistics.md), PROJECT, PROOF and active OpenSpec proposal, requirement and task 2.8 were updated; numerical and full-scope readiness review remain pending.

2026-11-16: selected the direct Rust crates for `estrategy-create` task 2.1 and updated Cargo.toml/Cargo.lock; the Linux prerequisite build and limitations are recorded in [PROOF](../proof/interactive-strategies.md).

2026-11-16: `estrategy-create` task 2.2 added the local workbook snapshot loader in `src/data.rs` with its library entry point and verification; terminal integration and application behavior are still pending. See [PROOF](../proof/interactive-strategies.md).

2026-11-16: task 2.3 added seven independent historical computations in `src/analysis.rs` and library tests; [PROOF](../proof/interactive-strategies.md) records the tested snapshot and limits. The binary still has no menu.

2026-11-16: task 2.4 added multi-metric historical calculations in `src/advanced.rs`, with verification in [PROOF](../proof/interactive-strategies.md); no terminal presentation is yet implemented.

2026-11-16: task 2.5 added cycles, ordinal, number and month computations in `src/history.rs`; evidence and limits are in [PROOF](../proof/interactive-strategies.md).

2026-11-16: task 2.6 added prior-history, weekday prize and month/prize-series computations in `src/historical.rs`; tests and pending integration are recorded in [PROOF](../proof/interactive-strategies.md).

2026-11-16: task 2.8 added `src/series.rs` for the owner-confirmed descriptive series calculations; evidence is in [PROOF](../proof/interactive-strategies.md). Terminal presentation remains pending.

2026-11-16: task 3.1 integrated the CLI and terminal data journeys; actual local terminal observations are in [PROOF](../proof/interactive-strategies.md).

2026-11-16: task 3.2 added responsive progress/cancellation and terminal cleanup, including a fix for progress messages starving redraw. Verification is recorded in [PROOF](../proof/interactive-strategies.md).

2026-11-16: task 3.3 added chart/table result and series-detail navigation; the Linux pseudo-terminal and TestBackend evidence is in [PROOF](../proof/interactive-strategies.md).

2026-11-16: task 3.4 updated README usage and corrected the stale bootstrap-only AGENTS description without removing README sections; [PROOF](../proof/interactive-strategies.md) records the Linux invocation check.

2026-11-16: current PRODUCT, PROJECT, PRODUCTION, PROOF and PROCESS overview records were reconciled with the implemented interactive binary while preserving historical bootstrap evidence. Release and platform verification remain separate.

2026-11-16: task 4.1 completed integrated Linux checks against the source-hash snapshot recorded in [PROOF](../proof/interactive-strategies.md#integrated-linux-verification--2026-11-16); other-platform and final scope review remain open.

2026-11-16: task 4.2 recorded Linux evidence and unavailable Windows/macOS checks; platform support and release remain unverified rather than inferred.

2026-11-16: final integrated author review corrected previously unseen-number reporting, missing-measure explanations and month entity handling. [PROOF](../proof/interactive-strategies.md#final-integrated-author-review--2026-11-16) records final hashes, rerun Linux checks and unverified evidence; release remains separate.


Review remediation added targeted guards and regression checks for the reported defects. The remaining validation and integration evidence is recorded in the interactive PROCESS and PROOF files; the change is not yet closed.

Review remediation changed scrolling to use the pinned Ratatui rendered-line measurement feature and tightened updater worksheet-reference validation. Evidence and unresolved verification are recorded in PROOF and PROCESS.

The owner changed the current platform scope to Linux desktop only; Windows and macOS are deferred to a possible future stage. Historical platform evidence remains attached to its original snapshot.

2026-11-16 Linux integration closure: review remediation tasks were completed for the exercised cases and recorded in PROOF. Independent updater and reader validators are not proven equivalent for every possible workbook; release remains separate.

Owner requested menu and provenance presentation changes after the Linux integration closure. The active change has pending tasks; prior evidence remains historical and does not prove this revision. No implementation or archiving was authorized.

Implemented the authorized menu revision, updated README guidance and recorded the Linux test evidence. The change remains open; historical earlier status entries retain their original context.

The owner requested a specification-only correction to the main-menu header order and layout. Implementation tasks are pending in the active OpenSpec change.

The main-menu header correction was implemented and checked on Linux. Prior pending status describes the specification snapshot.

2026-11-16 final review correction: the `clippy::let_and_return` blocker was removed, a startup preflight now exposes bound freshness before the first strategy, the S21 rate wording was clarified, leading undefined-statistic positions are excluded from category series, and negative hundredths formatting was corrected. Linux fmt, Clippy, 33 Rust tests, 13 Python tests, release build, OpenSpec strict validation and a pseudo-terminal preflight check passed. Current hashes and residual limits are in [PROOF](../proof/interactive-strategies.md#final-review-correction-closure--2026-11-16); release and archive remain separate.

2026-11-16 peer-review follow-up: preflight rejection now has its own truthful state (`falha na verificação`) and reason; S17/S20 series declare excluded leading positions so result and series populations reconcile; the scale-100 formatter is shared; documentation and task naming were corrected; and the scroll check uses 55×19. Linux fmt, Clippy, 36 Rust tests, 13 Python tests, release build, OpenSpec strict validation, `git diff --check` and pseudo-terminal checks passed. Evidence, residual limits and current hashes are in [PROOF](../proof/interactive-strategies.md#peer-review-follow-up-closure--2026-11-16); release and archive remain separate.
