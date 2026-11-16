# Proposal

## Why

ChuteGPT currently prints a development notice. The owner wants to explore prior Dia de Sorte results through independently selectable calculations and readable terminal visualizations.

## What Changes

- Read and validate a local official workbook snapshot and make freshness and source visible.
- Offer each specified and verifiable legacy contest-analysis strategy separately, with progress, cancellation and persistent results shown in charts or tables.
- Correct documented legacy calculation defects and capture differences and independent verification cases.
- Include the owner-confirmed definable legacy series statistics with explicit populations and missing-value behavior.
- Maintain PentaPRO and the developer documentation as the change is developed.
- Exclude candidate scoring, betting generation, simulations and combined strategy execution from this change, per the owner's confirmed scope.

## Capabilities

### New Capabilities

- `interactive-strategy-analysis`: local draw data, individually selected historical statistics and terminal result interaction.

### Modified Capabilities

None. The existing daily development updater remains a separate active change.

## Impact

The Rust binary, terminal dependencies, XLSX source adapter, calculation modules, tests and project documentation will change. The current target is Linux desktop. Windows and macOS are deferred to a possible future stage. The relevant PentaPRO records are the interactive-strategies files in all five pillars. The earlier overlapping draft is preserved at `openspec/history/interactive-strategy-analysis/`; `estrategy-create` is the sole active change for this capability.

The complete selected scope passed an author readiness review documented in [PentaPRO PROCESS](../../../pentapro/process/interactive-strategies.md#full-scope-author-readiness-review--2026-11-16). Numerical behavior, data-path interoperability and terminal/platform support remain unverified until the implementation and integration checks pass. Artifact completeness does not establish verified software.
