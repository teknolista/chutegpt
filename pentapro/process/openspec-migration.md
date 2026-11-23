# OpenSpec to PentaPRO migration coverage

Status: requested content migration, 2026-11-23. The user explicitly retired OpenSpec; no application implementation, new result acceptance, release or platform verification is implied. Existing 2026-11-19 archive authorization remains historical. Current source baseline was read from the working tree; original files and moved archived changes are retained through the Git history and historical draft copy below.

## Maintained capability inventory

| Retired requirement and scenarios | Canonical PentaPRO location |
| --- | --- |
| Daily official workbook refresh — First successful execution of the day, Repeated execution, Download or validation fails, Later draw on the same day, Invalid data in the download, Damaged local copy or metadata, Concurrent invocations | product/development-data.md#acceptance-scenarios |
| Load the selected historical snapshot — User specifies the development workbook, Invalid workbook, Workbook above resource limits | product/interactive-strategies.md#user-journey-and-requirements; project/interactive-strategies.md#workbook-input-contract |
| Disclose freshness before analysis — Old or unknown data | product/interactive-strategies.md#user-journey-and-requirements |
| Select a strategy — Analyze parity, Distinguish cycle thresholds, Historical comparison uses prior draws | product/interactive-strategies.md#strategy-catalog-and-expected-output |
| Present processing progress — Cancel a long calculation | product/interactive-strategies.md#user-journey-and-requirements |
| Retain readable results — Review and return, Resize the terminal | product/interactive-strategies.md#user-journey-and-requirements |
| Explain prize data gaps — Prize field missing | product/interactive-strategies.md#strategy-catalog-and-expected-output |
| Restore the terminal — Noninteractive execution | product/interactive-strategies.md#user-journey-and-requirements |
| Present descriptive series measures — Zero wait and censored tail, Undefined statistic, Leading positions without a defined statistic | product/series-statistics.md#series-and-populations; product/series-statistics.md#measures-and-edge-cases |
| Show workbook provenance across application headers — Matching same-day record, Open any strategy screen, Source status is unknown or stale, Source read and rejected | product/interactive-strategies.md#header-consistency-description-expansion-and-provenance-cleanup-2026-11-17-specification-update |
| Keep the selected workbook path accessible — Narrow terminal, Selected path exceeds the header budget | product/interactive-strategies.md#user-journey-and-requirements; product/interactive-strategies.md#header-consistency-description-expansion-and-provenance-cleanup-2026-11-17-specification-update |
| Explain the selected strategy alongside the menu — Detailed strategy calculation explanation without redundant source, Change the selected strategy, Explanation exceeds available space | product/interactive-strategies.md#header-consistency-description-expansion-and-provenance-cleanup-2026-11-17-specification-update |
| Simplify result provenance display — View an analysis | product/interactive-strategies.md#owner-requested-menu-revision-implemented |
| Retain readable result details under wrapped headers — Tall provenance header at minimum size | product/interactive-strategies.md#header-consistency-description-expansion-and-provenance-cleanup-2026-11-17-specification-update |

## Historical interactive-strategy-analysis draft

The original 2026-11-16 draft [proposal](../changes/archive/2026-11-16-interactive-strategy-analysis-draft/proposal.md), [design](../changes/archive/2026-11-16-interactive-strategy-analysis-draft/design.md), [tasks](../changes/archive/2026-11-16-interactive-strategy-analysis-draft/tasks.md), [scenarios](../changes/archive/2026-11-16-interactive-strategy-analysis-draft/specs/interactive-strategy-analysis/spec.md), and metadata are preserved as historical provenance. The later estrategy-create implementation superseded the draft; unchecked draft tasks are not current work. Its parity distribution, invalid-data and freshness paths, progress/result retention, and resize preservation map to IA-01–IA-05 in [PRODUCT](../product/interactive-strategies.md#user-journey-and-requirements). Draft Ratatui and immutable snapshot decisions map to [PROJECT](../project/interactive-strategies.md); requirements-first, verification and separate release remain in [PROCESS](execution.md). Candidate scoring, generation and simulation remain deferred. The draft intended three OS targets; the later owner decision defers Windows/macOS and current support/evidence is Linux only. No old proposal was promoted merely because it appeared in the draft.

## Five-pillar coverage and evidence limits

- PRODUCT: current behavior in [interactive analysis](../product/interactive-strategies.md), [series](../product/series-statistics.md), [numerical contracts](../product/numerical-contracts.md) and [development data](../product/development-data.md); 22 strategies remain in scope, betting generation/scoring/simulation deferred.
- PROJECT: [interactive design](../project/interactive-strategies.md) owns source, snapshot, data validation, constraints and selected technical decisions.
- PRODUCTION: [operations](../production/operations.md) and [interactive operation](../production/interactive-strategies.md) own refresh, recovery and distribution prerequisites.
- PROOF: [traceability](../proof/traceability.md), [verification](../proof/verification.md) and [interactive evidence](../proof/interactive-strategies.md) retain actual dated, revision-bound checks and gaps; this migration does not repeat or broaden them.
- PROCESS: [execution](execution.md), [change control](change-control.md), [status](index.md) and historical [draft archive](../changes/archive/2026-11-16-interactive-strategy-analysis-draft/MIGRATION.md) own workflow and provenance.
- No predictive advantage, production release, Windows/macOS support, human accessibility or independent sign-off is inferred from documentary consolidation.

## Source integrity

| Retired current file | SHA-256 of pre-removal bytes |
| --- | --- |
| openspec/specs/atualizacao-resultados/spec.md | e9fc560f5788c2695ace18597cd178ff40596961da48264e0a8c4f216b81e48f |
| openspec/specs/interactive-strategy-analysis/spec.md | 9be515c364d299387678881d40b64481cbffa9b5d8b6fb643743c36741ab0a8d |

Migration checks on the local working tree: source-to-scenario inventory covered all 33 archived-delta scenarios and all current maintained-capability scenario names; complete historical draft files and three original archive metadata files were checked byte-for-byte against their pre-removal sources; 408 internal Markdown links and anchors were checked with zero missing after deduplication; `git diff --check` passed. The local daily updater reported already current today at contest 1353; it did not download a workbook. No application runtime tests were executed for this documentation migration. Original OpenSpec archive YAML metadata was recovered from the tracked HEAD into each moved historical archive before removal. The redundant consolidated acceptance section was removed after review; unique resource-limit, rejected-preflight and same-day status clauses were folded into the existing interactive requirements and menu owner. This table is a source-to-owner index; preserved historical delta specs remain archival records, not independent current specifications.
