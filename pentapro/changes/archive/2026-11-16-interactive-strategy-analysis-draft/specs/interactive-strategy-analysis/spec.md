## ADDED Requirements

### Requirement: Choose an individual contest-analysis strategy
The application SHALL present a navigable menu of the selected historical analysis strategies using the validated local official workbook.

#### Scenario: Select parity
- **WHEN** the user chooses parity with a valid dataset
- **THEN** the application calculates counts of draws for each possible even count from zero through seven, shows the complementary odd count and displays the distribution with numeric labels

#### Scenario: Data is unavailable or invalid
- **WHEN** the local workbook is missing, unreadable or invalid
- **THEN** the application explains the issue and allows returning to the menu without calculating results

#### Scenario: Freshness is stale or unknown
- **WHEN** the data is valid and freshness is stale or cannot be established
- **THEN** the application warns the user and allows an explicit choice to continue or return

### Requirement: Show progress and retain results
The application SHALL show processing progress and keep the result visible until the user navigates.

#### Scenario: Processing and return
- **WHEN** a user starts a strategy and processing is underway
- **THEN** the application shows the current stage, completed and total units and percentage when measurable, or an indeterminate indicator otherwise
- **AND** the result remains available until the user chooses to return or quit

#### Scenario: Small terminal
- **WHEN** the terminal is too small to render the active view
- **THEN** the application requests resizing and retains the active view and results
