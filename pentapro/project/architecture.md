# Architecture

The current Rust binary in `src/main.rs` uses a local XLSX adapter, independent numerical modules and a Ratatui terminal interface; see [interactive analysis design](interactive-strategies.md) for boundaries and dependencies. The bootstrap previously had no dependencies. Release distribution and future capabilities remain pending.

## Source module map (implemented, Stage 1)

| Module | Role | Key contracts |
| --- | --- | --- |
| `src/main.rs` | CLI entry point: parses `--planilha`, resolves the source path and calls `ui::run`, which checks the terminal and starts preflight/event processing | Path resolution, interactive-TTY check, nonzero exit on error |
| `src/lib.rs` | Crate root: declares public modules for data, analysis, presentation and UI; APIs are accessed through those modules | Public API surface for library tests |
| `src/data.rs` | XLSX/ZIP adapter: bounded read, SHA-256, actual and declared decompression limits, per-cell text bound, required-field validation, draw invariants, freshness classification from the version-2 sidecar | IA-01, IA-02; workbook input contract in [PROJECT design](interactive-strategies.md) |
| `src/analysis.rs` | S01–S07 independent histograms: parity, repeat, recurrence (prior-only), adjacency, longest run, sum, digital root | Validated draws, cooperative progress/cancellation; first-draw exclusions |
| `src/advanced.rs` | S08–S12, S18 multi-metric computations: number dispersion, pooled/per-draw gaps, decade groups, last-digit columns, matrix maxima, distance | Validated draws, two-decimal rounding, explicit pooled-gap flag |
| `src/history.rs` | S13–S16 combinatorial and categorical: nonoverlapping 27/31 cycles with unfinished tails, exact 1-based ordinal ranking, individual number and lucky-month counts/delays | Lexicographic combinatorics, never-seen distinction, invalid-draw rejection |
| `src/historical.rs` | S17, S19–S22 prior-history metrics: frequency/absence ranking with tied tenth place, mean of roots, frequency-delay dispersion, weekday prize tiers, month and principal-prize series | Coverage gate for S20; checked integer monetary arithmetic; invalid-draw rejection |
| `src/numeric.rs` | Shared hundredths rounding helpers used by composed dispersion and historical interval measures | Single half-to-even and overflow rule |
| `src/series.rs` | Shared descriptive series statistics library: periodicity, closed-wait, all defined measures (mean, SD, skewness, kurtosis, circular mean, etc.), cancellable category iteration | Owner-confirmed formulas; invalid/zero/censored edge handling |
| `src/presentation.rs` | Result builders: typed `View`/`Section` results for all 22 strategies, strategy names and concise definitions (expanded explanations live in `src/ui.rs`), shared `format_value`, explicit unsupported-scale marker | IA-04, IA-06; Brazilian Portuguese labels; scale-100 sign preservation |
| `src/ui.rs` | Ratatui TUI: menu (30/70 split, scrollable explanation), background preflight, strategy dispatch, progress/cancellation, result/series navigation, resize handling, `TerminalSession` guard | IA-01–IA-07; minimum 55×19; preflight truthful freshness |

Development utility: `scripts/atualizar_resultados.py` uses the Caixa page download endpoint via Python standard library and stores a separate current workbook and local state under `docs/`. This is not an application feature; see the [OpenSpec change](../changes/archive/2026-11-19-atualizar-resultados-diarios/proposal.md). Tests are in `tests/test_atualizar_resultados.py`.

## Repository tooling outside the crate

Repository tooling does not change the application contract but is part of the working environment: `scripts/atualizar_resultados.py` refreshes the official workbook, and the OpenCode user-level plugin `~/.config/opencode/plugins/agents-log.ts` appends every prompt to the local `AGENTS.log` from the OpenCode `context` hook for all projects on this machine ([DEC-8](decisions.md), [policy](../../docs/agents-log-policy.md)). `AGENTS.md` carries the mandatory rule; `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursor/rules/agents-log.mdc` and `.windsurfrules` restate it for harnesses that do not load `AGENTS.md`. `~/.config/git/ignore` ignores `AGENTS.log` and `AGENTS.log.*` in every repository. `.opencode/`, `.cursor/` and `.windsurfrules` stay git-ignored, so those pointers are local artifacts; `AGENTS.md`, `CLAUDE.md`, `.github/copilot-instructions.md` and the policy document are tracked.

`src/test_fixture.rs` is compiled only under `#[cfg(test)]`; it writes deterministic synthetic XLSX workbooks so the integration gate runs without the ignored official copy. `ROADMAP.md` records the owner’s future generation/bolão idea; it grants no implementation authority.
