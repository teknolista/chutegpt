# Interactive strategy analysis

## Purpose

Enable a terminal user to inspect historical Dia de Sorte draws through independently selected numerical strategies and clearly presented distributions.

## ADDED Requirements

### Requirement: Load the selected historical snapshot

The application SHALL load a valid local workbook from the user's data directory or a user-specified path, and use a single snapshot throughout each analysis.

#### Scenario: User specifies the development workbook
- **WHEN** the user selects the workbook maintained by the existing development updater
- **THEN** the application loads that path and identifies the latest contest and snapshot used

#### Scenario: Invalid workbook
- **WHEN** the workbook is missing, unreadable, corrupt or contains invalid required draw data
- **THEN** the application explains the failure and allows the user to return without producing an analysis

### Requirement: Disclose freshness before analysis

The application SHALL warn the user when the selected valid workbook lacks a matching successful update on the local day, or when freshness is unknown.

#### Scenario: Old or unknown data
- **WHEN** the local data is valid but its matching update record is older than the local day or unavailable
- **THEN** the user can explicitly continue with that copy or return without analysis

### Requirement: Select a strategy

The application SHALL present the individually executable contest-analysis strategies selected in the PentaPRO catalog and identify each result's population and definition.

#### Scenario: Analyze parity
- **WHEN** the user selects parity for a valid set of contests
- **THEN** the result displays the number of contests for each even count from 0 to 7 and its complementary odd count

#### Scenario: Distinguish cycle thresholds
- **WHEN** the user selects cycle analysis
- **THEN** the result separately identifies cycles covering 27 numbers and cycles covering all 31 numbers, including unfinished tails

#### Scenario: Historical comparison uses prior draws
- **WHEN** a strategy evaluates a contest against its history
- **THEN** only earlier contests contribute to that contest's historical comparison

### Requirement: Present processing progress

The application SHALL display the current processing stage and completed, total and percentage values when measurable, or an indeterminate indicator when a denominator is unavailable.

#### Scenario: Cancel a long calculation
- **WHEN** the user cancels an active strategy
- **THEN** the application stops that calculation and returns to a usable navigation state

### Requirement: Retain readable results

The application SHALL present results in labeled charts with numeric values or in tables when appropriate and retain them until the user navigates away.

#### Scenario: Review and return
- **WHEN** an analysis finishes
- **THEN** its results remain visible for inspection and the user can return to the strategy menu

#### Scenario: Resize the terminal
- **WHEN** the terminal is too small for the active view
- **THEN** the application requests resizing and preserves the active view and results

### Requirement: Explain prize data gaps

The application SHALL use historical winner and prize fields where available and identify missing values in prize and weekday analyses.

#### Scenario: Prize field missing
- **WHEN** an applicable prize field is unavailable
- **THEN** the result identifies the omission and excludes unavailable values from the relevant calculation

### Requirement: Restore the terminal

The application SHALL restore terminal state on normal exit, cancellation and recoverable failure.

#### Scenario: Noninteractive execution
- **WHEN** the application is started without an interactive terminal
- **THEN** it reports a clear error and exits unsuccessfully

### Requirement: Present descriptive series measures

The application SHALL provide the definable and verifiable legacy series statistics for applicable strategy categories, with the sample and missing-value rules in the PentaPRO series contract.

#### Scenario: Zero wait and censored tail
- **WHEN** a category occurs in consecutive draws and the final interval is still open
- **THEN** the closed wait of zero is included in the completed-wait sample and the open interval is labeled separately as censored

#### Scenario: Undefined statistic
- **WHEN** a series has no observations or does not meet a measure's mathematical preconditions
- **THEN** the measure is identified as unavailable with its sample size rather than shown as zero

#### Scenario: Leading positions without a defined statistic
- **WHEN** a category series starts with positions whose statistic cannot be defined because no prior history or coverage exists
- **THEN** those positions are excluded from the series population, the series detail states how many were excluded, and that population stays reconcilable with the result screen's valid/unavailable notice

### Requirement: Show workbook provenance in the menu header

The main menu header SHALL display one horizontal line in this order: `ChuteGPT — Análise Histórica Descritiva | Fonte: <selected path> | <freshness status>`. Use the actual selected path and truthful Portuguese freshness status. Preserve existing freshness warnings and continuation choices, and do not trigger a download.

#### Scenario: Matching same-day record
- **WHEN** the selected workbook has a matching successful update record for the local day
- **THEN** the menu header shows `ChuteGPT — Análise Histórica Descritiva | Fonte: <actual path> | Verificada hoje (DD/MM/YYYY)` on one line when it fits

#### Scenario: Narrow terminal
- **WHEN** the complete header does not fit the available width
- **THEN** the header retains the specified order and the full selected path remains accessible through the existing scrollable explanation panel

#### Scenario: Source status is unknown or stale
- **WHEN** validation or matching metadata cannot establish a current check
- **THEN** the header reports the actual known status without presenting it as verified today

#### Scenario: Source read and rejected
- **WHEN** the background check reads the selected source but validation rejects it
- **THEN** the header reports `falha na verificação` instead of an unverified source, the menu explanation states the validation reason, and selecting a strategy reports the failure without producing an analysis

### Requirement: Explain the selected strategy alongside the menu

The main menu SHALL dedicate approximately 30% of its horizontal content space to the strategy list and 70% to a vertically separated explanation pane. Each of the 22 strategies SHALL have two or three Portuguese paragraphs accurately describing its calculation and distinct result sections. Existing navigation and resize recovery remain usable.

#### Scenario: Change the selected strategy
- **WHEN** the user moves the selection to another strategy
- **THEN** the adjacent explanation changes to describe that strategy and its result sections

#### Scenario: Explanation exceeds available space
- **WHEN** the description does not fit the pane
- **THEN** the user can access the complete text without losing menu selection

### Requirement: Simplify result provenance display

The strategy result screen SHALL omit the standalone `Fonte:` and `SHA-256:` lines and retain a blank line separating the valid-observations line from subsequent content. The source and digest SHALL continue to be used for snapshot identity and freshness validation; this display change SHALL NOT weaken data validation or remove required sample, range, exclusion or warning information.

#### Scenario: View an analysis
- **WHEN** a strategy result is displayed
- **THEN** the standalone source and digest lines are absent and the valid-observations line is separated by one blank line
