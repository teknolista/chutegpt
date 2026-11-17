# Architecture

The current Rust binary in `src/main.rs` uses a local XLSX adapter, independent numerical modules and a Ratatui terminal interface; see [interactive analysis design](interactive-strategies.md) for boundaries and dependencies. The bootstrap previously had no dependencies. Release distribution and future capabilities remain pending.

## Source module map (implemented, Stage 1)

| Module | Role | Key contracts |
| --- | --- | --- |
| `src/main.rs` | CLI entry point: parses `--planilha`, asserts an interactive terminal, launches the Ratatui event loop and the background preflight read | Path resolution, interactive-TTY check, nonzero exit on error |
| `src/lib.rs` | Crate root: re-exports the public API (`load`, `Snapshot`, `Freshness`, `StrategyResult`, `run`) | Public API surface for library tests |
| `src/data.rs` | XLSX/ZIP adapter: bounded read, SHA-256, required-field validation, freshness classification from the version-2 sidecar | IA-01, IA-02; workbook input contract in [PROJECT design](interactive-strategies.md) |
| `src/analysis.rs` | S01–S07 independent histograms: parity, repeat, recurrence (prior-only), adjacency, longest run, sum, digital root | Cooperative progress/cancellation; first-draw exclusions |
| `src/advanced.rs` | S08–S12, S18 multi-metric computations: number dispersion, pooled/per-draw gaps, decade groups, last-digit columns, matrix maxima, distance | Two-decimal rounding; pooled gap aggregation |
| `src/history.rs` | S13–S16 combinatorial and categorical: nonoverlapping 27/31 cycles with unfinished tails, exact 1-based ordinal ranking, individual number and lucky-month counts/delays | Lexicographic combinatorics, never-seen distinction |
| `src/historical.rs` | S17, S19–S22 prior-history metrics: frequency/absence ranking with tied tenth place, mean of roots, frequency-delay dispersion, weekday prize tiers, month and principal-prize series | Coverage gate for S20; checked integer monetary arithmetic |
| `src/series.rs` | Shared descriptive series statistics library: periodicity, closed-wait, all defined measures (mean, SD, skewness, kurtosis, circular mean, etc.), cancellable category iteration | Owner-confirmed formulas; invalid/zero/censored edge handling |
| `src/presentation.rs` | Result builders: typed `View`/`Section`/`SeriesDetail` for all 22 strategies, strategy names, two-paragraph Portuguese explanations, shared `format_value` | IA-04, IA-06; Brazilian Portuguese labels; scale-100 sign preservation |
| `src/ui.rs` | Ratatui TUI: menu (30/70 split, scrollable explanation), background preflight, strategy dispatch, progress/cancellation, result/series navigation, resize handling, `TerminalSession` guard | IA-01–IA-07; minimum 55×19; preflight truthful freshness |

Development utility: `scripts/atualizar_resultados.py` uses the Caixa page download endpoint via Python standard library and stores a separate current workbook and local state under `docs/`. This is not an application feature; see the [OpenSpec change](../../openspec/changes/atualizar-resultados-diarios/proposal.md). Tests are in `tests/test_atualizar_resultados.py`.
