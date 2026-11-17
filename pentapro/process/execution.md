# Execution

The owner authorized the interactive historical-analysis implementation after full-scope specification; integration is tracked separately and release authority remains separate. Check the applicable skill and project guidance. Track unresolved decisions, validation by platform and actual evidence. Do not equate artifact validity with behavior or release approval.

## Development cycle for this project

| Stage | Responsible | Input → output | Gate and evidence | Escalation |
| --- | --- | --- | --- | --- |
| Daily data refresh | Agent, at the start of each local working day | `python3 scripts/atualizar_resultados.py` → validated `docs/Dia de Sorte-atual.xlsx` and sidecar | Contest number reported by the updater; failure preserves the previous copy and is reported | Report the failure and the possibly stale copy; retry with `--force` after a draw |
| Specification of a behavior change | Owner decision with agent drafting | Owner request → OpenSpec proposal/spec/tasks and the affected PentaPRO records | Specification review before implementation; readiness recorded in [PROCESS](interactive-strategies.md) | Escalate a consequential product decision instead of assuming it |
| Implementation | Agent, within the authorized scope | Approved change → source, tests, documentation | [Traceability](../proof/traceability.md) maps each requirement to code and checks | Stop at the boundary of the authorization |
| Verification | Agent | Working tree → executed checks and recorded evidence | `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `python3 -m unittest discover -s tests`, `cargo build --release`, `openspec validate <change> --strict`, `git diff --check`, plus pseudo-terminal journeys on Linux | A failed required check stops the increment; report it |
| Review | Requested reviewer, or the implementing session when stated | Reviewed snapshot → findings with evidence | Recorded in [PROOF](../proof/interactive-strategies.md); self-review is labeled as such | Independent sign-off is still pending for the delivered snapshot |
| Delivery | Owner | Reviewed snapshot → commit and push | [Git delivery] is authorized per request; the published revision is verified against the remote | No release, archive or deployment without explicit authorization |
| Release and distribution | Owner, separate stage | Approved artifact → packaging and distribution | Not started; platform and accessibility evidence would be required | Out of scope for Stage 1 |

## Working rules observed in this repository

- Specification artifacts (PentaPRO and OpenSpec) are written in English; `docs/`, the root README and the conversation use Brazilian Portuguese.
- Every addition or modification is reflected in the owning PentaPRO records, with evidence bound to the tested source hashes.
- The Windows and macOS scope is deferred by owner decision; do not infer support from a Linux run.
- The application performs descriptive historical analysis only; no strategy, check or record may imply predictive advantage.
