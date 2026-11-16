# Interactive historical strategy analysis

Status: confirmed scope with an implemented development binary; implementation and Linux integration checks are complete within the evidence limits recorded in PROOF; release is separate. Original baseline: Rust bootstrap with no interactive application. Legacy behavior and defects are described in [legacy research](../../docs/analise-estrategias-legado.md). The owner requests all legacy contest-analysis strategies that can be specified and validated. Candidate scoring, betting generation and simulations are deferred to a later change.

## User journey and requirements

- IA-01: Launch in a terminal and show a selectable menu of individually runnable historical strategies. Read the latest available local workbook from the user data directory, with a user-supplied path override. During development, explicitly select `docs/Dia de Sorte-atual.xlsx`, maintained by the separate daily updater. Owner selected no automatic application download. A run freezes one validated snapshot for its entire duration.
- IA-02: If the copy is known stale, show its latest contest and date, warn and allow an explicit choice to continue or return. If freshness cannot be established, say unknown and offer the same choice. Missing, unreadable or invalid data produces an actionable error and a way back to the menu, without computing on invalid input. Freshness is unknown when no trustworthy matching update record is available; a valid matching record dated before the current local date is stale. A matching record means its source and SHA-256 correspond to the selected file. The same-day record means only that the copy was checked that day.
- IA-03: During a strategy execution, show the stage and measurable completion, including completed and total units and percentage, where calculable. When no honest denominator is known, show an indeterminate activity indicator. Keep the interface responsive and provide cancellation.
- IA-04: Show each strategy result as a labeled chart when suitable, with numeric values and readable category labels; use a table or textual summary when a chart is unsuitable. Include the data range, sample size, units, strategy definition and exclusions. Keep the result visible until the user requests navigation. Provide keyboard controls to return to the menu and quit, including discovery of controls.
- IA-05: On interactive terminal resize, keep controls and values accessible through resizing, scrolling or pagination. On terminals too small to display the active view, show a resize message and preserve state. Restore terminal settings on normal exit, cancellation and recoverable errors. Noninteractive invocation reports a clear error and exits nonzero.
- IA-06: Screen labels and user messages use Brazilian Portuguese. Support keyboard-only operation and avoid conveying information only by color.
- IA-07: Every result derives from the selected snapshot and its defined population. Explicitly mark descriptive results and model assumptions; no claim that historical patterns guarantee a future draw.

## Strategy catalog and expected output

This catalog identifies the full proposed contest-analysis scope. For every strategy the final contract must fix inputs, metric formulas, population, binning, missing values and example outputs before implementation. The source references are research, not acceptance evidence.

| ID | Strategy and proposed result | Source |
| --- | --- | --- |
| S01 | Parity: histogram of draws by even count 0..7, odd count = 7−even. | legacy report §2 |
| S02 | Repeat count: histogram of intersections with immediately previous draw, first draw excluded. | §2 |
| S03 | Historical recurrence: maximum overlap with other draws; require strictly earlier draws for historical evaluation and label first draw unavailable. | §2, §3.2 |
| S04 | Adjacent sequences: histogram of count of adjacent sorted pairs with difference 1. | §2 |
| S05 | Longest consecutive run: histogram of number of adjacent links in longest run, zero if no adjacent pair. | §2 |
| S06 | Sum: histogram of sum of seven numbers. | §2 |
| S07 | Digital-root numerology: histogram of repeated digit sums of the total. | §2 |
| S08 | Number dispersion: distribution/summary of range, mean, mean of roots, RMS, mean absolute deviation, population standard deviation and coefficient of variation. Use a scrollable screen with separately labeled charts and tables. Precision and binning are defined in the numerical contracts. | §2 |
| S09 | Gaps: distribution/summary of six adjacent sorted differences and associated dispersion; show both pooled gaps and per-draw summaries as defined in the numerical contracts. | §2 |
| S10 | Decade groups: occupancy of 01–10, 11–20, 21–30, 31 and histogram of maximum occupancy; Go legacy uses different decade boundaries. | §2, §3.2 |
| S11 | Last digit: counts per last digit 0..9 and distribution of maximum occupancy per draw. | §2 |
| S12 | Matrix: distribution of maximum per row, maximum per last-digit column, and their sum. | §2 |
| S13 | Full coverage cycles: completed non-overlapping cycles until all 31 numbers observed; summarize duration and show unfinished tail separately. Legacy Python's 27/31 cycle differs and needs explicit handling. | §2, §3.2 |
| S14 | Ordinal: lexicographic rank among C(31,7) combinations, distribution of rank bands and absolute differences between adjacent ranks. Bin boundaries are defined in the numerical contracts. | §2 |
| S15 | Individual numbers: total occurrences and elapsed draws since last occurrence for 01..31; never-seen distinct from zero. | §2 |
| S16 | Lucky month: occurrences and elapsed draws for 12 months, independent from numerical results. | §2, §3.3 |
| S17 | Historical frequency and absence: for each draw compare to prior-only historical per-number counts and delays; distribution of selected draw's counts/delays. Ties are owner-confirmed; aggregation is defined in the numerical contracts. | §2, §3.2 |
| S18 | Distance: histogram of maximum minus minimum. | §3.2 |
| S19 | Legacy mean of roots: histogram of rounded mean of sqrt(number), clearly distinct from conventional median. | §3.2 |
| S20 | Frequency-delay dispersion: legacy formula round(sqrt(population_std(freq) × population_std(delay))) on prior-only information; The never-seen rule is owner-confirmed; rounding is defined in the numerical contracts. | §3.2 |
| S21 | Weekly prize analysis: prizes by weekday; populations and missing-value rules are defined in the numerical contracts, including the per-known-rate-row monetary average. | §3.2 |
| S22 | Prize and month series: distributions and waiting intervals; distinguish closed gaps and censored current delay, including zero gaps. Prize inputs and missing-value rules are defined in the numerical contracts. | §3.2 |

Population of historical observations must not use later draws to evaluate an earlier one. S13 includes separate full 31-number and partial 27-number cycles, each using non-overlapping cycles and displaying the unfinished tail. S17 and S20–S22 have specified and implemented historical calculations; further acceptance and platform evidence remain separate. Prize and weekday analyses use historical winners and prize amounts where available and show missing-field warnings. Multi-metric results use scrollable charts and tables. Filters, candidate scoring, betting and simulation are deferred.

## Open decisions

- P-01: resolved by delegated numerical design in [numerical contracts](numerical-contracts.md): chart output, population, precision and grouping. Full-scope readiness review remains separate.
- P-02: resolved by owner and delegated numerical design in [numerical contracts](numerical-contracts.md): include ties at the tenth rank; S20 is unavailable until all 31 numbers have appeared in prior draws; define ranking, threshold and sample rules.
- P-03: resolved by delegated numerical design in [numerical contracts](numerical-contracts.md): ordinal bands and rounding; the owner confirmed separate 27-number and 31-number cycles.
- P-04: resolved by delegated numerical design in [numerical contracts](numerical-contracts.md): S21/S22 populations and missing-value handling; the owner confirmed use of historical winner and prize columns with warnings for missing fields.
- P-05: Resolved by owner: default to a user data directory, allow a path override, and warn when no matching update is recorded on the local day or the date is unknown. Exact platform path and updater interoperability are implementation design details.


## Additional numerical decisions proposed for review

All draw-number sets are distinct sorted integers from 1 to 31; counts use integer arithmetic, exact counts and denominators are preserved. Aggregated percentages are counts divided by the eligible sample size; a zero eligible population is shown as unavailable. For S01 and S02 chart every possible category including zeros. For S03 use maximum intersection over earlier draws only and exclude the first draw. For S04 sum indicators of consecutive pairs whose difference is one; S05 is the longest run's number of links. E.g. 03,04,05 contributes two to each. For S06 histogram the exact sum; S07 take the digital root of that sum, as a descriptive categorization.

S08 computes per-draw range, arithmetic mean, mean of square roots, RMS, mean absolute deviation from mean, population standard deviation and coefficient of variation. Use raw values for calculations and rounded display values, with rounding and chart grouping defined in the numerical contracts. S09 first computes six adjacent gaps per draw and shows both the pooled gap histogram and per-draw summary statistics; the precision contract is defined in the numerical contracts. For S10–S12 category and maximum counts are specified in the catalog. For S13 begin a cycle with the draw after the prior closure and display closed-cycle duration and incomplete final cycle separately, for both distinct thresholds.

S14 rank is 1-based lexicographic order among C(31,7) combinations. Use exact combinatorial ranking and distinguish duplicate historical draws, with percentage-band cut points defined in the numerical contracts. S15 and S16 distinguish never-seen values, zero draws since latest appearance, and current open intervals. S18 is range; S19 is mean of roots with separately documented legacy rounding. For S21/S22 use workbook fields when available and label missing data; exact denominators are defined in the numerical contracts. These decisions do not approve any unverified historical inference or prediction claim.

## Example for independently checked acceptance

For one draw 01,02,03,10,20,30,31: parity has 4 even and 3 odd, consecutive links = 3 and longest run = 2 links, sum = 97, digital root = 7, range = 30, and sorted gaps = [1,1,7,10,10,1]. Decade group occupancy 01–10,11–20,21–30,31 is [4,1,1,1]. In a dataset with only this draw, the parity histogram contains one draw at even count 4 and zero at all other counts.

## Reference vector for S01–S07

Use two draws in chronological order: A = [1, 2, 3, 10, 20, 30, 31] and B = [1, 4, 5, 10, 12, 20, 31]. All counts below are exact. For S01, even counts are [4, 4], so bin 4 has 2 and the other bins have 0. For S02, A has no predecessor and is excluded; B intersects A in {1, 10, 20, 31}, so bin 4 has 1. For S03, A is unavailable; B's maximum prior overlap is 4. For S04, A has three adjacent links (1–2, 2–3, 30–31) and B has one (4–5). For S05, the longest run has two links for A and one for B. For S06, sums are 97 and 83; for S07, digital roots are 7 and 2. These are descriptive historical counts, not predictive probabilities.

Legacy difference: S03 excludes subsequent draws from each comparison, unlike the documented Lothon retrospective recurrence; S04 counts links rather than runs. The consolidated difference table for the selected catalog is [Trace to legacy behavior](numerical-contracts.md#trace-to-legacy-behavior).

## Confirmed ranking and dispersion decisions (S17, S20)

Owner decisions, 2026-11-16: include every number tied with the value in tenth place, even when this yields more than ten ranked numbers; S20 is unavailable for a draw until all 31 numbers have appeared in its strictly prior history. No later or current draw may supply that prerequisite. Report the number of excluded draws.

For S17, show frequency and absence rankings separately. Frequency is the exact count of a number in prior draws. Absence is the count of prior draws strictly after its latest occurrence; absence is unavailable for a never-seen number, which is listed separately and excluded from the absence ranking. Count, for each eligible draw, how many of its seven numbers belong to each prior-only top ranking, and show histograms 0–7 and eligible denominators. For frequency, all 31 numbers participate, including frequency zero, after at least one prior draw; inclusion of ties can therefore include all 31. For absence, rank observed numbers only and include all when fewer than ten have been observed. The first draw has no prior history and is excluded from both histograms. Preserve the Chutan periodicity and threshold-based frequency/absence analyses as separately identified planned output; their contracts and implementation evidence are linked in the numerical contracts.

For S20, take the seven current draw numbers and, for each, obtain the number of appearances and the absence in the strictly prior draws. Calculate population standard deviation separately across the seven frequency values and across the seven absence values, then `sqrt(std_frequency * std_absence)` and round to the nearest integer, with exact halfway cases rounded to the even integer. Present a histogram of eligible per-draw results and unavailable count. This corrects the Lothon use of full-history values when analyzing past draws; zero standard deviation gives zero.

Reference vector: prior draws [1..7], [8..14], [15..21], [22..28], [25..31]; current draw [1..7]. All 31 have appeared. Four numbers have frequency 2 and 27 have frequency 1, so all 31 enter the frequency top after the tenth-place tie, and all 7 current numbers are included. Absences are 4 for 1..7, 3 for 8..14, 2 for 15..21, 1 for 22..24 and 0 for 25..31; the tenth-place threshold is 3, so 14 numbers enter the absence top, including all 7 current numbers. The current seven frequencies are all 1 and absences all 4, so S20 is 0. For any draw among those first five draws, S20 is unavailable if the strictly prior history does not cover all 31 numbers.

Detailed working formulas and reference vectors for the selected catalog are in [numerical contracts](numerical-contracts.md). They are owner-confirmed for this stage; numerical checks and their limits are recorded in [PROOF](../proof/interactive-strategies.md).

Owner decision, 2026-11-16: include all additional legacy series statistics that can be specified and validated in the initial implementation. Their shared definitions, sample sizes, unavailable cases and display contract are in [series statistics](series-statistics.md).

The binary now exposes the 22-strategy keyboard menu, `--planilha` override and local stale/unknown warning, with explicit continuation or return. This is implemented on the working tree; remaining presentation, platform and integration checks are tracked by OpenSpec and PROOF.

The current terminal result view renders labeled distributions and numeric, scrollable details, supports section/category navigation and shared-series statistics, retains the active result until navigation, and preserves it across a too-small-terminal prompt. The UI identifies snapshot range, sample/unknown counts and each strategy definition. Platform and full acceptance evidence remain in PROOF.


Review remediation: terminals below 55 columns or 19 rows request resizing. Historical series exclude the structurally ineligible first draw for repeat, recurrence and ordinal differences; leading undefined-statistic positions are excluded from their series population, while actual unknown prize values retain unknown-state semantics. The series detail declares how many leading positions were excluded, so its population can be reconciled with the result screen's valid/unavailable notice.

## Owner-requested menu revision (implemented)

The selected strategy has a PT-BR explanation of two or three paragraphs describing its calculation and result sections in a right-hand pane (70%); the selectable list occupies 30%, with a vertical separator. Selection changes the explanation; long text remains accessible. The menu header shows the actual selected workbook path and truthful freshness, replacing the existing disclaimer. Individual result screens omit the standalone source and SHA-256 lines and retain a blank separator by the valid-observations line. Freshness warnings, snapshot integrity and numerical disclosures remain required. Acceptance scenarios and tasks are in `estrategy-create`.

The menu revision is implemented in the development binary. A background preflight read establishes the bound freshness before the first strategy, so the initial menu can show the actual status; while it is pending, freshness is explicitly unverified. Existing warnings still control continuation. A source that the preflight read and rejected is reported as `falha na verificação` (not as unverified), with the validation reason in the explanation; Enter retries the load.

The completed correction renders the header on one line when it fits, ordered `ChuteGPT — Análise Histórica Descritiva | Fonte: <selected path> | <freshness status>`. The example date and path are illustrative. The status remains truthful, and the full path remains accessible on narrow terminals. Evidence is in [PROOF](../proof/interactive-strategies.md#final-review-correction-closure--2026-11-16).
