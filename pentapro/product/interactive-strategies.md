# Interactive historical strategy analysis

Status: confirmed, language-neutral product scope with an implemented development application; implementation and Linux integration checks are complete within the evidence limits recorded in PROOF; release is separate. Original product baseline: the terminal executable displayed only a development notice. Legacy behavior and defects are described in [legacy research](../../docs/analise-estrategias-legado.md). The owner requested all legacy contest-analysis strategies that can be specified and validated. Candidate scoring, betting generation and simulations are deferred to a later change.

## User journey and requirements

- IA-01: Launch in a terminal and show a selectable menu of individually runnable historical strategies. Read the latest available local workbook from the user data directory, with a user-supplied path override. During development, explicitly select `docs/Dia de Sorte-atual.xlsx`, maintained by the separate daily updater. Owner selected no automatic application download. A run freezes one validated snapshot for its entire duration. Reject a workbook that exceeds the compressed or actual/declarative uncompressed resource limits, or contains an oversized text cell, with a contextual error before producing analysis; the [workbook input contract](../project/interactive-strategies.md#workbook-input-contract) owns the limits.
- IA-02: If the copy is known stale, show its latest contest and date, warn and allow an explicit choice to continue or return. If freshness cannot be established, say unknown and offer the same choice. Missing, unreadable or invalid data produces an actionable error and a way back to the menu, without computing on invalid input. Freshness is unknown when no trustworthy matching update record is available; a valid matching record dated before the current local date is stale. A matching record means its source and SHA-256 correspond to the selected file. The same-day record means only that the copy was checked that day.
- IA-03: During a strategy execution, show the stage and measurable completion, including completed and total units and percentage, where calculable. When no honest denominator is known, show an indeterminate activity indicator. Keep the interface responsive and provide cancellation.
- IA-04: Show each strategy result as a labeled chart when suitable, with numeric values and readable category labels; use a table or textual summary when a chart is unsuitable. Include the data range, sample size, units, strategy definition and exclusions. Keep the result visible until the user requests navigation. Provide keyboard controls to return to the menu and quit, including discovery of controls.
- IA-05: On interactive terminal resize, keep controls and values accessible through resizing, scrolling or pagination. On terminals too small to display the active view, show a resize message and preserve state. Restore terminal settings on normal exit, cancellation and recoverable errors. Noninteractive invocation reports a clear error and exits nonzero. The selected workbook path remains accessible when its provenance header exceeds its reserved height: a visible keyboard affordance opens a scrollable path view, and closing it preserves the underlying screen and analysis state.
- IA-06: Screen labels and user messages use Brazilian Portuguese. Support keyboard-only operation and avoid conveying information only by color.
- Keyboard map implemented in the development binary: in the menu, ↑/↓ or J/K select, Page Up/Page Down or H/L scroll the explanation, Enter runs the strategy and Q quits; in a result, Tab/Shift+Tab (or ]/[) change the section, ←/→ change the category, S or Enter open the series detail, ↑/↓ scroll, Esc returns to the menu and Q quits; while loading or computing, Esc or C cancels; the stale/unknown prompt accepts S or Enter to continue and N or Esc to return.
- Provenance overflow interaction (owner-selected 2026-11-17; implemented and Linux-verified in the current working tree): P opens a dedicated view of the complete selected path from any supported screen; ↑/↓ and Page Up/Page Down scroll it, and Esc or P closes it without discarding the menu, result, warning or active processing state.
- IA-07: Every result derives from the selected snapshot and its defined population. Explicitly mark descriptive results and model assumptions; no claim that historical patterns guarantee a future draw.

## Strategy catalog and expected output

This catalog is a concise index of the selected contest-analysis scope. The canonical contracts fix inputs, formulas, populations, binning, missing values and reference examples in [numerical contracts](numerical-contracts.md) and [series statistics](series-statistics.md). Legacy sources are research, not acceptance evidence.

| ID | Strategy behavior summary | Legacy source |
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

Population of historical observations must not use later draws to evaluate an earlier one. S13 includes separate full 31-number and partial 27-number cycles, each using non-overlapping cycles and displaying the unfinished tail. S01–S22 and shared series calculations are specified and implemented. Linux evidence supports the tested behavior; exhaustive input, platform, accessibility, predictive and release evidence remains separate or unverified. Prize and weekday analyses use historical winners and prize amounts where available and show missing-field warnings. Multi-metric results use scrollable charts and tables. Filters, candidate scoring, betting and simulation are deferred.

## Resolved decisions

- P-01: resolved by delegated numerical design in [numerical contracts](numerical-contracts.md): chart output, population, precision and grouping.
- P-02: resolved by owner and delegated numerical design in [numerical contracts](numerical-contracts.md): include ties at the tenth rank; S20 is unavailable until all 31 numbers have appeared in prior draws; define ranking, threshold and sample rules.
- P-03: resolved by delegated numerical design in [numerical contracts](numerical-contracts.md): ordinal bands and rounding; the owner confirmed separate 27-number and 31-number cycles.
- P-04: resolved by delegated numerical design in [numerical contracts](numerical-contracts.md): S21/S22 populations and missing-value handling; the owner confirmed use of historical winner and prize columns with warnings for missing fields.
- P-05: Resolved by owner: default to a user data directory, allow a path override, and warn when no matching update is recorded on the local day or the date is unknown. Linux behavior is implemented; other platform paths remain unverified and outside this stage's support claim.


## Numerical-contract summary (confirmed)

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

For S17, show frequency and absence rankings separately. Frequency is the exact count of a number in prior draws. Absence is the count of prior draws strictly after its latest occurrence; absence is unavailable for a never-seen number, which is listed separately and excluded from the absence ranking. Count, for each eligible draw, how many of its seven numbers belong to each prior-only top ranking, and show histograms 0–7 and eligible denominators. For frequency, all 31 numbers participate, including frequency zero, after at least one prior draw; inclusion of ties can therefore include all 31. For absence, rank observed numbers only and include all when fewer than ten have been observed. The first draw has no prior history and is excluded from both histograms. Preserve the Chutan periodicity and threshold-based frequency/absence analyses as separately identified output; their contracts and implementation evidence are linked in the numerical contracts.

For S20, take the seven current draw numbers and, for each, obtain the number of appearances and the absence in the strictly prior draws. Calculate population standard deviation separately across the seven frequency values and across the seven absence values, then `sqrt(std_frequency * std_absence)` and round to the nearest integer, with exact halfway cases rounded to the even integer. Present a histogram of eligible per-draw results and unavailable count. This corrects the Lothon use of full-history values when analyzing past draws; zero standard deviation gives zero.

Reference vector: prior draws [1..7], [8..14], [15..21], [22..28], [25..31]; current draw [1..7]. All 31 have appeared. Four numbers have frequency 2 and 27 have frequency 1, so all 31 enter the frequency top after the tenth-place tie, and all 7 current numbers are included. Absences are 4 for 1..7, 3 for 8..14, 2 for 15..21, 1 for 22..24 and 0 for 25..31; the tenth-place threshold is 3, so 14 numbers enter the absence top, including all 7 current numbers. The current seven frequencies are all 1 and absences all 4, so S20 is 0. For any draw among those first five draws, S20 is unavailable if the strictly prior history does not cover all 31 numbers.

Detailed working formulas and reference vectors for the selected catalog are in [numerical contracts](numerical-contracts.md). They are owner-confirmed for this stage; numerical checks and their limits are recorded in [PROOF](../proof/interactive-strategies.md).

Owner decision, 2026-11-16: include all additional legacy series statistics that can be specified and validated in the initial implementation. Their shared definitions, sample sizes, unavailable cases and display contract are in [series statistics](series-statistics.md).

Implementation status (2026-11-17): the 22-strategy keyboard menu, `--planilha` override, local stale/unknown warning and explicit continue/return choice are implemented. The user-visible contract does not depend on Rust or a specific terminal library; Linux evidence and unverified limits are in PROOF.

The implemented terminal result view renders labeled distributions and numeric, scrollable details, supports section/category navigation and shared-series statistics, retains the active result until navigation, and preserves it across a too-small-terminal prompt. It identifies snapshot range, sample/unknown counts and each strategy definition. Platform and human acceptance limits remain in PROOF.

Terminal behavior is specified for a minimum of 55 columns by 19 rows; smaller terminals request resizing without discarding state. Historical series exclude structurally ineligible leading positions for repeat, recurrence and ordinal differences; actual unknown prize values retain unknown-state semantics. The series detail declares excluded positions so its population can be reconciled with the result screen's valid/unavailable notice.

## Owner-requested menu revision (implemented)

The selected strategy has a three-paragraph PT-BR explanation describing its calculation and result sections in a right-hand pane (70%); the selectable list occupies 30%, with a vertical separator. Selection changes the explanation; long text remains accessible. The menu header shows the selected workbook path and truthful freshness. Individual result screens omit the standalone source and SHA-256 lines and retain a blank separator by the valid-observations line. Freshness warnings, snapshot integrity and numerical disclosures remain required. Acceptance scenarios and completed tasks are in `estrategy-create`.

The menu revision is implemented in the development binary. A background preflight read establishes the bound freshness before the first strategy, so the initial menu can show the actual status; while it is pending, freshness is explicitly unverified. Existing warnings still control continuation. A source that the preflight read and rejected is reported as `falha na verificação` (not as unverified), with the validation reason in the explanation; selecting a strategy reports the failure without producing analysis, and Enter retries the load.

The completed correction renders the header on one line when it fits, ordered `ChuteGPT — Análise Histórica Descritiva | Fonte: <selected path> | <freshness status>`. For a matching same-day update, the status is `Verificada hoje (DD/MM/YYYY)`; stale or unknown evidence never claims verification today. The example date and path are illustrative. The status remains truthful, and the header grows by wrapping the same text within the space reserved for the body and key legend. A selected path that exceeds this height budget is handled by the owner-selected scrollable provenance view below.

## Header consistency, description expansion and provenance cleanup (2026-11-17 specification update)

The owner requested three presentation refinements:
1. Uniform application header: when opening any strategy screen (analysis results, category details, or processing progress), the top header must display the same information as the main menu header (`ChuteGPT — Análise Histórica Descritiva | Fonte: <selected path> | <freshness status>`), replacing the previous fallback `ChuteGPT — análise histórica descritiva`.
2. Removal of redundant source in the explanation pane: on the main menu, the right-hand explanation pane previously appended `Fonte: <selected path>` to each strategy text. This is removed because the workbook path and freshness are already shown in the top header.
3. Detailed strategy explanations: the strategy descriptions were previously very brief (one or two short sentences). All 22 now provide exactly three paragraphs: a concise definition followed by detailed calculation, input/population, eligibility and result-section explanations, including strictly-prior rules where applicable.

Specification and tasks are recorded in the [archived `estrategy-create` change](../changes/archive/2026-11-19-estrategy-create) (tasks 9.1–9.5). The owner authorized implementation on 2026-11-17; the shared provenance header, explanation-pane cleanup and three-paragraph descriptions are implemented and covered by Linux UI tests. Revision-bound evidence and remaining limits are recorded in [PROOF](../proof/interactive-strategies.md). The change was archived on 2026-11-19; release is separate.

Peer-review remediation, 2026-11-17: removing the source from the explanation pane left the tested path unreachable at the minimum supported width, because the header truncated the text and the fallback repeated only the freshness status. The resolution keeps the owner's decision (no source repetition in the explanation): the header wraps provenance within its height budget, bounded so the body keeps at least eight rows and the key legend two, and the single line is retained when the text fits. A 55×19 TestBackend regression and a Linux pseudo-terminal render assert that the exercised long path is shown in full. Tasks 10.1–10.4 record that correction in [PROOF](../proof/interactive-strategies.md#narrow-terminal-header-path-closure--2026-11-17); task 11 extends access to exceptional paths that exceed the header budget.

Owner-selected overflow follow-up, 2026-11-17: when an unusually long valid path exceeds the header budget, the header states that the full path is available through the P shortcut. The dedicated view displays the complete selected path with keyboard scrolling and preserves the underlying UI state while open. This extends IA-05; TestBackend, Linux pseudo-terminal and integrated Linux verification passed on the recorded working tree. Cross-platform terminal behavior and human accessibility remain unverified; reconciliation is recorded in active OpenSpec task 11.5.

Final review acceptance (IA-04/IA-05, 2026-11-18): result details retain at least one readable content row at every supported size, even under the tallest allowed provenance header; keyboard scrolling exposes the final detail line.
