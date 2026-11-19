# Proposal

## Why

At this change's proposal baseline, the terminal executable printed only a development notice. The owner requested independently selectable calculations and readable terminal visualizations for prior Dia de Sorte results.

## What Changes

- Read and validate a local official workbook snapshot and make freshness and source visible.
- Present a uniform header with application title, source workbook and verified freshness across all screens (menu, progress, results, series details).
- Keep the selected workbook path accessible at supported terminal sizes: wrap it in the header when it fits the reserved height, and provide a dedicated scrollable provenance view when it does not.
- Offer each specified and verifiable legacy contest-analysis strategy separately, with progress, cancellation, persistent results shown in charts or tables, and exactly three Portuguese description paragraphs per strategy without redundant source lines in the explanation pane.
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

The selected scope passed an author readiness review documented in [PentaPRO PROCESS](../../../../pentapro/process/interactive-strategies.md#full-scope-author-readiness-review--2026-11-16). Implementation and Linux integration checks are complete for the reviewed working tree: all 79 tracked tasks are complete, with revision-bound evidence and limits in [PentaPRO PROOF](../../../../pentapro/proof/interactive-strategies.md#final-cross-artifact-review--2026-11-18). Other platforms, human accessibility, predictive efficacy, installation, distribution and release remain unverified or separate. Archival was explicitly authorized and completed on 2026-11-19. Artifact completeness does not establish verified software beyond the recorded evidence.
