# Traceability

This map links the current scope to the implementation, checks and evidence recorded in this repository. It is a pointer map, not an independent claim of correctness: every link is bounded by the limits recorded in the linked evidence.

## Bootstrap

- BOOT-1 in [features](../product/features.md): `cargo run` and `cargo test`; portability is separately unverified on Windows and macOS. The bootstrap behavior was superseded by the interactive-analysis change.

## Interactive historical analysis

| Requirement | Implementation | Checks |
| --- | --- | --- |
| IA-01 Path, override and frozen snapshot ([PRODUCT](../product/interactive-strategies.md)) | `src/data.rs` (resolution, bounded XLSX/ZIP read, SHA-256, validation, freshness), `src/ui.rs` (preflight and load journeys) | `data::tests::*`; `src/ui.rs` tests `preflight_updates_menu_status_without_starting_analysis`, `preflight_failure_is_reported_truthfully` |
| IA-02 Stale/unknown freshness and explicit choice | `src/ui.rs` (`Confirm` screen, `freshness_status`, `menu_explanation`) | `src/ui.rs` tests `result_omits_source_and_digest_and_menu_uses_snapshot_status`, `failed_reload_clears_previously_verified_snapshot` |
| IA-03 Progress and cancellation | `src/ui.rs` (worker threads, bounded message channel) | `src/ui.rs` tests `indeterminate_spinner_changes_and_cancel_returns_to_menu`, `progress_updates_are_bounded_per_frame` |
| IA-04 Labeled results, retained output, keyboard controls | `src/presentation.rs` (sections, values, bins, definitions), `src/ui.rs` (bar charts, result scroll) | `src/ui.rs` tests `menu_keyboard_wraps`, `synthetic_snapshot_renders_each_section_and_available_series`, `results_survive_resize_and_series_navigation`; `src/presentation.rs` tests `result_views_include_zero_bins_and_wait_histograms` |
| IA-05 Resize and too-small terminals, terminal restoration, complete provenance access | `src/ui.rs` (`TerminalSession`, minimum size 55×19, `draw`, `header_height`, `display_header`, `body_area_height`, `provenance_viewport`, `App::key`) | `src/ui.rs` tests `menu_and_small_terminal_render`, `results_survive_resize_and_series_navigation`, `provenance_header_keeps_the_full_path_visible_on_narrow_terminals`, `oversized_provenance_view_scrolls_and_preserves_processing_state`, `explanation_scrolling_reaches_end_with_multi_line_header`, `scroll_viewports_adjust_to_dynamic_header_height`, `result_details_remain_readable_under_tall_headers`; pseudo-terminal evidence in [PROOF](interactive-strategies.md) |
| IA-06 Brazilian Portuguese labels and keyboard-only operation | All user-facing strings in `src/presentation.rs` and `src/ui.rs`; strategy names and explanations in `presentation::STRATEGIES`, `presentation::DEFINITIONS` | `src/ui.rs` tests `menu_explanations_and_scrolling_follow_selection`, `explanation_omits_redundant_source_path_and_header_includes_it` |
| IA-07 Snapshot-bound results, explicit populations, no prediction claims | `src/data.rs` (hash binding), `src/analysis.rs`, `src/advanced.rs`, `src/history.rs`, `src/historical.rs`, `src/series.rs` (eligible populations, unknown positions, censored tails) | See strategy mapping below; evidence in [PROOF](interactive-strategies.md) |

## Strategies to implementation and checks

| IDs | Contract | Implementation | Checks |
| --- | --- | --- | --- |
| S01–S07 | [Numerical contracts](../product/numerical-contracts.md) | `src/analysis.rs` | `analysis::tests::independent_reference_vector_and_boundaries`; `analysis::tests::recurrence_uses_only_prior_draws_and_progress_can_cancel` |
| S08–S12, S18 | Numerical contracts | `src/advanced.rs` | `advanced::tests::independently_derived_composite_measures`; `advanced::tests::grouping_boundaries_and_progress` |
| S13 | Numerical contracts | `src/history.rs` | `history::tests::separate_cycles_and_incomplete_tail` |
| S14–S16 | Numerical contracts | `src/history.rs` | `history::tests::exact_rank_endpoints_duplicates_and_changes` |
| S17, S20 | [Confirmed ranking decisions](../product/interactive-strategies.md#confirmed-ranking-and-dispersion-decisions-s17-s20) | `src/historical.rs` | `historical::tests::prior_only_rank_ties_periodicity_and_coverage` |
| S19 | [Numerical contracts](../product/numerical-contracts.md) | `src/historical.rs` | `historical::tests::prior_only_rank_ties_periodicity_and_coverage` |
| S21–S22 | Numerical contracts | `src/historical.rs` | `historical::tests::missing_prizes_weekdays_and_censored_month_series` |
| Shared series measures | [Series statistics](../product/series-statistics.md) | `src/series.rs` | `series::tests::*` recorded in [PROOF](interactive-strategies.md) |

## Development workbook refresh

- `DEV-DATA-1` in [features](../product/features.md): [updater change](../../openspec/changes/atualizar-resultados-diarios/proposal.md) implemented in `scripts/atualizar_resultados.py`; checks in `tests/test_atualizar_resultados.py`; evidence in [verification](verification.md) and [PROOF](interactive-strategies.md).

## Remaining untraced gaps

Windows/macOS builds and runtime behavior, human accessibility acceptance, release distribution and out-of-sample predictive performance have no implementation or evidence to trace to; they remain unverified. Exhaustive malformed-workbook equivalence between the Python updater and the Rust reader is also unverified. Future statistical requirements must trace to independently derived examples, boundary checks and source/version assumptions.

Final review boundary checks: `presentation::tests::every_strategy_rejects_invalid_public_draws` covers all S01–S22 routes with out-of-range, unsorted, duplicate numbers and invalid months; `data::tests::iso_text_and_cell_size_boundary_are_checked_in_real_workbooks` covers ISO text and the inclusive 64 KiB limit. See [final review evidence](interactive-strategies.md#final-cross-artifact-review--2026-11-18).
