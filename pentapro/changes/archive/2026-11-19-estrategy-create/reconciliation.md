# Reconciliation of interactive analysis drafts

## Historical reconciliation — 2026-11-16

The owner requested reconciliation on 2026-11-16. The active change is `estrategy-create`; the earlier unimplemented `interactive-strategy-analysis` draft is preserved under `openspec/history/interactive-strategy-analysis/`. This relocation did not mark the old change or its tasks as implemented and did not synchronize a main capability spec.

The earlier proposal and design describe the same local workbook, independent strategies, Ratatui UI, progress, result retention and deferred generation/simulation scope. These are represented in the active proposal and design. The earlier capability scenarios for parity, invalid input, stale or unknown freshness, progress and result retention, and small terminals are retained in the active capability spec. The active spec adds cancellation, terminal restoration, prize-field handling, cycle thresholds and prior-only comparisons. The five earlier unfinished tasks are represented by active tasks 1.2–1.4, 2.1–2.7, 3.1–3.4, 4.1–4.3 and the release follow-up.

At that historical reconciliation snapshot, no implementation task was completed by the reconciliation itself; P-01–P-04, detailed strategy acceptance, and source path interoperability remained open. Those statements describe the 2026-11-16 state only.

## Current status — 2026-11-18

The active `estrategy-create` change now contains the selected capability, implementation/design history and 79/79 completed tasks, including the final cross-artifact review in section 15. P-01–P-05 and the S01–S22 and shared-series contracts are reconciled in PRODUCT; technical choices and workbook contracts are in PROJECT; local operation is in PRODUCTION; tested snapshot evidence and residual limits are in PROOF; authority, task status and resumption are in PROCESS. The root README and traceability map are aligned with the implemented behavior.

OpenSpec reports the change complete but it remains unarchived. No main capability spec is synchronized while the change remains active. Linux implementation and integration evidence is bounded to the snapshots and journeys in [PentaPRO PROOF](../../../../pentapro/proof/interactive-strategies.md#final-cross-artifact-review--2026-11-18); release, distribution, owner acceptance, other platforms, human accessibility and predictive efficacy remain separate or unverified.
