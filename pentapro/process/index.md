# Specification status and resumption

Last updated: 2026-11-17. Scope: Rust development workflow and interactive historical analysis. The owner authorized the current feature implementation; Linux integration evidence is recorded in PROOF, and release authority is tracked separately. This is not acceptance of a complete product design.

- [Product](../product/overview.md), [domain](../product/domain.md), [features](../product/features.md), [quality](../product/quality.md) and [terminal experience](../product/experience.md).
- [Architecture](../project/architecture.md), [decisions](../project/decisions.md), [delivery](../project/delivery.md), [security](../project/security.md).
- [Environments](../production/environments.md), [operations](../production/operations.md), [evolution](../production/evolution.md).
- [Traceability](../proof/traceability.md), [verification](../proof/verification.md), [validation](../proof/validation.md).
- [Change control](change-control.md), [execution](execution.md), [stage 1 completion and readiness](stage-1-completion.md).

Coverage: bootstrap and selected interactive-analysis behavior Included; numerical checks and Linux terminal evidence are linked below. Other target-platform, user-value, release and distribution evidence is Pending; hosted services are not applicable. The active changes own no remaining implementation tasks; archive and release decisions remain separate.

Active changes: [interactive analysis](../../openspec/changes/estrategy-create/proposal.md) and [daily results update](../../openspec/changes/atualizar-resultados-diarios/proposal.md) are both implemented and validated on the recorded Linux snapshot; neither is archived. Review their validation and reconcile according to the OpenSpec workflow.

Legacy strategy research: [inventory](../../docs/analise-estrategias-legado.md). Its selected historical strategies are represented in the active change; additional product capabilities remain future scope.

Active full-scope interactive analysis: [readiness and open decisions](interactive-strategies.md), [product catalog](../product/interactive-strategies.md), [design](../project/interactive-strategies.md), [operations](../production/interactive-strategies.md), [verification plan](../proof/interactive-strategies.md), and [active OpenSpec change](../../openspec/changes/estrategy-create/proposal.md). Implementation and Linux integration checks are recorded in PROOF; release evidence remains separate.

2026-11-16: final cross-artifact review reconciled the PRODUCTION invocation note, added requirement-to-code-to-check traceability and updated the PROCESS index status. Both OpenSpec changes are implemented and validated on the recorded Linux snapshot; neither is archived, and release remains a separate stage.

2026-11-16: the [stage 1 completion and readiness assessment](stage-1-completion.md) records coverage, the increment readiness verdict (ready for continued local use on Linux at the recorded evidence level) and the evidence that remains absent. Distribution, other platforms, accessibility and independent sign-off stay Not assessed or Pending.

2026-11-16: [interactive strategy full-scope author review](interactive-strategies.md#full-scope-author-readiness-review--2026-11-16) assessed the selected change ready for authorized implementation. Previous bootstrap-level and early-draft status statements above are historical; implementation and Linux integration were completed afterwards, while target-platform and release verification remain pending.

2026-11-16: the [final integrated author review](interactive-strategies.md) records the implemented interactive scope and Linux checks. OpenSpec owns completion status; release and independent review evidence remain separate. Historical bootstrap and early-draft entries describe their original snapshots.


Historical: review remediation was in progress; the final correction status is recorded below and in [interactive strategy status](interactive-strategies.md).

Current owner decision: Linux desktop is the sole target for this stage. Windows and macOS are deferred to a possible future stage. The implementation and Linux integration tasks are complete; release remains a separate stage.

Historical: the owner added menu explanation and provenance tasks to the still-open `estrategy-create` change; that revision was later implemented and corrected as recorded below.

The menu revision is implemented and verified within the limits in PROOF; `estrategy-create` remains active and unarchived. Release remains separate.

Review findings were corrected and regression evidence recorded in interactive PROOF. The change remains open.

Historical: the single-line main-menu header correction was formerly recorded as awaiting implementation. Previous completion records describe earlier scopes.

The single-line main-menu header revision is implemented and checked as recorded in PROOF; the change remains unarchived.

2026-11-16 final review correction: the reported lint gate now passes; a startup preflight exposes bound freshness before the first strategy; S21 wording and leading-undefined series handling were reconciled; negative hundredths formatting was corrected. Linux fmt, Clippy, 33 Rust tests, 13 Python tests, release build, OpenSpec strict validation and a pseudo-terminal preflight check passed. Current hashes and remaining limits are in [PROOF](../proof/interactive-strategies.md#final-review-correction-closure--2026-11-16). Release and archive remain separate.

2026-11-16 peer-review follow-up: a source rejected by the background preflight is reported as `falha na verificação` with the reason in the explanation; S17/S20 series declare the excluded leading positions; scale-100 formatting is shared through `presentation::format_value`; the S17/S20 naming and the design implementation status were corrected; and the explanation-scroll check uses the minimum supported size (55×19). Linux fmt, Clippy, 36 Rust tests, 13 Python tests, release build, OpenSpec strict validation, `git diff --check` and the pseudo-terminal checks passed. Evidence, limits and current hashes are in [PROOF](../proof/interactive-strategies.md#peer-review-follow-up-closure--2026-11-16). Release and archive remain separate.

2026-11-17: the daily updater downloaded and validated the official workbook through contest 1349 and the Linux checks plus a pseudo-terminal parity/S20 journey were rerun; the PROCESS "binary only prints a notice" reconstruction was labeled as a proposal-snapshot record and the README strategy list now names all 22 strategies. Evidence and current limits are in [PROOF](../proof/interactive-strategies.md#workbook-refresh-and-revalidation--2026-11-17-contest-1349); release, distribution, archive and other platforms remain separate.

2026-11-17 cross-artifact review: all PentaPRO pillar files were compared against the actual Rust source; hashes confirmed unchanged from the peer-review closure snapshot. One test flakiness was identified: `ui::tests::preflight_updates_menu_status_without_starting_analysis` fails intermittently when the full 36-test suite runs in parallel (`cargo test --offline`) due to file-access contention on the shared official workbook; it passes reliably with `--test-threads=1` (36/36) or in isolation. This is a test-harness constraint documented in [decisions](../project/decisions.md) (DEC-6) and [PROOF](../proof/interactive-strategies.md). Architecture, features and decisions records were updated to reflect the full module map, all 22 strategy names and accurate dependency decisions. Source code and workbook snapshot are unchanged. Release, distribution, archive and independent sign-off remain separate.
