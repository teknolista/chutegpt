## ADDED Requirements

### Requirement: Daily official workbook refresh
The development workflow SHALL attempt to refresh the official Dia de Sorte workbook at the start of each local working day and keep the original user-provided workbook.

#### Scenario: First successful execution of the day
- **WHEN** the updater is invoked and no successful update is recorded for the local day
- **THEN** it downloads and validates the official workbook before replacing the separate current copy and recording the download metadata

#### Scenario: Repeated execution
- **WHEN** the current copy matches the recorded hash and the updater has already succeeded on the local day
- **THEN** it skips the download unless forced

#### Scenario: Download or validation fails
- **WHEN** the download fails or the file is invalid or older than the current copy
- **THEN** the updater reports failure without replacing the current copy

#### Scenario: Later draw on the same day
- **WHEN** the updater is invoked with the force option
- **THEN** it attempts a fresh download even if it already succeeded that day

#### Scenario: Invalid data in the download
- **WHEN** the response is incomplete, the workbook is corrupted, or its draw history has invalid dates, months, numbers or missing contest IDs
- **THEN** the updater rejects it and leaves the local workbook and success record unchanged

#### Scenario: Damaged local copy or metadata
- **WHEN** the current workbook or its success record is damaged
- **THEN** the updater retries against Caixa, validates the response, and guards against regression using readable local history or the last recorded successful contest

#### Scenario: Concurrent invocations
- **WHEN** another updater already holds the local update lock
- **THEN** the second invocation reports that an update is in progress without replacing either file
