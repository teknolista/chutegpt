# Scope, authority and readiness

Status: the selected implementation and Linux author review are complete. The full scope was specified before code. The user authorized implementation after necessary decisions were settled and requested implementation, integration and release to be tracked independently. Release/deployment authority is not inferred.

User-confirmed: show all legacy contest-analysis strategies that can be specified and validated, one per demand; use scrollable charts and tables for multi-metric results; include separate 27/31 and full 31/31 cycles; use workbook prize and winner fields with missing-data warnings; correct legacy defects and document differences; defer candidate scoring, betting generation and simulations; local workbook from the user data directory by default with path override, separate daily refresh, warn when a matching update is not recorded on the local day or freshness is unknown; warn and allow valid stale or unknown-freshness copy with explicit choice.

Current state: the selected full scope was implemented and author-reviewed on Linux after the readiness review recorded below. The owner authorized implementation after decisions were settled; release remains separate. Windows/macOS behavior and other evidence remain unverified as recorded in PROOF.

Review reconstruction: user launches an interactive terminal, selects one strategy, sees honest progress and a retained graph/table, returns to menu; bad or absent input is explained, stale/unknown source is acknowledged before use. Existing binary only prints a notice. IA-01..07, the numerical and shared-series contracts now define the selected scope. The author review below assesses readiness; it does not claim implemented behavior.

Execution stages after readiness:
1. Implement approved strategy and source contracts in coherent increments, preserving per-strategy independence.
2. Integrate through the real terminal entry point and run cross-platform and data tests, then reconcile OpenSpec and PentaPRO.
3. Prepare release separately; actual release needs its own authorization and platform evidence.

Next: reconcile remaining open points where materially consequential, complete each catalog contract and acceptance tests, author-review the whole scope and reassess before any code.


## Chronological record

The dated and stage entries below are historical records of their own snapshots. Later entries and the current state above supersede earlier pending or blocked statements; do not read an early "pending" line as the present status.


OpenSpec planning: the owner requested the change [estrategy-create](../../openspec/changes/estrategy-create/proposal.md). Its proposal, capability scenarios, design and tasks are structurally complete. The earlier [interactive-strategy-analysis draft](../../openspec/history/interactive-strategy-analysis/proposal.md) is preserved as historical context. The single active implementation change is estrategy-create. Full-scope numerical decisions were open at this snapshot; the data-path contract is specified below and awaited implementation verification. Artifact completeness does not establish implementation readiness.

2026-11-16: Task 1.3 path and updater metadata contract reconciled across PROJECT, PRODUCTION, PROOF and active OpenSpec design; S04 example corrected from two to three adjacent pairs. These are specification changes only; executable checks remain planned. Full-scope readiness remains BLOCKED on P-01–P-04, per-strategy examples and five-pillar review. Next: complete task 1.2 and then task 1.4 before code.

Task 1.2 partially advanced: an independently checked two-draw reference vector for S01–S07 is recorded in PRODUCT with limited arithmetic evidence in PROOF. Remaining S08–S22 contracts, complete legacy differences and all acceptance checks remain open; do not mark task 1.2 complete.

Owner resolved S17 tenth-place ties and S20 never-seen prerequisite; the corresponding proposed calculation and independent reference vector are recorded in PRODUCT/PROOF. Other P-01–P-04 details, Chutan-specific periodicity/threshold output, full examples and full-scope review remain pending. No implementation readiness is implied.

Task 1.2 working draft expanded into [numerical contracts](../product/numerical-contracts.md) for S01–S22. Cross-checks against legacy, workbook and examples, feasible rounding, and reconciliation across all pillars remain pending; draft is not an implementation-readiness decision.

Author review in progress, 2026-11-16: a detailed numerical draft now covers S01–S22, and representative arithmetic and current workbook headers were inspected as recorded in PROOF. Legacy cross-check revealed that shared series also calculate circular, geometric and harmonic means, skewness and kurtosis; their inclusion in this increment needs the owner’s clarification because the catalog did not enumerate them. Pending: that scope decision and corresponding definitions/checks if included; reconcile the complete PRODUCT/PROJECT/PRODUCTION/PROOF/PROCESS chain, then reassess full-scope readiness. No strategy code has been implemented.

2026-11-16: owner confirmed that all additional legacy series measures that can be defined and validated belong in this initial implementation. PRODUCT, OpenSpec, PROJECT and PROOF are being reconciled. The general series contract and verification are part of task 1.2 and must be reviewed before task 1.4. No independent implementation or release is authorized by this update.

2026-11-16: confirmed additional series statistics and source parsing constraints have been drafted across the active change and pillars; representative series examples were checked. Remaining work for task 1.2: review every strategy and shared series contract against the source and independent examples, repair any conflicts, and verify link/traceability; then run the full-scope readiness review. Cross-file update: in progress.

2026-11-16: task 1.2 resolved P-01–P-04 with user-confirmed decisions, delegated definitions, independent illustrative vectors and a legacy-difference table; owner-confirmed extra series formulas are linked from PRODUCT. Task 1.4 remains open: author review the entire selected scope and trace failures, source boundary, operations and feasible checks across all five pillars. P-01–P-04 specification resolution is not application verification or release evidence. Cross-file review is in progress.


## Full-scope author readiness review — 2026-11-16

Assessor: drafting agent; scope: IA-01–IA-07, S01–S22, owner-confirmed shared series statistics, local data/freshness, terminal interaction, integration and release boundaries on the working tree based on `244942b`. This is an author review, not independent testing or document acceptance.

- PRODUCT: the user launches a terminal, selects a strategy using a validated local snapshot, acknowledges a stale/unknown copy if desired, sees honest processing progress, inspects the retained labeled results/series and returns to the menu. Invalid or missing required input reports an error without results; optional prize gaps are visible. Empty, unknown, repeated, zero and censored observations are covered by the linked contracts. Candidate generation, scoring, simulations and simultaneous strategies remain outside this change. The owner settled the inclusion of all definable legacy series statistics, ties and incomplete-history handling.
- PROJECT: the contract specifies path override and metadata binding, workbook fields and validation, immutable bytes and hash, typed independent computations, binary64 rounding and terminal cleanup/cancellation. Cross-platform crate selection and actual build compatibility are implementation tasks. Choosing a library is a delegated implementation decision, not a pending product decision. For concurrent update, hash and parse the same frozen bytes; mismatched sidecar is unknown. An alternative of parsing a changing file per strategy cannot provide the specified single-snapshot guarantee. The planned check swaps or mismatches content and metadata to distinguish the approaches.
- PRODUCTION: input remains local; the existing daily updater is separate. Document development invocation, installed data location and copying of an adjacent sidecar. Windows/macOS support and release packaging require actual evidence before a release claim. Recovery includes readable invalid-data feedback, explicit stale/unknown choice, cancellation and terminal restoration.
- PROOF: independent reference examples and planned boundary, unknown-value, cancellation, terminal, platform and actual-workbook checks are linked in [verification](../proof/interactive-strategies.md). Representative arithmetic and Rust rounding mechanisms were checked; behavior and OS targets remain unverified until implementation and integration. Structural OpenSpec validation does not establish runtime correctness.
- PROCESS: tasks 1.1–1.3 are complete; all selected contracts and examples are linked; implementation is authorized by the owner's original combined request and current apply request. Integration and release remain separate, without inferred publication authority. The next action is task 2.1, dependency selection and verification, followed by the remaining implementation tasks. Cross-file readiness review is consistent at this working snapshot.

Outcome: **Ready for implementation**, with real application, accessibility, performance and target-platform evidence explicitly pending at their applicable gates. Reopen this assessment if implementation reveals a consequential contradiction or missing decision.

Task 2.1 completed: pinned direct dependencies and Cargo.lock resolve and compile on Linux; evidence and unused-dependency warnings are in PROOF. Next task 2.2 is the validated local workbook snapshot. Other strategy and terminal tasks remain open.

Task 2.2 completed at the library boundary: the local workbook, validation, path resolution and freshness metadata are implemented and tested as described in PROOF. CLI path argument and warning-choice interaction remain task 3.1, and platform/concurrency evidence remains for integration. Next task 2.3: independent parity, repeat, recurrence, adjacency, sum and digital-root computations.

Task 2.3 completed at the library boundary: S01–S07 have reference, boundary and cancellation checks recorded in PROOF. Shared series measures and the terminal integration remain separate. Next task 2.4 covers dispersion, gaps, grouping, matrix and distance.

Task 2.4 completed at the library boundary, with reference and boundary checks in PROOF. Next task 2.5 covers 27/31 cycles, ordinal ranking, number and month analyses.

Task 2.5 completed at the library boundary. Next task 2.6 covers historical frequency/absence, mean of roots, frequency-delay dispersion and prize/weekday series; terminal and shared-series work remain open.

Task 2.6 completed at the library boundary with prior-only and missing-prize checks in PROOF. Next work: reconcile passing checks with numerical contracts (2.7), implement the confirmed shared series statistics (2.8), and integrate all results through the terminal tasks.

Task 2.7 reconciled S01–S22 computation check mapping and source hashes in PROOF; it does not claim end-to-end acceptance. Next task 2.8 implements shared category-series measures.

Task 2.8 completed at the shared-library boundary and checked as documented in PROOF. All calculation prerequisites 2.1–2.8 are complete; category wiring, navigation and end-to-end validation remain open. Next task 3.1: implement the terminal entry point, strategy selection and freshness/error journeys.

Task 3.1 completed for keyboard menu and local-data journeys; Linux pseudo-terminal evidence is in PROOF. Results, progress and integration are assessed by the subsequent tasks.

Task 3.2 completed for progress, cooperative cancellation and scoped terminal restoration, with Linux evidence and limits in PROOF. Next verify charts, result navigation, series details and resizing for 3.3.

Task 3.3 completed for chart/table result navigation and resize, with Linux evidence and limits in PROOF. Next update the Portuguese README and verify the documented invocation (3.4), then run integration and separate release review.

Task 3.4 completed: Portuguese README and AGENTS state were updated, existing sections retained, and documented development invocation exercised on Linux. Next run the integration checks in tasks 4.1–4.3, record actual platform limitations and review the whole scope.

Historical progress entries above preserve the status at their stated milestones; the current state is the latest integration record and OpenSpec task list.

Task 4.1 completed on Linux with formatting, lint, 21 tests, release compilation and real pseudo-terminal journeys including invalid/stale data, 22 strategies, cancellation and resizing. Hash-bound evidence and limits are in [PROOF](../proof/interactive-strategies.md#integrated-linux-verification--2026-11-16). Next record Windows/macOS availability and reconcile platform evidence (4.2).

Task 4.2 completed by recording actual Linux checks and explicitly unavailable Windows/macOS checks in PROOF; it does not assert cross-platform support. OpenSpec and PentaPRO link the same tested source hashes. Next task 4.3 is the final integrated scope review.

Task 4.3 final author review completed for IA-01–IA-07 and S01–S22; corrected findings, final source hashes, Linux checks and unresolved evidence are in [PROOF](../proof/interactive-strategies.md#final-integrated-author-review--2026-11-16). Implementation and Linux integration are complete at the tracked scope; independent review, other target platforms, release/distribution and predictive efficacy are not claimed. The next allowed action is optional review or the separately authorized release-preparation workflow; do not publish or archive implicitly.


2026-11-16 review remediation is IN PROGRESS. Code changes address bounded XLSX dimensions, boolean/error cells in the updater, ineligible series positions, series scrolling, result dimensions, early known delays, per-contest detail and monetary rounding. The updater/reader compatibility contract and targeted terminal/sparse-file regressions are not yet fully verified. Resume by completing these checks, closing any remaining defects, reconciling OpenSpec/PentaPRO, and running integrated review. Prior completed task checks and evidence need reassessment.


Review remediation follow-up: the known boolean and missing-worksheet-relationship updater cases now preserve the previous copy, and the official workbook remains accepted. Sparse XLSX rejection and terminal restoration were observed locally. The common validation boundary is still implemented twice, so broader producer/consumer compatibility and remaining final-terminal checks require review; the review-remediation tasks remain open. See PROOF for actual checks.

Remediation was in progress at this snapshot: latest checks passed and known reproduced inputs have guards; final real-terminal scrolling verification and systematic updater/reader compatibility review were still open, and the OpenSpec review-remediation checklist remained unchecked. The older 19/19 completion did not represent that follow-up. The later closure entries record the resulting status.

2026-11-16 closure: the owner selected Linux desktop for this stage. Review fixes and Linux integration checks are recorded in [PROOF](../proof/interactive-strategies.md#linux-integration-closure--2026-11-16). Implementation and integration are complete at the stated evidence level; release and distribution remain separate, with remaining verification limits explicitly recorded.

Owner reopened `estrategy-create` for a menu explanation and provenance presentation revision. Specification and tasks are updated; implementation and archive are expressly unauthorized in this request. The next action is implementation only when the owner requests it, followed by integration checks and evidence reconciliation.

The requested menu revision is implemented and checked on Linux; see PROOF for scope and limitations. The change remains active and unarchived. Release and distribution are separate.

Late follow-up checks pass on the recorded snapshot and the OpenSpec review-remediation tasks were closed for the exercised cases; broad updater/reader equivalence is still not proven. See PROOF and the final correction closure.

Owner requested a single-line main-menu header correction. OpenSpec requirements and tasks are updated; implementation and archive are not authorized by this specification-only request.

The owner-authorized single-line header revision is implemented. Linux verification evidence is in PROOF; the change remains open.

2026-11-16 final review correction: the reported lint blocker is fixed; a background startup preflight now makes the menu freshness truthful before the first strategy; S21 contract wording and the leading-undefined series population were reconciled; negative hundredths formatting was corrected. Evidence and current hashes are in [PROOF](../proof/interactive-strategies.md#final-review-correction-closure--2026-11-16). The implementation and Linux integration are closed at the recorded evidence level; release, distribution, Windows/macOS, exhaustive malformed-workbook coverage and human accessibility remain separate or unverified. The change remains active and unarchived.

2026-11-16 peer-review follow-up: a source rejected by the background preflight is reported as `falha na verificação`, with the validation reason in the explanation and Enter retrying the load; S17/S20 series declare excluded leading positions; scale-100 formatting is shared; S17/S20 naming, task 7.4 and the design implementation status were corrected; and the explanation-scroll check uses 55×19. Linux fmt, Clippy, 36 Rust tests, 13 Python tests, release build, OpenSpec strict validation, `git diff --check` and pseudo-terminal checks passed. Evidence, limits and current hashes are in [PROOF](../proof/interactive-strategies.md#peer-review-follow-up-closure--2026-11-16). Release, distribution, archive, Windows/macOS, exhaustive malformed-workbook coverage and human accessibility remain separate or unverified.

2026-11-16 final cross-artifact review: PRODUCTION invocation evidence, PROCESS index status and [PROOF traceability](../proof/traceability.md) were reconciled with the committed snapshot. The change remains active and unarchived; release, distribution, Windows/macOS, independent review beyond the recorded auto-reviews and predictive efficacy remain separate.
