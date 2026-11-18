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

## 9. Header consistency across screens, description expansion and explanation pane cleanup

- [x] 9.1 Render the uniform provenance header `ChuteGPT — Análise Histórica Descritiva | Fonte: <selected path> | <freshness status>` across all strategy screens (result views, category series details, progress, and warning screens), eliminating the legacy fallback `ChuteGPT — análise histórica descritiva`.
- [x] 9.2 Remove the redundant `Fonte: <selected path>` text from the main-menu strategy explanation pane, keeping only the strategy title, calculation explanation, and preflight validation failure details when applicable.
- [x] 9.3 Author exactly three Portuguese paragraphs for all 22 strategies: a concise definition followed by detailed calculation, input/population, eligibility, strictly-prior rules where applicable, and resulting distribution/sections.
- [x] 9.4 Add and update automated tests verifying the uniform header across all screens, absence of redundant "Fonte:" in the explanation pane, and presence and keyboard-scrollability of the expanded multi-paragraph descriptions for all 22 strategies at supported terminal sizes (including 55×19).
- [x] 9.5 Reconcile affected PentaPRO records (PRODUCT, PROJECT, PROOF, PROCESS) and root README.md after implementation and verification. Keep the change unarchived until explicitly authorized.

## 10. Peer-review remediation: narrow-terminal header path (2026-11-17)

- [x] 10.1 Decide the narrow-terminal requirement: keep the complete selected path visible instead of relying on the explanation pane, which no longer repeats the source.
- [x] 10.2 Wrap the provenance header onto additional lines measured with Ratatui's word wrapper, bounded so the body keeps at least eight rows and the key legend two, and remove the previous two-line fallback that repeated only the freshness status.
- [x] 10.3 Add a TestBackend regression at 55×19 and 100×24 asserting that a long selected path is rendered in full and that the menu body and explanation title stay usable; exercise the same case in a Linux pseudo-terminal.
- [x] 10.4 Reconcile the capability specification, design, README, PRODUCT, PROJECT, PROOF and PROCESS records, and correct the DEC-6 mechanism description.

## 11. Exceptional selected-path overflow access (2026-11-17)

- [x] 11.1 Specify the owner-selected behavior for paths exceeding the bounded header: an explicit header notice and a dedicated scrollable provenance view that preserves the underlying screen state.
- [x] 11.2 Implement keyboard access to the full selected path when it cannot fit in the reserved header height; preserve menu, result, warning, error and in-progress processing state while the view is open.
- [x] 11.3 Add TestBackend regressions at the minimum supported size for a path beyond the header budget, including a long path component, complete beginning/end access by scrolling, overflow notice, unchanged fitting-path header, and return to the same underlying screen state.
- [x] 11.4 Exercise the overflow path in a Linux pseudo-terminal, then run targeted UI tests and the required Linux checks plus strict OpenSpec validation; record tested snapshot evidence and limits.
- [x] 11.5 Reconcile PRODUCT, PROJECT, PROOF, PROCESS, README and traceability with the implemented behavior and actual verification evidence; keep the change unarchived.

## 12. Dynamic scroll viewport remediation (2026-11-17)

- [x] 12.1 Derive the visible scrolling viewport dynamically from `header_height` and screen layout for menu explanation, series details and result values in `src/ui.rs`, replacing the static `saturating_sub(8)` and `saturating_sub(17)` offsets.
- [x] 12.2 Add regression tests verifying that explanation scrolling reaches the final word at 55×19 when the selected path wraps to 3 or more header lines, and that result details scrolling adjusts to dynamic header heights.
- [x] 12.3 Reconcile affected PentaPRO records (PRODUCT, PROJECT, PROOF, PROCESS) and OpenSpec artifacts with the tested revision.

## 13. Peer-review robustness remediation (2026-11-17)

- [x] 13.1 Bound the bytes actually inflated per ZIP entry in addition to the declared uncompressed total, rejecting entries that understate their size.
- [x] 13.2 Reject text cells above 64 KiB before Calamine materializes the dense worksheet range.
- [x] 13.3 Treat wholly empty rows before the first data row as spacing while still rejecting an empty row between data rows.
- [x] 13.4 Accept ISO 8601 date strings with and without a time component, in addition to `DD/MM/YYYY` text and Excel date cells.
- [x] 13.5 Recheck draw invariants at the analysis entry points and reject invalid `Draw` values instead of shifting or indexing out of range.
- [x] 13.6 Consolidate the hundredths rounding rule in `src/numeric.rs` and dispatch pooled gaps by an explicit metric flag rather than the display title.
- [x] 13.7 Mark unsupported display scales explicitly and replace direct month indexing with a checked lookup.
- [x] 13.8 Add regression tests and reconcile the capability spec, PROJECT, PROOF, PROCESS and decisions records with the tested snapshot.

## 14. Review remediation: verification gate hardening (2026-11-18)

- [x] 14.1 Replace every silently skipping test on the gitignored official workbook with an always-on synthetic fixture generated in the test (`src/test_fixture.rs`); keep the official golden values as an explicitly loud check when the local workbook is present.
- [x] 14.2 Promote the `Keep the selected workbook path accessible` capability section to a top-level requirement (`###`) and reconcile the provenance scenario grouping; revalidate the strict OpenSpec change.
- [x] 14.3 Replace the machine-specific fixture paths in UI tests with a neutral `/opt/fixture/` display path that reproduces the previous header layout, keeping the recorded display evidence valid.
- [x] 14.4 Bound `u64` casts in the floating-point prize/number paths by 2^64 exactly, aligning both `integer` and `cents`, and collapse the test-only header alias.
- [x] 14.5 Re-run format, Clippy, Rust and Python tests, release build and strict OpenSpec validation; record the tested snapshot and limits in PROOF.

## 15. Final cross-artifact review (2026-11-18)

- [x] 15.1 Preserve at least one visible result-detail row at 55×19 under every bounded header height, using a shared adaptive result layout for rendering and scrolling; add a rendered end-marker regression.
- [x] 15.2 Complete invalid-draw rejection in the simple analyses and ordinal entry point; verify all 22 presentation routes reject malformed draws and preserve cancellation errors.
- [x] 15.3 Verify ISO text/date-time input and the 64 KiB text-cell boundary with real XLSX fixtures; reconcile input contracts with accepted formats.
- [x] 15.4 Reconcile current task counts, test references, asset inventory, README and review/evaluation records; rerun Linux gates and terminal checks before the authorized commit and push.
