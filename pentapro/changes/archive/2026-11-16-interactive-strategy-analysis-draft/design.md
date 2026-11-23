## Context
The current Rust binary is a bootstrap. The owner requests a terminal UI with individual strategies and local official results. Legacy research is evidence, not an accepted implementation contract.

## Goals / Non-Goals
Specify all selected historical contest-analysis strategies before implementation; retain results and show progress. Candidate scoring, generation and simulation are deferred.

## Decisions
Ratatui is requested. Proposed boundaries separate validated immutable snapshot, strategy computations and UI. Consult the five-pillar draft for source details, acceptance and open decisions. Crate versions, terminal/backend compatibility and path/freshness contract remain pending.

## Risks / Trade-offs
Legacy formulas include ambiguity and leakage. Correct and document differences, and validate against independent expected examples. A same-day local download does not prove upstream freshness.
