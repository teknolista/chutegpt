# Development workbook refresh — DEV-DATA-1

The development workflow attempts the official Caixa Dia de Sorte workbook refresh at the start of each local working day while preserving the original user-provided workbook and the last known valid current copy. This is a developer workflow, not an end-user download or a predictive feature.

## Acceptance scenarios

- **First successful execution of the day:** download and validate the official workbook before replacing the separate current copy and recording successful download metadata.
- **Repeated execution:** when the current copy matches the recorded hash and today already succeeded, skip download unless forced.
- **Download, validation or regression failure:** report failure; do not replace the last valid current workbook with missing, corrupt, incomplete, invalid or older data.
- **Later draw on the same day:** `--force` attempts a fresh download regardless of earlier success.
- **Invalid draw data:** reject incomplete responses, corruption, invalid dates, months or numbers, and missing contest IDs; leave the local workbook and success record unchanged for this rejection.
- **Damaged current copy or metadata:** retry the official source, validate it and guard against regression using readable local history or the last recorded successful contest.
- **Concurrent invocations:** when another updater holds the local update lock, report that an update is in progress without replacing either file.

A failure writing metadata after workbook replacement is separately reported; the next run can retry rather than claiming a completed success. Operational details and prerequisites belong to [PRODUCTION](../production/operations.md), source/data contract to [PROJECT](../project/interactive-strategies.md), and recorded checks and limitations to [PROOF](../proof/verification.md). Python/Rust validator equivalence for all XLSX inputs is unverified.
