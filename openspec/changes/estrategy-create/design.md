# Design

## Context

The current Cargo binary only prints a notice. The development updater writes an XLSX file and update metadata under the repository's docs directory. Product requirements, strategy inventory and remaining numerical decisions are in the PentaPRO interactive-strategies records.

## Goals / Non-Goals

**Goals:** isolate verified draw data, numerical strategies and terminal presentation while supporting cancellation and observable progress.

**Non-Goals:** implement candidate scoring, betting generation, simulations or combined strategy execution.

## Decisions

- **Snapshot boundary:** read and validate one immutable snapshot with contest ID, date, seven sorted distinct numbers, month and available prize fields. Retain source, hash and update provenance. This avoids a changing workbook producing inconsistent results. Streaming the file separately for each strategy would require additional consistency controls.
- **Strategy boundary:** use independent computations returning typed results and explicit eligible populations. A strategy receives only the snapshot and settings, not terminal state. This supports independent numerical checks and later composition. Shared helpers are appropriate when formulas actually agree.
- **Terminal:** use Ratatui as requested. A terminal backend and compatible versions must be verified before implementation. Keep progress and cancellation responsive and use scoped cleanup for terminal restoration. Bar charts and scrollable tables support differing result sizes.
- **Data location:** choose a Linux user data location and provide a path override, per owner decision. The existing updater's repository path is supplied explicitly during development. A freshness record applies only if it matches the file hash and source; same-day verification does not prove the publisher has posted a subsequent draw.
- **Numerics:** preserve integer counts, define rounding and precision explicitly, and use strictly prior draws for historical comparisons. P-01 through P-04 and shared series are resolved in linked PRODUCT contracts; reproducibility and examples guide changes from legacy behavior.

## Risks / Trade-offs

- **Numerical correctness** → implement the completed strategy-by-strategy formulas, displays and reference examples, and verify each against independent checks.
- **Old or absent data** → expose freshness and let the user choose whether to proceed with a valid copy.
- **Terminal and dependency differences** → test real supported platforms; do not infer Windows or macOS behavior from Linux.
- **Unbounded computation** → include measurable progress, cancellation and representative efficiency checks.

## Migration Plan

Keep the development updater as a separate operation. Support explicit selection of its workbook; document the installed data location when decided. Integration must establish how a user places an official workbook and matching provenance there. No existing application data migration or release is implied.

## Open Questions

Precise platform support versions and distribution packaging can be settled for the release gate after implementation, provided actual tested platforms and limitations are reported.

## Data path and updater interoperability

The delegated technical selection and full sidecar validation contract are in [PentaPRO PROJECT](../../../pentapro/project/interactive-strategies.md#workbook-path-and-freshness-contract-ia-01-ia-02). `--planilha PATH` overrides the platform user-data default. The development invocation selects `docs/Dia de Sorte-atual.xlsx`; installed use copies the workbook and adjacent updater sidecar or explicitly accepts unknown freshness. Path resolution, the explicit override, sidecar validation and hash binding are implemented in `src/data.rs` and exercised on Linux; installing a workbook in an installed user-data location remains an open integration step.

## Current platform scope

The owner selected Linux desktop as the only target for this stage. Windows and macOS are deferred to a possible future stage. Historical platform notes describe the earlier scope.

## Menu and result presentation revision (implemented 2026-11-16)

The owner requested the source path and verified/unknown/stale status in the main-menu header, and a 30/70 list/explanation layout with a vertical divider. Source status must reflect actual matching metadata, including before the first analysis and after returning from results; the implementation performs a background preflight read so the menu can show the bound status before a strategy is selected, while retaining snapshot/hash binding internally and all existing stale/unknown choice flows. Long two- or three-paragraph PT-BR strategy descriptions are keyboard-scrollable without conflicting with selection. Selection and text access are preserved across resize; narrow terminals use the existing resize prompt. The two standalone provenance lines are removed from results and the requested blank separator is retained. Details of input handling remain an implementation choice, subject to the scenarios and checks. Implementation and Linux evidence are recorded in [PROOF](../../../pentapro/proof/interactive-strategies.md).

## Single-line main-menu header (implemented 2026-11-16)

The owner specified the order and labels `ChuteGPT — Análise Histórica Descritiva | Fonte: <selected path> | <freshness status>` on one line when it fits. The example date and path are illustrative, not constants. The implementation keeps truthful verified, stale and unknown states, performs a background preflight read for the initial menu status, and retains access to the full path in the scrollable explanation when the terminal is narrow. Implementation and Linux evidence are recorded in [PROOF](../../../pentapro/proof/interactive-strategies.md).

## Preflight failure state (implemented 2026-11-16)

A background check that reads and rejects the selected source is reported as `falha na verificação`, never as an unverified source; the menu explanation carries the validation reason and Enter retries the load. A successful preflight, load or reload restores the bound freshness status, and the failed-reload journey keeps the local snapshot invalidated. Implementation and Linux evidence are recorded in [PROOF](../../../pentapro/proof/interactive-strategies.md).
