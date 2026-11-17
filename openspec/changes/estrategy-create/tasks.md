# Tasks

## 1. Complete specification and readiness

- [x] 1.1 Reconcile the overlapping `interactive-strategy-analysis` draft and `estrategy-create`; verify the canonical change and PentaPRO links identify one active implementation scope.
- [x] 1.2 Resolve P-01 through P-04 for S01–S22, including rounding, bins, ranking ties, never-seen values and prize denominators; record independent input/output examples and legacy differences for every selected strategy.
- [x] 1.3 Define the user data directory, explicit path override and matching freshness metadata interoperability; verify development and installed use cases are reproducible from the written contract.
- [x] 1.4 Review all five affected PentaPRO pillars and the complete selected scope; record readiness, open issues and authority before implementation.

## 2. Source and independent computations

- [x] 2.1 Select and verify compatible Rust dependencies and terminal backend; document chosen versions and run dependency/build checks.
- [x] 2.2 Implement workbook snapshot loading, validation, path selection and freshness handling; verify current Caixa workbook, malformed data, missing fields, stale and unknown metadata.
- [x] 2.3 Implement parity, repeat/recurrence, sequence/run, sum and digital-root strategies; verify independently derived examples and first-draw boundaries.
- [x] 2.4 Implement dispersion, gaps, decade/column/matrix and distance strategies; verify formulas, rounding and legacy boundary differences.
- [x] 2.5 Implement both cycles, ordinal, individual-number and month analyses; verify unfinished cycles, repeated results, absent numbers and exact rank boundaries.
- [x] 2.6 Implement historical frequency/absence, mean of roots, frequency-delay dispersion and prize/weekday series; verify prior-only inputs, unavailable values and denominators.
- [x] 2.7 Update affected PentaPRO numerical contracts and capture each strategy's passing checks against the tested snapshot.
- [x] 2.8 Implement owner-confirmed shared series measures and category details with explicit populations, zero and missing cases; verify independently derived vectors and censored intervals.

## 3. Interactive terminal

- [x] 3.1 Implement keyboard-accessible strategy selection and local-data warning/error journeys; verify user-selected path and stale/unknown-data decisions.
- [x] 3.2 Implement stage progress, cancellation and terminal restoration; verify measurable and indeterminate progress and failure paths.
- [x] 3.3 Implement charts and scrollable tables with labels, values, sample context and retained results; verify navigation, resize and too-small-terminal handling.
- [x] 3.4 Update Portuguese README and terminal usage guidance; verify documented invocation and interaction.

## 4. Integration

- [x] 4.1 Run formatting, lint, unit and integration checks and exercise the actual terminal entry point using the official workbook and representative invalid fixtures.
- [x] 4.2 Run Linux checks and reconcile OpenSpec and PentaPRO evidence with the tested revision; Windows and macOS are deferred to a possible future stage.
- [x] 4.3 Review the integrated scope against all selected strategies and report unresolved evidence or defects before considering implementation complete.

## Workflow follow-up

- Prepare release and distribution separately after implementation and integration; publish only with the applicable authority.

## Review remediation

- [x] Reconcile updater and workbook-reader validation for the exercised official, synthetic and malformed cases; verify previous-copy preservation and sparse worksheet dimensions. Full equivalence for every possible XLSX is not claimed.
- [x] Verify numerical, series eligibility, per-contest display and rounding with targeted automated regressions; exercise minimum terminal size and wrapped scrolling through the actual entry point.
- [x] Re-run Linux integration checks and reconcile evidence across PentaPRO and OpenSpec.

## 5. Menu explanation and provenance revision (implemented)

- [x] 5.1 Specify and implement accurate two- or three-paragraph Portuguese descriptions for each of the 22 strategies, covering calculation and all result sections against the existing numerical contracts and actual result views.
- [x] 5.2 Implement the 30/70 menu and explanation panes with divider, selection-driven updates, discoverable explanation scrolling, and usable resize and narrow-terminal behavior.
- [x] 5.3 Move the selected workbook path and truthful freshness status to the menu header, remove the replaced disclaimer and the standalone result source and digest lines, and add the requested blank separation. Preserve internal hash binding and stale/unknown/error journeys.
- [x] 5.4 Add meaningful checks for all descriptions, changing selection, scrolling and resizing, source path override, same-day/stale/unknown/missing/mismatched metadata, startup and return to menu, unchanged validation, and result layout. Exercise the Linux terminal and record evidence.
- [x] 5.5 Reconcile README usage and all affected PentaPRO records after implementation and verification; keep release separate.

## Review follow-up

- [x] Reject duplicate spreadsheet headers before replacement and preserve the prior files in the exercised regression.
- [x] Remove incorrect per-draw labels from pooled gaps and invalidate cached freshness after a failed reload.
- [x] Make the full selected name and workbook path available in the scrollable explanation and run Linux regression checks.

## 6. Main-menu header correction

- [x] 6.1 Render the main-menu header in the requested single-line order with dynamic selected path and truthful freshness status, preserving existing warning and recovery behavior.
- [x] 6.2 Verify the exact layout when it fits, selected path overrides, current/stale/unknown/failed-reload statuses, and narrow-terminal access to the full path; exercise the Linux terminal.
- [x] 6.3 Reconcile affected PentaPRO evidence and README guidance after verification. Keep the change unarchived.

## 7. Final review corrections

- [x] 7.1 Remove the `clippy::let_and_return` failure in `src/ui.rs` and restore the documented lint gate.
- [x] 7.2 Run a background preflight read at startup so the menu shows bound freshness before the first strategy, without starting an analysis or a download; add a regression check.
- [x] 7.3 Clarify the S21 weekday rate contract as an average known principal prize amount per known rate row, not per winner; align PentaPRO wording.
- [x] 7.4 Exclude leading undefined-statistic positions from S17/S20 category series instead of counting them as unknown events; add a regression check.
- [x] 7.5 Fix negative scale-100 formatting and add a regression check.
- [x] 7.6 Re-run formatting, lint, Rust/Python tests, release build, OpenSpec strict validation and a Linux pseudo-terminal preflight check; record the tested snapshot hashes and remaining limits in PROOF.

## 8. Peer-review follow-up

- [x] 8.1 Report a source that the background check read and rejected as "falha na verificação" instead of "ainda não verificada", and expose the validation reason in the menu explanation; add a regression check with an unreadable source.
- [x] 8.2 Declare the excluded leading undefined-statistic positions in the category-series panel so the result and series notices agree for S17/S20; add a regression check comparing both.
- [x] 8.3 Use the shared scale-100 formatter in both the advanced sections and the UI, preserving the sign of negative values; add presentation-layer and UI regression checks.
- [x] 8.4 Correct the S17/S19 naming in task 7.4 to S17/S20, matching the implemented strategy indices.
- [x] 8.5 Record the implemented path/freshness selection and the preflight failure state in the design document.
- [x] 8.6 Exercise explanation scrolling at the minimum supported terminal size (55×19) and assert that the last explanation line is reachable.
