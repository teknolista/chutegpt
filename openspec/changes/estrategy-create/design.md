# Design

## Context

At proposal time the Cargo binary only printed a notice; the implementation recorded in the later sections replaced that bootstrap behavior. The development updater writes an XLSX file and update metadata under the repository's docs directory. Product requirements, strategy inventory and remaining numerical decisions are in the PentaPRO interactive-strategies records.

## Goals / Non-Goals

**Goals:** isolate verified draw data, numerical strategies and terminal presentation while supporting cancellation and observable progress.

**Non-Goals:** implement candidate scoring, betting generation, simulations or combined strategy execution.

## Decisions

- **Snapshot boundary:** read and validate one immutable snapshot with contest ID, date, seven sorted distinct numbers, month and available prize fields. Retain source, hash and update provenance. This avoids a changing workbook producing inconsistent results. Streaming the file separately for each strategy would require additional consistency controls.
- **Strategy boundary:** use independent computations returning typed results and explicit eligible populations. A strategy receives only the snapshot and settings, not terminal state. This supports independent numerical checks and later composition. Shared helpers are appropriate when formulas actually agree.
- **Terminal:** Ratatui with Crossterm is the selected implementation and was built and exercised on Linux. Exact compatible versions and Linux evidence are recorded in PentaPRO PROJECT and PROOF; other OS/backends remain unverified. Keep progress and cancellation responsive and use scoped cleanup for terminal restoration. Bar charts and scrollable tables support differing result sizes.
- **Data location:** choose a Linux user data location and provide a path override, per owner decision. The existing updater's repository path is supplied explicitly during development. A freshness record applies only if it matches the file hash and source; same-day verification does not prove the publisher has posted a subsequent draw.
- **Numerics:** preserve integer counts, define rounding and precision explicitly, and use strictly prior draws for historical comparisons. P-01 through P-04 and shared series are resolved in linked PRODUCT contracts; reproducibility and examples guide changes from legacy behavior.

## Risks / Trade-offs

- **Numerical correctness** → implement the completed strategy-by-strategy formulas, displays and reference examples, and verify each against independent checks.
- **Old or absent data** → expose freshness and let the user choose whether to proceed with a valid copy.
- **Terminal and dependency differences** → test real supported platforms; do not infer Windows or macOS behavior from Linux.
- **Unbounded computation** → include measurable progress, cancellation and representative efficiency checks.

## Migration Plan

Keep the development updater as a separate operation. Support explicit selection of its workbook; the Linux user-data location and sidecar-copy contract are documented in PentaPRO PROJECT. Installed-location use, packaging and distribution remain unverified, and no existing application data migration or release is implied.

## Open Questions

Precise platform support versions and distribution packaging can be settled for the release gate after implementation, provided actual tested platforms and limitations are reported.

## Data path and updater interoperability

The delegated technical selection and full sidecar validation contract are in [PentaPRO PROJECT](../../../pentapro/project/interactive-strategies.md#workbook-path-and-freshness-contract-ia-01-ia-02). `--planilha PATH` overrides the platform user-data default. The development invocation selects `docs/Dia de Sorte-atual.xlsx`; installed use copies the workbook and adjacent updater sidecar or explicitly accepts unknown freshness. Path resolution, the explicit override, sidecar validation and hash binding are implemented in `src/data.rs` and exercised on Linux; installing a workbook in an installed user-data location remains an open integration step.

## Current platform scope

The owner selected Linux desktop as the only target for this stage. Windows and macOS are deferred to a possible future stage. Historical platform notes describe the earlier scope.

## Menu and result presentation revision (implemented 2026-11-16)

The owner requested the source path and verified/unknown/stale status in the main-menu header, and a 30/70 list/explanation layout with a vertical divider. Source status must reflect actual matching metadata, including before the first analysis and after returning from results; the implementation performs a background preflight read so the menu can show the bound status before a strategy is selected, while retaining snapshot/hash binding internally and all existing stale/unknown choice flows. Long two- or three-paragraph PT-BR strategy descriptions are keyboard-scrollable without conflicting with selection. Selection and text access are preserved across resize; narrow terminals use the existing resize prompt. The two standalone provenance lines are removed from results and the requested blank separator is retained. Details of input handling remain an implementation choice, subject to the scenarios and checks. Implementation and Linux evidence are recorded in [PROOF](../../../pentapro/proof/interactive-strategies.md).

## Single-line main-menu header (implemented 2026-11-16)

The owner specified the order and labels `ChuteGPT — Análise Histórica Descritiva | Fonte: <selected path> | <freshness status>` on one line when it fits. The example date and path are illustrative, not constants. The implementation keeps truthful verified, stale and unknown states, performs a background preflight read for the initial menu status, and wraps the same header text onto the extra lines it needs when the terminal is narrow, so the complete path stays visible without repeating it in the explanation pane. Implementation and Linux evidence are recorded in [PROOF](../../../pentapro/proof/interactive-strategies.md).

## Preflight failure state (implemented 2026-11-16)

A background check that reads and rejects the selected source is reported as `falha na verificação`, never as an unverified source; the menu explanation carries the validation reason and Enter retries the load. A successful preflight, load or reload restores the bound freshness status, and the failed-reload journey keeps the local snapshot invalidated. Implementation and Linux evidence are recorded in [PROOF](../../../pentapro/proof/interactive-strategies.md).

## Uniform screen header across strategy screens and explanation cleanup (2026-11-17)

The owner identified that while the main-menu header displays `ChuteGPT — Análise Histórica Descritiva | Fonte: <selected path> | <freshness status>`, navigating into any strategy screen reverted the top header to the legacy label `ChuteGPT — análise histórica descritiva`. The shared header renderer now uses the uniform format in menu, loading, working, confirmation, result, series and error states, keeping the selected path and actual freshness visible throughout the journey.

The right-hand strategy explanation pane no longer appends `Fonte: <selected path>` because the selected path is already displayed in the header. The pane retains the strategy title, calculation text and preflight validation failure detail when applicable.

The header measures the wrapped provenance text with Ratatui's own word wrapper and grows to show it while bounded so the body keeps at least eight rows and the key legend two. A path that exceeds this height budget requires the overflow view below; it must not be represented as fully visible by the capped header. Paths that fit were verified by TestBackend and a Linux pseudo-terminal at 55×19 with a long `--planilha` path (see [PentaPRO PROOF](../../../pentapro/proof/interactive-strategies.md#narrow-terminal-header-path-closure--2026-11-17)).

Each of the 22 strategies now has a three-paragraph Portuguese explanation: the concise definition and two detailed paragraphs describing inputs, calculation, eligibility, strictly-prior history where applicable, and result sections. The explanation remains independently keyboard-scrollable without conflicting with list selection. Scroll measurement uses the actual 30/70 Ratatui pane width; Linux TestBackend evidence at 55×19 and 100×24 is recorded in [PentaPRO PROOF](../../../pentapro/proof/interactive-strategies.md#header-consistency-and-expanded-strategy-explanations--2026-11-17).

## Exceptional selected-path overflow (owner-selected 2026-11-17)

The header retains its order and wraps the complete selected path while it fits the bounded header height. When it cannot fit without displacing the minimum body or key legend, render a concise overflow notice instead of a misleading partial path. The `P` shortcut opens a dedicated view that wraps the complete displayed path to the current terminal width and supports vertical keyboard scrolling. Treat this view as an overlay so asynchronous preflight/loading/calculation polling and the underlying menu, confirmation, result, series or error state continue unchanged; `Esc` or `P` closes it and restores that same state. Keep the existing header rendering unchanged for paths that fit. Verify long unbroken path components, viewport boundaries, resize behavior, and preservation of an active processing state.

## Dynamic scroll viewport derivation (implemented 2026-11-17)

Static vertical subtraction offsets in keyboard scroll clamping (`saturating_sub(8)` for menu/series, `saturating_sub(17)` for results) assumed a constant 4-row header. When narrow terminals cause the provenance header to wrap across 3 or more lines (header height 5 or greater), those static offsets overclamped scroll positions and prevented users from reaching the bottom lines of strategy descriptions and result details. The visible scroll viewport is now derived dynamically from `header_height(&display_header(app, area), area)` and the explicit layout constraints of each screen state (`body_area_height`), ensuring complete keyboard reachability regardless of header wrapping.

## Final review corrections (2026-11-18)

The header budget still retains eight body rows plus two legend rows. Within the result body, a shared `result_areas` layout reserves two subtitle rows and at least three detail-panel rows (two borders and one content row), shrinking the chart below seven rows when necessary. Scrolling measures that same detail rectangle. This closes the zero-readable-row case missed by the earlier dynamic-offset tests. The simple-analysis and ordinal entry points now complete the shared draw validation contract; ISO text and fractional date-time parsing and real XLSX size-boundary checks are covered in final PROOF.
