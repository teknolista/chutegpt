# Verification and acceptance plan

The following plan traces IA-01–IA-07 and S01–S22 to independently derived examples, known inputs and expected numbers. Executed evidence and remaining unverified checks are identified separately in the later records. The requirement-to-source-to-test map is maintained in [traceability](traceability.md).

- Source: existing actual Caixa workbook, numeric and named months, UTF-8/HTML March, duplicate/missing IDs, invalid dates/balls/month, missing file, permissions, corrupt workbook, stale/unknown provenance and snapshot changed during work.
- Numerical: parity on synthetic contests covering 0..7 even numbers and n−even odd; counts sum to eligible draws; independently calculate overlaps, runs, cycles, gaps, groups and dispersion including empty/first/zero boundaries, rounding and never-seen numbers. Test no future draw enters a prior-only metric. Regression cases from legacy report; preserve known-bug differences.
- TUI: menu keyboard traversal, launch each selected strategy, valid progress and cancellation, continued rendering, chart labels and exact values, result retention and return, long results, resize and too-small terminal, quit, non-TTY and terminal restoration after failure.
- Reproducibility: same snapshot and options produce identical results; record source hash, latest contest and chosen cutoff. Automated tests for computations and simulated terminal backend; real terminal smoke tests on each declared target.
- Integration: cargo fmt, clippy, test and release build; run user journeys on Windows/Linux/macOS as supported. Release review verifies distribution and evidence separately from specification and implementation. Missing target evidence remains unverified.

Path and provenance checks (planned): assert each OS default and missing-base error; explicit absolute and working-directory-relative overrides; repository and installed copied-pair journeys; version/source/hash/latest/date matching and earlier, absent, future, invalid or mismatched sidecars; updates between file selection and reading; unavailable base directory. Test the actual application entry point and record each OS separately. A same-day metadata record is not a check for a later published draw.

Reference-vector arithmetic check, 2026-11-16, working tree based on `244942b`: an independent short Python calculation of the two PRODUCT S01–S07 input sets returned even counts [4, 4], adjacent-link counts [3, 1], sums [97, 83], digital roots [7, 2] and intersection size 4. This checks the written arithmetic only; no Rust strategy or user journey was exercised. S05 and histogram/boundary behavior remain planned checks.

S17/S20 reference-vector check, 2026-11-16, working tree based on `244942b`: an independent Python calculation on the five prior draws and current draw in PRODUCT returned frequency top size 31 and 7 hits, absence top size 14 and 7 hits, and dispersion 0. This validates only the example arithmetic; rounding boundaries, unavailable history and the future Rust implementation remain unverified.

2026-11-16 author checks on working tree based on `244942b`: `python3` independently checked the representative PRODUCT vectors for S08–S16, S18–S19 and S21–S22 using explicit assertions (including percentage CV, example grouping, cycle thresholds, combination count endpoints and month gaps); S01–S07 and S17/S20 were checked separately as recorded above. These checks validate only selected example arithmetic and do not validate complete contracts, floating-point boundary behavior or Rust code. Inspection of the workbook header found principal and secondary winner/rate columns including lucky-month prizes. `cargo fmt --check` succeeded and `CARGO_TARGET_DIR=/tmp/chutegpt-target cargo test --offline` succeeded with zero tests on the bootstrap; neither establishes strategy behavior. `openspec validate estrategy-create --strict` succeeded on the working change.

Planned owner-confirmed series-statistics checks: independently verify every measure for [1,2,3], all tied modes and lower empirical median; empty, singleton, zero-only and angular cancellation cases; sample versus population variance; unknown observations, consecutive events, missing gaps and a censored tail; measure applicability and category navigation across S01–S22. Run both pure numerical checks and the real menu/result flow. See [PRODUCT](../product/series-statistics.md).

2026-11-16: author independently checked representative series arithmetic for [1,2,3], sample/population variance, circular mean, skewness, excess kurtosis, lower median and [yes,yes,no,yes] interval/periodicity vectors using Python assertions. Other listed edge cases and all Rust implementations remain planned. `openspec validate estrategy-create --strict` passed after the series requirement and task were added.

2026-11-16: an independent Python assertion script checked every defined statistic for [1,2,3], tied modes, lower median for four observations, cancellation for [0,π], and the known-event interval/periodicity vector in [series statistics](../product/series-statistics.md). A standalone Rust binary checked `f64::round_ties_even` at 2.5, 3.5, -2.5 and the two-decimal 1.125 case on the local Linux toolchain. These are reference-arithmetic and mechanism checks only; they do not exercise ChuteGPT. Network feasibility: sandboxed `cargo search ratatui --limit 1` failed DNS, while the approved host execution returned ratatui 0.30.2; no dependency has been added or built by that check.

2026-11-16, task 2.1 working snapshot: `CARGO_TARGET_DIR=/tmp/chutegpt-target cargo check` completed successfully on Linux with rustc 1.100.0 after registry retrieval under host network access. It emitted unused-dependency warnings for four dependencies not yet used. This compiles the bootstrap and dependency graph only; no statistical behavior, real terminal, Windows or macOS was checked.

2026-11-16 task 2.2, working tree based on `244942b`: `CARGO_TARGET_DIR=/tmp/chutegpt-target cargo test --offline --lib` passed 4 tests on Linux. They exercised the actual local official workbook (first draw and principal winner/rate), corrupt and missing files; generated workbooks with missing optional fields, invalid dates, duplicate numbers, missing contest ID and missing header; and stale, current, unknown, mismatched and future freshness records. `cargo fmt` and `cargo clippy --offline --all-targets -- -D warnings` passed after fixing one style diagnostic; Cargo separately emitted an unused Ratatui manifest warning. CLI/TUI entry-point behavior, concurrency during updates, comprehensive format variants and Windows/macOS remain unverified.

2026-11-16 task 2.3 on Linux working tree based on `244942b`: `cargo test --offline --lib` passed 6 tests, including the two-draw independent S01–S07 reference vector, all-zero/maximum parity, empty and first-draw exclusions, histogram totals, prior-only recurrence and callback cancellation/progress. `cargo clippy --offline --all-targets -- -D warnings` passed with Cargo's unused Ratatui manifest warning. UI, actual end-to-end processing and other operating systems remain unverified.

2026-11-16 task 2.4 on Linux working tree based on `244942b`: `cargo test --offline --lib` passed 8 tests, including independently derived seven-measure number/gap vectors, pooled gap counts, all decade and column boundaries, matrix values, distance, one rounding tie, empty input and callback cancellation. `cargo clippy --offline --all-targets -- -D warnings` passed with the existing unused Ratatui manifest warning. UI and cross-platform behavior remain unverified.

2026-11-16 task 2.5 on Linux working tree based on `244942b`: `cargo test --offline --lib` passed 10 tests. New cases cover 27/31 closure and tail, repeated combinations, first/last lexicographic ranks, first rank-band boundary (262958/262959, independently derived by combinatorial unranking), zero rank change, never-seen values, month delays and cancellation. `cargo clippy --offline --all-targets -- -D warnings` passed before the additional test, with the existing unused Ratatui manifest warning. UI and other platforms remain unverified.

2026-11-16 task 2.6 on Linux working tree based on `244942b`: `cargo test --offline --lib` passed 12 tests, including the independently specified prior-history tie/coverage vector, zero dispersion, selected-number periodicity and thresholds, mean-of-roots value, weekday winner/rate denominators, zero closed waits, unknown prizes and censored tails. `cargo clippy --offline --all-targets -- -D warnings` passed after one checked-division correction; Cargo still reports an unused Ratatui manifest dependency. Integration, UI and other operating systems remain unverified.

## Strategy computation evidence — 2026-11-16

Working-tree source snapshot based on `244942b`: `src/analysis.rs` SHA-256 `327710a187fa31f4f24b4542fe642f95faad76ec7ec79abffc85dfc47e4dfc16`; `src/advanced.rs` `243cffc29f85534eb74e1929ad1f1a48c3e724dcc2a138266e0e62f260b2ea2d`; `src/history.rs` `2d2d6f3649ab98ff329c7af76b985f199b4e7db5e7da45326b899dd0d33860a9`; `src/historical.rs` `f30d4c49a4fd05c9f4383a90b2d49e8a4d1a046db758148c1be39f5ee41b367c`. Workbook used by loader test: SHA-256 `5eca7d6edee23f998b665a8178daf2429f0789f6ca184a15c150c1d5da41fb06`. `CARGO_TARGET_DIR=/tmp/chutegpt-target cargo test --offline --lib` passed 12 tests on Linux; the relevant tests below check independent examples and selected boundaries, not exhaustive correctness.

| Strategy IDs | Passing library check |
| --- | --- |
| S01–S07 | `analysis::tests::independent_reference_vector_and_boundaries`; S03 also `recurrence_uses_only_prior_draws_and_progress_can_cancel`. |
| S08–S09 | `advanced::tests::independently_derived_composite_measures`. |
| S10–S12, S18 | `advanced::tests::grouping_boundaries_and_progress`. |
| S13 | `history::tests::separate_cycles_and_incomplete_tail`. |
| S14–S16 | `history::tests::exact_rank_endpoints_duplicates_and_changes`. |
| S17, S19–S20 | `historical::tests::prior_only_rank_ties_periodicity_and_coverage`. |
| S21–S22 | `historical::tests::missing_prizes_weekdays_and_censored_month_series`. |

These checks run calculations on synthetic vectors; the real workbook was validated through the source-loader test, not yet analyzed through the terminal. Shared series statistics, chart correctness, cancellation in an actual terminal, full integration and Windows/macOS remain unverified. Future source edits invalidate evidence for affected functions until the corresponding checks are rerun.

2026-11-16 task 2.8 on Linux working tree based on `244942b`: `src/series.rs` SHA-256 `ff7484ad5304373d0255b9315e84b375e25dbcbd968e3418643b31ef1f359fae`; `cargo test --offline --lib` passed 14 tests including all [1,2,3] reference measures, tied modes, lower median, zero/empty/singleton samples, undefined circular mean, invalid values, zero waits, missing intervals, censored tails and cancellation. `cargo clippy --offline --all-targets -- -D warnings` passed with Cargo's existing unused Ratatui manifest warning. Terminal category wiring and platform behavior are unverified.

2026-11-16 task 3.1, Linux working snapshot: pseudo-terminal runs of the actual debug binary with an explicit official workbook path selected parity and produced results through contest 1348; a copied workbook without sidecar prompted for unknown freshness and returned to the menu on refusal; a matching sidecar with an old date produced a stale warning and continued only after `s`; a missing file produced an error and returned to the menu. A noninteractive invocation exited nonzero with a terminal-required message. Terminal settings matched their initial values on these exits. These are local Linux observations, not Windows/macOS evidence or exhaustive UI acceptance.

2026-11-16 task 3.2 Linux working snapshot: a real pseudo-terminal exercise observed completed/total/percentage progress during recurrence, cancelled with Esc and confirmed successful exit and restored terminal settings; another cancelled during initial loading. A TestBackend regression check confirmed progress updates are capped per frame, a changing indeterminate spinner and transition back to the menu on cancellation. Earlier testing exposed progress messages starving redraw; the queue processing was bounded and the recurrence pseudo-terminal check then observed progress. Error-path restoration was also observed for a missing file. Sudden termination and Windows/macOS remain unverified.

2026-11-16 task 3.3, Linux working snapshot: the actual debug binary was exercised through a pseudo-terminal with the local official workbook; all 22 menu selections reached a retained result screen and returned to the menu, and terminal settings were restored at exit. A separate real-terminal parity run opened series details and returned. `cargo test --offline` passed 21 tests, including TestBackend checks for parity labels, section values, retained category and series navigation through resize and too-small state, and tests of zero bins, ordinal range labels and closed-wait histograms. These establish the exercised Linux flows, not exhaustive visual/accessibility review or Windows/macOS support.

2026-11-16 task 3.4 on Linux: `CARGO_TARGET_DIR=/tmp/chutegpt-target cargo run --offline -- --help` printed the documented path option. The README's `cargo run -- --planilha "docs/Dia de Sorte-atual.xlsx"` journey was exercised with Cargo offline in a pseudo-terminal: parity result through contest 1348, series details, return and normal exit succeeded; terminal attributes were restored. All existing README section headings were retained. Installed and other-platform invocations remain unverified.

## Integrated Linux verification — 2026-11-16

Tested working tree based on `244942b`, Linux x86_64, rustc/Cargo 1.100.0. The following source hashes identify the tested snapshot: Cargo.toml `370a31ce28e1997be6bf17f01f6e0ef33ef30213abf0ceceec7e75dfae1a60a7`; Cargo.lock `c706b4158fa8e499c27c6a5602a782ab00d496a6e3d732d0cf449f64390c0e51`; src/data.rs `5daea4dda4099cf947b1c8dfb20d23e07f9eaecb870300e22ba74d7c70524b76`; src/analysis.rs `327710a187fa31f4f24b4542fe642f95faad76ec7ec79abffc85dfc47e4dfc16`; src/advanced.rs `243cffc29f85534eb74e1929ad1f1a48c3e724dcc2a138266e0e62f260b2ea2d`; src/history.rs `2d2d6f3649ab98ff329c7af76b985f199b4e7db5e7da45326b899dd0d33860a9`; src/historical.rs `f30d4c49a4fd05c9f4383a90b2d49e8a4d1a046db758148c1be39f5ee41b367c`; src/series.rs `ff7484ad5304373d0255b9315e84b375e25dbcbd968e3418643b31ef1f359fae`; src/presentation.rs `ef02790228bd0f7b644796c09bafbd70ed6c40b27affcc99bd0497b0f3e39061`; src/ui.rs `e5dd1697e74b3c6c7f68e137c3a081a12a42b3d34c0a72842009ec5675cd1ad1`; src/lib.rs `8941a7e932a90c812d701b7df118d38f74d98ada8c1604bf5137301095dbe563`; src/main.rs `2429794a393fd5871425c0e0764f954ae576934234fea555f1f152e7f0ffa277`. The official workbook snapshot reached contest 1348 and had SHA-256 `5eca7d6edee23f998b665a8178daf2429f0789f6ca184a15c150c1d5da41fb06`.

- `cargo fmt --check`, `CARGO_TARGET_DIR=/tmp/chutegpt-target cargo clippy --offline --all-targets -- -D warnings`, `cargo test --offline`, debug build and `cargo build --offline --release` all passed. The test suite ran 21 library tests plus zero binary/doc tests.
- The actual debug entry point was exercised in a Linux pseudo-terminal with the official workbook: all 22 selections reached a result and returned to the menu. A separate parity run opened category-series statistics. Measurable recurrence progress was observed and cancelled; terminal attributes were restored. Known-stale and unknown copies prompted as specified, with explicit continue/refuse; missing and corrupt workbook paths produced errors; a noninteractive invocation failed clearly.
- A Linux pseudo-terminal resize from 100×24 to 30×8 showed the too-small warning; restoring 100×24 recovered the retained result. Initial failure to match the warning was a test-harness text/escape-sequence issue and was corrected; no application change was needed. Earlier progress starvation was fixed and has a regression test.
- Independently derived numerical examples and the source-specific check mapping are recorded above. These checks and a local traversal do not establish predictive advantage, exhaustive correctness, usability for all terminals, or Windows/macOS runtime behavior. Human accessibility assessment and actual installed/distributed use remain unverified.

Platform availability audit, 2026-11-16: actual checks ran on Linux x86_64 as detailed above. `rustup target list --installed` showed only `x86_64-unknown-linux-gnu`. Windows and macOS build, interactive terminal, workbook, path and recovery checks were unavailable in this environment and are **Unverified**; no portability or release claim is made. The evidence is tied to the recorded working-tree source hashes, not merely the base commit.

## Final integrated author review — 2026-11-16

Review scope: IA-01–IA-07, S01–S22, owner-confirmed shared series measures, local data and freshness handling, terminal navigation, numerical and integration checks. This is a self-review by the implementer, not an independent code review or release approval.

The review traced every menu selection to its calculation and presentation. The final Linux pseudo-terminal traversal reached and retained results for all 22 selections using the official workbook; TestBackend exercised every generated section and available series detail. Reference vectors cover each strategy family, and the selected numerical and failure boundaries are listed above. During review, the UI was corrected to show numbers not previously seen, per-draw periodicity distributions, per-field missing counts and reasons for unavailable measures; the first draw's missing-number count was fixed. The data reader was corrected to accept decimal and hexadecimal HTML entities in month names. A previously observed progress-queue starvation issue was fixed and regression-tested.

After those changes, Linux `cargo fmt --check`, `cargo clippy --offline --all-targets -- -D warnings`, `cargo test --offline` (22 library tests), and `cargo build --offline --release` passed. The 22-result pseudo-terminal traversal and representative parity/series, stale/unknown/corrupt/missing source, cancellation, resize and terminal-restoration journeys were rerun successfully. The final UI test addition and Clippy-only cleanup changed test code after the last terminal traversal, not runtime behavior. The current source hashes superseding the corresponding earlier ones are: `src/historical.rs` `0a13351cf7e5df3f1d4270117f43459edbc54eb4e8c76602e9aa01dc9c5885b8`; `src/presentation.rs` `fe488aecbe71587bbcfcc6bc7e80a941ce9e616cde8417c7c39717f9b57f53b5`; `src/ui.rs` `cfd562768bbbb21adf8129b9bba17df79072a8b706898e40abd2175de428d1bc`. Other hashes and the workbook snapshot remain those in the preceding integrated verification record.

No additional unresolved defect was identified in this author pass. Windows and macOS builds and terminal behavior, real installed distribution, independent code/security review, human accessibility assessment, broad performance characterization and predictive efficacy remain **Unverified**. The 22-choice traversal and tests are evidence for the conditions exercised, not exhaustive numerical or platform proof. Release and publication are outside the implementation and integration gate and require their own applicable authority and evidence.


Review remediation working snapshot: Rust `cargo test --offline` passed 22 tests, Python updater tests passed 10 tests, Clippy passed and release build passed on Linux. Added checks cover a boolean required field, early known delays, per-contest rows and scroll advancement. The sparse-workbook path, full updater/reader validation compatibility, minimum-size real-terminal view, complete wrapped-series navigation and exact monetary ties still require targeted end-to-end regression evidence. Windows and macOS remain unverified. Prior final-review claims and source hashes do not apply to the modified files.


Review remediation follow-up: on Linux, `cargo fmt --check`, `cargo clippy --offline --all-targets -- -D warnings`, `cargo test --offline` (24 Rust tests), 11 Python updater tests and `cargo build --offline --release` passed. Added regression cases cover prior-only series, known early delays, boolean fields, missing worksheet relationship, per-contest rows and half-cent rounding. An isolated 97 KiB sparse XLSX was rejected in a Linux pseudo-terminal and the terminal was restored. The release executable reached all 22 result screens before the final scroll-bound adjustment. Targeted real-terminal checks for the final scroll behavior and a systematic equivalence check between Python updater and Rust reader remain pending; Windows and macOS remain unverified. Earlier source hashes refer to historical snapshots.

Latest remediation checks on Linux: `cargo fmt --check`, `cargo clippy --offline --all-targets -- -D warnings`, `cargo test --offline` (24 passed), and `python3 -m unittest discover -s tests` (11 passed). Ratatui TestBackend confirmed a long wrapped series can reach its last line. The sparse-workbook pseudo-terminal check returned an error and restored terminal settings. An earlier release-binary traversal reached all 22 strategies before the final scrolling adjustment. Complete real-terminal verification of the final scrolling behavior and broad updater/reader compatibility remain pending.

## Linux integration closure — 2026-11-16

The owner narrowed this stage to Linux desktop; Windows and macOS are deferred. On Linux x86_64, `cargo fmt --check`, `cargo clippy --offline --all-targets -- -D warnings`, `cargo test --offline` (24 Rust tests), `python3 -m unittest discover -s tests` (12 Python tests), and `cargo build --offline --release` passed. `openspec validate estrategy-create --strict` passed. The official local workbook was accepted by both the Python updater validator and the Rust loader through contest 1348. A synthetic workbook accepted by the Python validator also reached a parity result through the release binary. Tests rejected missing package and worksheet relationships and boolean mandatory cells while preserving the preceding workbook and metadata. A separate sparse workbook was rejected by the Rust entry point with terminal restoration. These cases do not establish equivalence for every possible malformed XLSX; the validators remain independent.

The final release binary in a Linux pseudo-terminal displayed results for all 22 menu selections and restored terminal settings on exit. At 55 columns by 19 rows, a result was displayed and the terminal restored; resizing to 30 by 8 displayed the small-terminal message and restoring 100 by 24 retained the result. A corrupt workbook produced a recoverable error and restored the terminal. In a 100 by 24 terminal, scrolling the parity category series reached the final kurtosis and explanation after wrapped lines. Unit regressions exercise prior-only eligibility, per-contest rows and rounding. These checks cover the conditions stated, not every terminal, workbook variant or numerical input.

Affected source SHA-256: `src/data.rs` adb1fdc74ba97cbec0faa769c51487fbe8aa77c76a1eccc1ad6e336320ba158b; `src/ui.rs` 22a5f9b5c91cd16aa11371f0f33c8123bcf632cb775fc8754159e00fa345d77c; `src/presentation.rs` 2edeed265cdea58ac5d41901fa0061f50a4719f1355863b7234647f698962611; `scripts/atualizar_resultados.py` 98ec13abcf0d2fab895ba3a4aca7b0154e8caf4b970cdb6be2e9bea7e1f1eced; `tests/test_atualizar_resultados.py` 42fec46fbebf405da2ee123b8092fe0643b8769e668a7aeec05fe260fc15d83d. Prior snapshot hashes remain historical.

Implementation and Linux integration can be closed at this evidence level. Packaging, installed use, distribution, release authorization, human accessibility assessment, broader Linux environments, independent review after the latest fixes and predictive efficacy remain unverified or separate; these tests do not show an ability to predict a lottery draw.

## Pre-revision menu verification (historical)

Historical pending statement: at that pre-revision snapshot, the new menu proportions, all 22 Portuguese explanations, scrolling, resize, source header and removal of result provenance lines had not been implemented or tested. Later sections record their implementation and checks.

## Menu revision implementation checks

On Linux, `cargo fmt --check`, `cargo clippy --offline --all-targets -- -D warnings`, `cargo test --offline` (26 Rust tests), Python updater tests (12) and release build passed. The final runtime implementation was exercised in a Linux pseudo-terminal through the 22 result screens, with exit and terminal restoration; the later change to that source only added test assertions. Rust tests check descriptions exist in two paragraphs for all 22 entries, selection and scrolling, menu provenance states, result source/digest omission, and return to menu. Existing data tests cover matched, stale, missing and mismatched metadata. The descriptions were checked against the existing strategy definitions and result builders; these checks are not a proof of statistical correctness. The source SHA-256 for this test snapshot is `12d2fbc89dbb9c74d7de493a0d6dad796fcbd8f6154bb6fab4e5ae3092f47511`. An initial terminal capture did not reliably reconstruct cursor-positioned screen text; it is not evidence that individual menu strings were displayed. Human usability, all terminal widths and exhaustive metadata combinations remain unverified.

Review remediation on Linux: formatting and Clippy passed, 29 Rust tests and 13 Python tests passed, and the release build succeeded. Regression checks cover duplicated-header preservation, failed reload invalidation, pooled gaps without incorrect draw labels, and full path and selected name in explanation text. A pseudo-terminal run traversed all 22 result screens and restored terminal state. The narrow-terminal visual reachability of the complete path and all possible malformed workbook variants remain unverified.

Historical pending statement: the single-line menu-header order, dynamic source and freshness, failed-reload recovery and narrow-terminal access required tests and Linux terminal evidence at that snapshot. The later single-line correction and final review closure records contain the resulting checks; previous entries describe the earlier header layout.

Single-line header correction: on Linux, 30 Rust tests, rustfmt and Clippy passed; the release build passed. A TestBackend check asserted the exact rendered line with a selected path and matching current date. Existing tests cover stale, unknown and failed reload. The Linux release executable traversed all 22 result screens and restored the terminal. Full visual inspection of a long path at narrow width remains unverified; the existing scrollable explanation retains the full path. Tested src/ui.rs SHA-256: `38916db663ff0bad29e313dd4cdc9735e00bb6e11f1d94996518800785a4ec00`.

## Final review correction closure — 2026-11-16

This section supersedes the pending statements above for the current working tree. The prior records remain historical evidence for their original snapshots.

Changes:
- `src/ui.rs` no longer triggers `clippy::let_and_return`; the documented lint gate passes again.
- A background preflight read runs at startup, so the menu can show the hash- and sidecar-bound freshness before the first strategy. It does not start an analysis or a download. A strategy selection still reloads and revalidates the snapshot.
- S21 contract wording now states “average known principal prize amount per known rate row (not per winner)”; the implementation already followed the documented example.
- S17/S20 category series drop leading positions whose statistic is undefined instead of counting them as unknown events.
- Negative scale-100 values keep their sign in the formatter.
- README states the initial menu status accurately while the background verification is pending.

Executed on Linux x86_64:
- `cargo fmt --check`: passed.
- `cargo clippy --offline --all-targets -- -D warnings`: passed.
- `cargo test --offline`: 33 Rust tests passed. The new checks cover preflight status without starting an analysis, leading undefined-statistic exclusion, and negative hundredths formatting.
- `python3 -m unittest discover -s tests`: 13 Python updater tests passed.
- `cargo build --offline --release`: passed.
- `openspec validate estrategy-create --strict`: passed.
- Linux pseudo-terminal at 110×30 with `TERM=xterm-256color` and the official workbook: the first menu render contained `ChuteGPT`, `Fonte:` and `Verificada hoje` before any strategy key, then exited with `q` and restored the terminal.
- `--help`: usage printed, exit 0. Noninteractive invocation: clear interactive-terminal error, exit 1.

Source SHA-256 for this tested working tree: `src/data.rs` `adb1fdc74ba97cbec0faa769c51487fbe8aa77c76a1eccc1ad6e336320ba158b`; `src/analysis.rs` `327710a187fa31f4f24b4542fe642f95faad76ec7ec79abffc85dfc47e4dfc16`; `src/advanced.rs` `243cffc29f85534eb74e1929ad1f1a48c3e724dcc2a138266e0e62f260b2ea2d`; `src/historical.rs` `911030330f1396f5cd2097750d08b0ddc5ce149aec3897106ba42c85d1abd859`; `src/history.rs` `2d2d6f3649ab98ff329c7af76b985f199b4e7db5e7da45326b899dd0d33860a9`; `src/series.rs` `ff7484ad5304373d0255b9315e84b375e25dbcbd968e3418643b31ef1f359fae`; `src/presentation.rs` `2c003b18d2022a82419fdba65bdcfe4f5192c5fcf634fd4269a553e422b388d0`; `src/ui.rs` `6c983324552dc707d224db67ee0dcc1238086df21a17404381fb5695ca4801ab`; `src/main.rs` `2429794a393fd5871425c0e0764f954ae576934234fea555f1f152e7f0ffa277`; `src/lib.rs` `8941a7e932a90c812d701b7df118d38f74d98ada8c1604bf5137301095dbe563`; `scripts/atualizar_resultados.py` `22e6746e90cd60cb0f662d6c660d20104d10a9506bd98d9666133aca4695558d`; `tests/test_atualizar_resultados.py` `84b931d5128f6eb1bd8138d72c896296ea955a93f5c934bbeaf6c5dbdb2aa130`.

Limits: this closure covers the recorded Linux working tree and exercised journeys. It does not establish exhaustive numerical correctness for every strategy against all historical inputs, every malformed XLSX, Windows/macOS behavior, human accessibility or release/distribution readiness. The independent updater and Rust reader validators remain separate implementations and are not proven equivalent for every possible workbook.

## Peer-review follow-up closure — 2026-11-16

This section supersedes the immediately preceding closure for the current working tree; earlier records remain historical evidence for their tested snapshots.

Changes:
- A source that the background preflight read and then rejected is reported as `falha na verificação` in the menu header, never as `ainda não verificada`; the validation reason is appended to the menu explanation and Enter retries the load. A successful preflight, load or reload restores the bound freshness status, and a failed reload still invalidates the cached snapshot.
- S17/S20 category series declare the leading positions excluded as undefined (`Posições iniciais sem estatística definida, fora da população da série: N`), reconciling the series population with the result notice's valid/unavailable counts.
- Scale-100 formatting is shared through `presentation::format_value`, used by both the advanced sections and the UI, and preserves the sign of negative values.
- The earlier S17/S19 wording in task 7.4 and in this PROOF was corrected to S17/S20; the implementation applies the exclusion at strategy indices 16 and 19.
- The OpenSpec design now records the implemented path/override/sidecar selection and the preflight failure state, and the explanation-scroll check exercises the minimum supported terminal size (55×19).

Executed on Linux x86_64:
- `cargo fmt --check`: passed.
- `cargo clippy --offline --all-targets -- -D warnings`: passed.
- `cargo test --offline`: 36 Rust tests passed. New or changed checks cover a truthful preflight failure, preflight status without starting an analysis, agreement between the result and series populations, leading undefined-statistic exclusion, negative scale-100 formatting in the presentation layer, and explanation reachability at 55×19.
- `python3 -m unittest discover -s tests`: 13 Python updater tests passed.
- `cargo build --offline --release`: passed.
- `openspec validate estrategy-create --strict`: passed; `git diff --check`: passed.
- Linux pseudo-terminal at 110×30 with `TERM=xterm-256color`, screen reconstructed with a minimal cursor/CSI emulator because Ratatui redraws only changed cells:
  - `/tmp/opencode/invalido.xlsx` (11 bytes, not an XLSX): header `ChuteGPT — Análise Histórica Descritiva | Fonte: /tmp/opencode/invalido.xlsx | falha na verificação`; explanation shows `A verificação desta fonte falhou: Planilha Excel inválida: invalid Zip archive: Could not find EOCD`; `ainda não verificada` is absent.
  - Missing path: header also reports `falha na verificação`.
  - Official workbook: header reports `Verificada hoje` and no failure state.
  - S20 at 140×34: result `Observações válidas: 1337; indisponíveis: 11`; series panel declares `Posições iniciais sem estatística definida, fora da população da série: 11` with `Eventos: 4; desconhecidos: 0`, so the same 11 positions are explicitly excluded from the series population instead of being reported as unknown events.

Source SHA-256 for this tested working tree: `src/data.rs` `adb1fdc74ba97cbec0faa769c51487fbe8aa77c76a1eccc1ad6e336320ba158b`; `src/analysis.rs` `327710a187fa31f4f24b4542fe642f95faad76ec7ec79abffc85dfc47e4dfc16`; `src/advanced.rs` `243cffc29f85534eb74e1929ad1f1a48c3e724dcc2a138266e0e62f260b2ea2d`; `src/historical.rs` `911030330f1396f5cd2097750d08b0ddc5ce149aec3897106ba42c85d1abd859`; `src/history.rs` `2d2d6f3649ab98ff329c7af76b985f199b4e7db5e7da45326b899dd0d33860a9`; `src/series.rs` `ff7484ad5304373d0255b9315e84b375e25dbcbd968e3418643b31ef1f359fae`; `src/presentation.rs` `64f611027629c9fa91482e1df579e9eea69fa353faeadd95df9f7f99eef13266`; `src/ui.rs` `a138689aa181ba446b35f1f5f74697c206a449e4fdf0b97d27b18a5244fa7b45`; `src/main.rs` `2429794a393fd5871425c0e0764f954ae576934234fea555f1f152e7f0ffa277`; `src/lib.rs` `8941a7e932a90c812d701b7df118d38f74d98ada8c1604bf5137301095dbe563`; `scripts/atualizar_resultados.py` `22e6746e90cd60cb0f662d6c660d20104d10a9506bd98d9666133aca4695558d`; `tests/test_atualizar_resultados.py` `84b931d5128f6eb1bd8138d72c896296ea955a93f5c934bbeaf6c5dbdb2aa130`. Cargo.toml `bb0d0aaea0a2de9e158c6e7703170bdeeb059492596e5ef5c2612ede0aa8ad24`; Cargo.lock `c706b4158fa8e499c27c6a5602a782ab00d496a6e3d732d0cf449f64390c0e51`. Official workbook at contest 1348: `5eca7d6edee23f998b665a8178daf2429f0789f6ca184a15c150c1d5da41fb06`.

Limits: this closure covers the recorded Linux working tree and the exercised journeys. It does not establish exhaustive numerical correctness for every strategy and historical input, every malformed XLSX, other terminal emulators or widths, Windows/macOS behavior, human accessibility or release/distribution readiness. The pseudo-terminal evidence was reconstructed from cursor/CSI output; colors and content outside that subset were not evaluated. The independent updater and Rust reader validators remain separate implementations and are not proven equivalent for every possible workbook.

## Stage 1 final usability traversal — 2026-11-16 (commit 865acea)

On the published snapshot (`865acea`, source hashes as recorded in the preceding closure), the release executable was exercised in a Linux pseudo-terminal at 140×34 with `TERM=xterm-256color` and the official local workbook (contest 1348). The menu header showed the bound freshness state (`Verificada hoje`), and all 22 menu selections rendered a result screen with their first section title before returning to the menu: paridade, repetição, recorrência histórica, sequências adjacentes, maior sequência consecutiva, soma, raiz digital, dispersão das dezenas (7 seções), espaçamento (8), decenário (6), colunas (12), matriz (3), ciclos de cobertura (2), ordinal (2), dezenas individuais (32), mês da sorte (13), frequência e ausência (33), distância, média das raízes, dispersão entre frequência e atraso, premiação por dia da semana (10) e séries de mês e premiação (26). The application exited on `q` and restored terminal settings.

This traversal confirms that the delivered binary starts, selects every strategy, produces readable results and exits cleanly on the recorded Linux environment. It does not re-verify the numerical contracts (covered by the Rust checks), the freshness rejection state (covered by the pseudo-terminal checks in the preceding closure), other terminal sizes, other platforms, human accessibility or predictive performance.

## Workbook refresh and revalidation — 2026-11-17 (contest 1349)

On 2026-11-17 the daily updater was executed again on Linux and downloaded a new official workbook validated through contest 1349. `docs/Dia de Sorte-atual.xlsx` now has SHA-256 `b42c0304cde384c2eabfded20eca695e5b094aa3e264854002b1bfee1e57e900`; the version-2 sidecar records `date` `2026-11-17`, `latest` `1349`, the same source URL and that hash. The original `docs/Dia de Sorte.xlsx` still has SHA-256 `5eca7d6edee23f998b665a8178daf2429f0789f6ca184a15c150c1d5da41fb06` and was not replaced. This supersedes the workbook-bound evidence above for day-to-day use; the earlier contest-1348 records remain historical evidence for their tested snapshots.

Source SHA-256 for this tested working tree (unchanged from the peer-review closure): `src/data.rs` `adb1fdc74ba97cbec0faa769c51487fbe8aa77c76a1eccc1ad6e336320ba158b`; `src/analysis.rs` `327710a187fa31f4f24b4542fe642f95faad76ec7ec79abffc85dfc47e4dfc16`; `src/advanced.rs` `243cffc29f85534eb74e1929ad1f1a48c3e724dcc2a138266e0e62f260b2ea2d`; `src/historical.rs` `911030330f1396f5cd2097750d08b0ddc5ce149aec3897106ba42c85d1abd859`; `src/history.rs` `2d2d6f3649ab98ff329c7af76b985f199b4e7db5e7da45326b899dd0d33860a9`; `src/series.rs` `ff7484ad5304373d0255b9315e84b375e25dbcbd968e3418643b31ef1f359fae`; `src/presentation.rs` `64f611027629c9fa91482e1df579e9eea69fa353faeadd95df9f7f99eef13266`; `src/ui.rs` `a138689aa181ba446b35f1f5f74697c206a449e4fdf0b97d27b18a5244fa7b45`; `src/main.rs` `2429794a393fd5871425c0e0764f954ae576934234fea555f1f152e7f0ffa277`; `src/lib.rs` `8941a7e932a90c812d701b7df118d38f74d98ada8c1604bf5137301095dbe563`; `scripts/atualizar_resultados.py` `22e6746e90cd60cb0f662d6c660d20104d10a9506bd98d9666133aca4695558d`; `tests/test_atualizar_resultados.py` `84b931d5128f6eb1bd8138d72c896296ea955a93f5c934bbeaf6c5dbdb2aa130`. Cargo.toml `bb0d0aaea0a2de9e158c6e7703170bdeeb059492596e5ef5c2612ede0aa8ad24`; Cargo.lock `c706b4158fa8e499c27c6a5602a782ab00d496a6e3d732d0cf449f64390c0e51`.

Executed on Linux x86_64 against the contest-1349 workbook:
- `cargo fmt --check`, `cargo clippy --offline --all-targets -- -D warnings`, `cargo test --offline` (36 Rust tests), `python3 -m unittest discover -s tests` (13 tests), `cargo build --offline --release`, `openspec validate estrategy-create --strict` and `git diff --check` passed.
- Linux pseudo-terminal at 140×34 with `TERM=xterm-256color`, screen reconstructed with the same minimal cursor/CSI emulator: the menu header showed `ChuteGPT — Análise Histórica Descritiva | Fonte: …/docs/Dia de Sorte-atual.xlsx | Verificada hoje`; parity reported `Observações válidas: 1349; indisponíveis: 0`; S20 reported `Observações válidas: 1338; indisponíveis: 11` and its series panel declared `Posições iniciais sem estatística definida, fora da população da série: 11` with `Eventos: 4; desconhecidos: 0`; the application exited cleanly and restored terminal settings.

Limits: one additional official draw was incorporated and revalidated on Linux; this does not establish exhaustive numerical correctness, other terminal emulators or widths, Windows/macOS behavior, human accessibility, release/distribution readiness or predictive performance. The independent updater and Rust reader validators remain separate and are not proven equivalent for every possible workbook.

## Test flakiness note — 2026-11-17

Observed on 2026-11-17 during the cross-artifact review: `ui::tests::preflight_updates_menu_status_without_starting_analysis` failed with `assertion failed: app.snapshot.is_some()` when the full library suite ran in parallel (default `cargo test --offline` with multiple threads). The test polls the background preflight for a fixed budget (500 × 10 ms); with several tests reading the same official workbook concurrently, that budget can expire before the message arrives. `data::load` has no timeout. Running with `--test-threads=1` or in isolation passes reliably. This is a test-harness constraint, not a defect in the analysis, loader or UI logic; source code and computed results are unaffected.

Mitigation: use `cargo test --offline -- --test-threads=1` or run the affected test individually for deterministic verification. This constraint is recorded in [decisions](../project/decisions.md) (DEC-6) and in [PROCESS execution](../process/execution.md). The failing assertion verifies only that the preflight populated the snapshot; the downstream behavior (all 22 strategies, freshness display) is correct on the recorded snapshot.

## Header consistency and expanded strategy explanations — 2026-11-17

Tested working-tree snapshot on Linux x86_64; `src/ui.rs` SHA-256 `d439b058a3b9caf63137068b33b2814162a0e339ae8c0f3a694ece5efeecfb59`, `README.md` SHA-256 `4dd70b616a2992e3af88ddabbd50ba27412217e1c1b001ae67cb53dbd60a6e9a`. The selected official workbook remained contest 1349, SHA-256 `b42c0304cde384c2eabfded20eca695e5b094aa3e264854002b1bfee1e57e900`.

- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test -- --test-threads=1` (37 Rust tests), `cargo run -- --help`, `cargo build --release`, `openspec validate estrategy-create --strict` and `git diff --check` passed.
- UI TestBackend regressions verify the shared provenance header in menu, loading, working, confirmation, result, series and error states; verify no source path is appended to the menu explanation; and verify all 22 descriptions contain three paragraphs and change with the selected strategy.
- At both 55×19 and 100×24, keyboard Page Down reaches the final word of each strategy description and selection resets the scroll. The 55×19 case exposed a one-column width mismatch between scroll measurement and the rendered 30/70 pane; scroll measurement now uses Ratatui's same split and interior width.

These are automated TestBackend layout and interaction checks, not a human accessibility assessment or a test of every terminal emulator. They do not establish Windows/macOS support, release readiness or predictive efficacy. The active change remains unarchived.

## Narrow-terminal header path closure — 2026-11-17

Peer-review finding (P2): after the explanation pane stopped repeating `Fonte:`, the selected path became unreachable at narrow widths, because the header truncated its text and the old fallback rendered only `header` plus the freshness status on a second line. The capability specification still required the complete path to be accessible, and PRODUCT/PROJECT still asserted it. This section records the remediation.

Resolution for the path lengths exercised in this review: the owner's decision to keep the explanation free of the source is preserved, and the path is shown in the header when it fits the bounded space. `src/ui.rs` measures the provenance text with Ratatui's own word wrapper at the header's interior width and sizes the header block from that count plus borders, bounded by `area.height - 10` so the body keeps at least eight rows and the key legend two. The single-line layout is retained when the text fits; otherwise the same text wraps within that limit. The previous two-line fallback was removed. Exceptional paths beyond this budget are addressed in the following section.

Executed on Linux x86_64, working tree with `src/ui.rs` SHA-256 `445e9cbd650b39672f453d9675d4bc5354fffb160cacb0561414a65fb53d7c25`, `README.md` `d10a0b55e423655314bbd077c83e5327eac19c6822c54dbfff59a2d6144f4874` and the capability specification `0f4db2cd46c011517bfcabfb9fa0bfc971c9ce98071789bf2efaefc4db54c7fb`; official workbook at contest 1349, SHA-256 `b42c0304cde384c2eabfded20eca695e5b094aa3e264854002b1bfee1e57e900`.

- `cargo fmt --check`, `cargo clippy --offline --all-targets -- -D warnings`, `cargo test --offline -- --test-threads=1` (38 Rust tests), `python3 -m unittest discover -s tests` (13 tests), `cargo build --offline --release`, `openspec validate estrategy-create --strict` and `git diff --check` passed.
- New TestBackend regression `ui::tests::provenance_header_keeps_the_full_path_visible_on_narrow_terminals` renders a 71-character selected path at 55×19 and 100×24 and asserts that the whole path appears in the header block, that the title is still rendered and that the explanation pane (`Como funciona`) remains visible below it.
- Linux pseudo-terminal, release binary, `TERM=xterm-256color`, screen reconstructed with the cursor/CSI emulator:
  - 55×19 with `--planilha /tmp/opencode/uma/pasta/bem/comprida/para/testar/o/cabecalho/ChuteGPT/Dia de Sorte-atual.xlsx`: the header occupied four text lines plus borders and displayed the complete path followed by `Verificada hoje (17/11/2026)`; the strategy list and `Como funciona` pane remained usable.
  - 55×19 with the repository workbook and a parity result: the three-line header kept the whole path and the result sections rendered normally.
  - 180×34 with the repository workbook: the header stayed on one line with the full path.
  - 30×8: the minimum-size message replaced the views, as specified.

Limits: these checks cover TestBackend layout plus one Linux terminal emulator through a reconstructed screen; other terminals, human accessibility, Windows/macOS, release/distribution and predictive efficacy remain unverified. A path whose wrapped header would exceed `area.height - 10` lines is still bounded by that limit, which is far above the lengths exercised here.

## Exceptional selected-path overflow view — 2026-11-17

The owner selected a dedicated scrollable provenance view for paths that exceed the header's reserved height. Acceptance is specified in the active `estrategy-create` capability and task 11. In the tested implementation, the bounded header displays an explicit overflow notice; `P` overlays the complete selected path, vertical scrolling reaches both ends, and `Esc` or `P` returns to the preserved underlying screen. The overlay leaves background progress and completion polling active.

Tested Linux x86_64 working tree based on HEAD `8e40f033e34634015d0272abaaf9de6fe116d259` with existing local changes. Snapshot hashes: `src/ui.rs` `f79fd77d294041e8c7f90bf01243c8d7fede5478560312347d640c0e3ab3e416`; `README.md` `5c269012ee52b16974128821d4e3f48db92558599dc0f5a2f45834971f24b935`; active capability spec `e08a2af98d183c1f9a7cf2cc90dce1d168005a52ffca7a4fdcf3c3c248e33859`; official workbook at contest 1349, SHA-256 `b42c0304cde384c2eabfded20eca695e5b094aa3e264854002b1bfee1e57e900`.

- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test -- --test-threads=1` (39 Rust tests), `python3 -m unittest discover -s tests` (13 tests), `cargo run -- --help`, `cargo build --release`, `openspec validate estrategy-create --strict` and `git diff --check` passed.
- TestBackend `ui::tests::oversized_provenance_view_scrolls_and_preserves_processing_state` passed at 55×19 with five 240-character path components. It asserts the overflow notice, access to the first and final path markers after scrolling, and preservation of a `Working` state through progress and completion events while the overlay is open; closing restores the resulting `Result` screen. The existing fitted-path regression still passes at 55×19 and 100×24.
- Linux pseudo-terminal with the release binary, 55×19: a missing-source path containing the same long components produced the overflow notice; `P` opened the view, the initial page showed `START-OF-PATH`, End exposed `END-OF-PATH.xlsx`, and Esc closed the view. Enter then reached the expected load error from the restored menu; Esc/Q returned and the process exited 0.
- After reconciling the final PROCESS records and task 11.5, `openspec instructions apply --change "estrategy-create" --json` reported `all_done`, 59/59 tasks complete; `openspec validate estrategy-create --strict` and `git diff --check` passed on the updated documentation snapshot.

Limits: the pseudo-terminal case deliberately used a nonexistent workbook to exercise the UI and error journey; it does not establish a successful read from that extreme path. Evidence covers Linux TestBackend and one Linux pseudo-terminal at 55×19, not other terminal emulators, Windows/macOS, human accessibility, release/distribution or predictive efficacy. `git diff --check` and strict OpenSpec validation were rerun on this working tree; archive and release remain separate.

## PentaPRO alignment audit — 2026-11-17

Scope: compare all PRODUCT, PROJECT, PRODUCTION, PROOF and PROCESS records with the current implementation and its tests; reconcile the root README, PentaPRO navigation, AGENTS, `docs/`, Cargo metadata, both active OpenSpec changes, the preserved historical OpenSpec draft and relevant source/test contracts. Baseline is HEAD `8e40f033e34634015d0272abaaf9de6fe116d259` plus the pre-existing working-tree changes and the documentation corrections in this audit. No application behavior was changed by the audit.

Outcome: the selected Stage 1 behavior is consistent across the implementation and artifacts at the evidence level below. PRODUCT describes observable behavior, domain rules and acceptance without requiring a programming language or framework; Rust, Ratatui/Crossterm, Calamine and module boundaries remain in PROJECT. Historical status entries remain as historical snapshots. README content and section structure were preserved. At the time of this audit `estrategy-create` was complete at 59/59 tasks and `atualizar-resultados-diarios` at 6/6; both remain active and unarchived, and `estrategy-create` later reached 70/70 with the remediation tasks below.

The official CAIXA Dia de Sorte page was checked on 2026-11-17 and its current mechanics, schedule and listed values were recorded in [the reference](../../docs/dia-de-sorte-referencia.md). The Manual v19 PDF endpoint repeatedly redirected to itself and could not be revalidated; manual-specific exceptions remain attributed to the earlier source consultation and are not represented as newly verified. Logo source/redistribution rights and exact SPDX identification remain Pending. This audit does not establish owner acceptance, legal conclusions, human accessibility, installation, other-platform support, predictive efficacy or release readiness.

Checks on Linux x86_64, at the same source/test snapshot: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test -- --test-threads=1` (39 passed), `python3 -m unittest discover -s tests` (13 passed), `cargo build --release`, `cargo run -- --help`, strict validation of both OpenSpec changes, local Markdown/image link-target validation (46 Markdown files) and `git diff --check` passed. The two OpenSpec task counts were 59/59 and 6/6. Link validation checks local target existence, not external links or all heading-anchor semantics.

## Dynamic scroll viewport remediation — 2026-11-17

Peer-review finding (CR-001): static vertical offsets `saturating_sub(8)` in menu/series and `saturating_sub(17)` in results assumed a constant 4-row header. When the provenance header wraps onto 3 or more lines (height 5 or greater on narrow terminals), those static offsets overclamped scroll positions and prevented users from reaching the bottom lines of strategy descriptions and result details.

Resolution: the visible vertical viewports for menu explanation (`menu_explanation_viewport_height`), series details (`series_viewport_height`) and result details (`result_details_viewport_height`) are derived dynamically from `body_area_height(app)`, which accounts for `header_height(&display_header(app, area), area)`. Two new TestBackend regressions (`explanation_scrolling_reaches_end_with_multi_line_header` and `scroll_viewports_adjust_to_dynamic_header_height`) verify complete reachability at 55×19 under a 3-line wrapped header and verify viewport adjustment across narrow and wide layouts.

Executed on Linux x86_64:
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo test -- --test-threads=1`: 41 Rust tests passed (0 failed).
- `python3 -m unittest discover -s tests`: 13 Python tests passed.
- `cargo build --release`: passed.
- `cargo run -- --help`: passed.
- `openspec validate estrategy-create --strict`: passed.
- `git diff --check`: passed.

Snapshot hashes (SHA-256):

| Area | File | SHA-256 |
| --- | --- | --- |
| Application | `src/main.rs` | `2429794a393fd5871425c0e0764f954ae576934234fea555f1f152e7f0ffa277` |
| Application | `src/ui.rs` | `f63158c1c3fec3c5bf7eb46e22c1337fbb01cc47d43995676a2d39185fb18acf` |
| Data and analysis | `src/data.rs` | `adb1fdc74ba97cbec0faa769c51487fbe8aa77c76a1eccc1ad6e336320ba158b` |
| Data and analysis | `src/analysis.rs` | `327710a187fa31f4f24b4542fe642f95faad76ec7ec79abffc85dfc47e4dfc16` |
| Data and analysis | `src/advanced.rs` | `243cffc29f85534eb74e1929ad1f1a48c3e724dcc2a138266e0e62f260b2ea2d` |
| Data and analysis | `src/history.rs` | `2d2d6f3649ab98ff329c7af76b985f199b4e7db5e7da45326b899dd0d33860a9` |
| Data and analysis | `src/historical.rs` | `911030330f1396f5cd2097750d08b0ddc5ce149aec3897106ba42c85d1abd859` |
| Data and analysis | `src/series.rs` | `ff7484ad5304373d0255b9315e84b375e25dbcbd968e3418643b31ef1f359fae` |
| Presentation | `src/presentation.rs` | `64f611027629c9fa91482e1df579e9eea69fa353faeadd95df9f7f99eef13266` |
| Build metadata | `Cargo.toml` | `41d8ef40faf832515541d53ac13e6dc6b35728f15cb683985f2cff20a410cfbf` |
| User documentation | `README.md` | `083d437cbfed6bbb859b3a93d723240560a0e9aaefdfe26300f7aa15e75a300b` |
| User documentation | `docs/dia-de-sorte-referencia.md` | `6ae6c4e429adba17ada3e951647223b6a721899e18ac0aa487a22ce74e9aac0a` |
| PRODUCT contract | `pentapro/product/interactive-strategies.md` | `d8642a5e1445fbfa1802d6eadc4c5af78c23c14c01c847444fb94fc93f4e1f96` |
| PROJECT contract | `pentapro/project/interactive-strategies.md` | `69805e6b97427e657ee78b71cb249ea4650aba2b87c0e0dadb262faec3862d1d` |
| OpenSpec proposal | `openspec/changes/estrategy-create/proposal.md` | `2e982fb54f9af6efa6c4456f9d7ce10ca1b8930dc491101713dcd373b7469552` |
| OpenSpec capability | `openspec/changes/estrategy-create/specs/interactive-strategy-analysis/spec.md` | `c3ca9929ec382e2481de7635de39a1c0aa1429fd31987f0be4fb40c480a30281` |

## Peer-review robustness remediation — 2026-11-17

Scope: the findings supported by the independent review of the working tree based on HEAD `8e40f033e34634015d0272abaaf9de6fe116d259` plus local changes. No release, archive or publication was performed.

Corrections in `src/data.rs`: declared ZIP sizes are still summed, but the bytes actually inflated are now counted per entry and capped by the same 50 MiB total, so an entry that understates its size cannot bypass the limit; the first streaming pass rejects text cells above 64 KiB and ISO date strings with or without a time component are accepted alongside `DD/MM/YYYY` and Excel dates; wholly empty rows before the first data row are skipped while an empty row between data rows still fails. In the analysis modules, `Draw::is_valid` rechecks the seven-number/month invariants at the public entry points, invalid draws return an explicit error instead of shifting or indexing out of range, the hundredths rounding rule is shared through `src/numeric.rs`, pooled gaps are dispatched by an explicit metric flag, unsupported display scales are marked, and month labels use a checked lookup.

Executed on Linux x86_64 at this snapshot:

- `cargo fmt --check`: passed.
- `cargo clippy --offline --all-targets -- -D warnings`: passed.
- `cargo test --offline -- --test-threads=1`: 47 Rust tests passed (0 failed), including new checks for leading blank rows, bounded inflation, ISO date variants, wrong-typed/version-3/missing-date sidecars, invalid draws and unsupported scales.
- `python3 -m unittest discover -s tests`: 13 Python tests passed.
- `cargo build --offline --release`: passed.
- `openspec validate estrategy-create --strict`: passed.
- `openspec validate atualizar-resultados-diarios --strict`: passed.
- `git diff --check`: passed.
- Markdown link and anchor sweep over 167 files: 659 local links resolved, none missing, no broken anchors (external URLs and source-file `#Lnnn` anchors excluded).

Snapshot hashes (SHA-256):

| Area | File | SHA-256 |
| --- | --- | --- |
| Application | `src/data.rs` | `d2dbd753cc7ba5eba59eb54c9b1bc3466844b5b5971b431d7fbe3af0c371b367` |
| Application | `src/numeric.rs` | `d7b74c0e8d2ecfee44f00d3b9fa71d4517e9a89468005d4f5a8d2870ebc6ee2a` |
| Application | `src/history.rs` | `2c139bec854226a12cecb730893df87160e354030216c07b6f34ff181d410149` |
| Application | `src/historical.rs` | `a497c63b319a2e4cadf2b03b4d2a368deace9cef30f18b7671191016e2b49107` |
| Application | `src/advanced.rs` | `17ae24582f75411f207b26633c5726f5f8cbf09de8bef60cb3ad08f0110e02cd` |
| Application | `src/presentation.rs` | `8df524f8e282193094c4b5c0bc369b11d3a97566a89f3d247f9f461d09da3c2f` |
| Application | `src/lib.rs` | `586625b16f82dc457ee87a041f5cb85879cd44cf4829bada59fa96a934fcc5ac` |
| Application | `src/ui.rs` | `f63158c1c3fec3c5bf7eb46e22c1337fbb01cc47d43995676a2d39185fb18acf` |
| Application | `src/main.rs` | `2429794a393fd5871425c0e0764f954ae576934234fea555f1f152e7f0ffa277` |
| Application | `src/series.rs` | `ff7484ad5304373d0255b9315e84b375e25dbcbd968e3418643b31ef1f359fae` |
| Application | `src/analysis.rs` | `327710a187fa31f4f24b4542fe642f95faad76ec7ec79abffc85dfc47e4dfc16` |
| OpenSpec | `openspec/changes/estrategy-create/tasks.md` | `efdd4136a51ba2ecc8b0693dba25cc841ccd6e2c159b30e7d3ab159eb6a33ed0` |
| OpenSpec | capability spec | `34c16c096d8ba250f17421b479777109b8fc0f380f06337d3a2de967a88a4332` |
| PROJECT | `pentapro/project/decisions.md` | `25d7fac1b67bb86b4edfa61049a15eb1e8ec66705f9f1c51036b197b22fb5c4a` |
| PROJECT | `pentapro/project/architecture.md` | `cc07e26dd8a7e0c95c9dde60d16d9d1c6662fbfe9f0fff15aacce7d17687165d` |
| PROJECT | `pentapro/project/interactive-strategies.md` | `59a5c1750d07c8b444966e41eb81bb154f26e47ac7a96ffd8539d93432679482` |

Limits: the ZIP defence was verified with a synthetic counting-reader test, not with a hand-crafted archive that lies about its sizes; the per-cell text bound was not exercised with a 64 KiB live workbook. The `Draw` validation changes the public error types of the history/advanced/historical modules, which are exposed through public modules in this development crate; the CLI interface is unchanged. Duplicate interval-censoring logic between `src/series.rs` and `src/historical.rs` remains a maintenance risk to consolidate in a future increment. Linux x86_64 only; Windows/macOS, release, distribution and predictive efficacy remain unverified.

## Review remediation: verification gate hardening — 2026-11-18

Addresses the findings of the same-day peer review of the working tree (HEAD `8e40f03`) plus its uncommitted remediation snapshot.

- `cargo test -- --test-threads=1`: 47 Rust tests passed (0 failed) after the remediation; the eight tests that previously returned early when the gitignored official workbook was absent were converted to always-on synthetic-fixture runs (`src/test_fixture.rs`, `#[cfg(test)]`-only, 2–1100 draws per fixture). The official golden checks (contest 1 == `[3, 5, 8, 9, 19, 21, 30]`, 1000+ contests, winners/cents of the key prize row) run when the local file is present; absence emits a diagnostic visible with `--nocapture` (the normal successful-test harness captures it).
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo build --release`: passed.
- `python3 -m unittest discover -s tests`: 13 Python tests passed.
- `openspec validate estrategy-create --strict`: passed after the requirement promotion.
- The prompt log itself is user-local and ignored; `.gitignore` and `docs/agents-log-policy.md` are shared repository policy artifacts. They were not changed by that remediation.

Snapshot hashes (SHA-256):

| Area | File | SHA-256 |
| --- | --- | --- |
| Application | `src/data.rs` | `e8f8324d2d35e037d94dfce18ed12be898ec494dd89244e71355fbbb9c9ed91a` |
| Application | `src/lib.rs` | `f9afa534f1b147c3347afed127be616f54c0c89bf442afed966c30a63bb9e8ff` |
| Application | `src/test_fixture.rs` | `f4a1984539a4e992463df336b4a6b574c3288fe31d9c81696c48913cff339e72` |
| Application | `src/ui.rs` | `1c363daf9dbfa832f727dd71d34ecd8ce66fe01265ed5ea57d4f92826e681194` |
| Application | `src/presentation.rs` | `da5196bfaf4c242d1d69c843d54f5c41a58f45cdc93ae3a48efec78444146ebd` |
| OpenSpec | capability spec | `194da86646af90c1546eed7b9537492f305cd90434e7a3f4db7f388a87d8005b` |
| OpenSpec | `openspec/changes/estrategy-create/tasks.md` | `d8f3552219b58827977e7099f42496ee87674ffc339eda601e655d76ac06de24` |

Limits: the official-workbook golden checks still depend on a local, non-committable copy of `docs/Dia de Sorte-atual.xlsx` (gitignored); fresh clones and any CI without that file get the synthetic gate only and both must pass independently. The synthetic fixture verifies integration contracts and golden rendering, not the official prize-row values. No CI workflow exists in this repository; the gate remains a documented local sequence.

## Final cross-artifact review — 2026-11-18

Scope: the complete unpublished diff and relevant untracked files against HEAD `8e40f033e34634015d0272abaaf9de6fe116d259`, including active `estrategy-create`, source/tests, the daily updater contracts, root README, reference/research files, review/evaluation records, all affected PentaPRO pillars, agent guidance and assets. This session reviewed the earlier implementation and performed its own remediation/re-review; the final fixes have not received a separate independent sign-off. No change was archived and no release was prepared. The owner explicitly requested commit and normal push to the existing `origin/main` destination.

### Findings and resolution

| ID / severity | Confirmed trigger and impact | Resolution and regression |
| --- | --- | --- |
| RV-01 / P2, resolved | `src/ui.rs` result layout reserved 2 subtitle + 7 chart rows even when a tall wrapped header left fewer than 12 body rows at 55×19. The bordered detail pane had zero readable rows, so scrolling could not expose results. Introduced by the expanded-header layout; reproduced before the fix with a 120-character path. Earlier viewport tests only checked arithmetic for 4/5-row headers. | Shared `result_areas` shrinks the chart to preserve a detail content row. `result_details_remain_readable_under_tall_headers` asserts a final rendered marker for paths of 120/180/240/300/400 characters. The reproducer failed before correction and passes afterwards. |
| RV-02 / P2, resolved | Task 13.5 and DEC-9 claimed validation at public analysis entry points, but `analysis::calculate` still accepted malformed draws (unsorted adjacency could panic), and `history::ordinals` ignored an invalid month. These were pre-existing API gaps missed by the hardening scope; the CLI loader already rejected invalid source draws. | Both entry points reject invalid draws. `every_strategy_rejects_invalid_public_draws` checks all 22 presentation routes with zero/out-of-range, unsorted and duplicate numbers and months 0/13; simple-analysis cancellation retains its separate error. Public Rust error types change within this development crate; CLI arguments are unchanged. |
| RV-03 / P2, resolved | The input contract accepted ISO date strings with time, but `data::date` rejected ISO text cells and fractional local date-times. A valid XLSX text date could fail required-data loading. The earlier ISO extension only handled a subset of native ISO cells. | ISO text/native date cells accept date-only, local time with optional fractions, and RFC 3339 offset forms. Direct date cases and a real XLSX integration fixture pass. Numeric boundary assertions also cover exact 2^64 rejection, the last representable lower integer, maximum textual centavos/overflow and signed hundredths limits. The XLSX test also proves the 65,536-byte text-cell boundary is inclusive and 65,537 is rejected, closing the earlier unexercised text-bound gap. |
| RV-04 / P2, resolved | Current proposal/index/reconciliation counts lagged behind tasks; traceability named a removed test; architecture claimed nonexistent root re-exports/types and placed expanded explanations in the wrong module; evidence miscounted fixture migrations and implied a nonexistent remediation commit. The prompt policy omitted shared CLAUDE/policy files. These inconsistencies undermined review and resumption. | Current records now report 79/79 tasks, actual module/test names, eight migrated tests and uncommitted historical snapshots. README sections are preserved; the blank ticket and future roadmap are inventoried without claiming current rules or generation implementation. Historical evidence remains tied to its original snapshots. |

### Executed checks

Linux x86_64, rustc/Cargo 1.100.0, final source snapshot below:

- `python3 scripts/atualizar_resultados.py`: same-day skip; the existing validated copy reaches contest 1350. No second download or later same-day publication check was performed.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo test -- --test-threads=1`: 50 Rust tests passed, including the official-copy golden checks.
- `cargo test -- --test-threads=1 --nocapture` with the official workbook temporarily unavailable at its actual expected path: all 50 tests passed and the expected missing-official-copy diagnostic was observed. The workbook was restored in a `finally` block and its SHA-256 verified unchanged. This directly verifies the synthetic gate without the ignored workbook; it does not verify official prize values in a fresh clone.
- `python3 -m unittest discover -s tests`: 13 Python tests passed.
- `cargo build --release` and `cargo run -- --help`: passed.
- Both active changes pass `openspec validate <change> --strict`; the interactive apply status reports all 79 tasks complete after reconciliation.
- Release binary in a Linux pseudo-terminal (`TERM=xterm-256color`), official contest-1350 workbook through a 161-character selected path: 55×19 menu/preflight/result, scrolling to the final detail line, P overlay/close, resize to 30×10 and back, menu/quit and terminal-attribute/alternate-screen restoration passed. ANSI cursor updates were reconstructed to check the visible final detail line, rather than searching the raw incremental byte stream.
- Local Markdown targets/anchors (50 files, 361 local targets/anchors), candidate-path credential-pattern scan and `git diff --check`: passed. Scan scope is repository Markdown files (excluding ignored agent packages) and all 45 candidate publishable paths; no prompt log is selected. This is a bounded pattern scan, not a comprehensive security audit.
- The current CAIXA page was rechecked on 2026-11-18; listed mechanics/prices/schedule agree with the reference. The Manual v19 and historical rule effectiveness were not revalidated in this pass.

### Snapshot

SHA-256 of the final source, data and key contracts (historical hashes above retain their original scope):

| File | SHA-256 |
| --- | --- |
| `src/advanced.rs` | `17ae24582f75411f207b26633c5726f5f8cbf09de8bef60cb3ad08f0110e02cd` |
| `src/analysis.rs` | `dc1373634f1cfc1addd2e618687a9126987b59443646479990c8eca3d28c72ee` |
| `src/data.rs` | `698477b37a02180b341160b88ba24d0e530af1dc0c9f3117a101c024f8972bb2` |
| `src/historical.rs` | `a497c63b319a2e4cadf2b03b4d2a368deace9cef30f18b7671191016e2b49107` |
| `src/history.rs` | `e4f69bebe7d4e2339110f16037b7d59c0553d4640d4d90fb7931d7729f56fddd` |
| `src/lib.rs` | `f9afa534f1b147c3347afed127be616f54c0c89bf442afed966c30a63bb9e8ff` |
| `src/main.rs` | `2429794a393fd5871425c0e0764f954ae576934234fea555f1f152e7f0ffa277` |
| `src/numeric.rs` | `401d394f53f4279856964e8ea2b96049872e5fa29939b59dddcc4ebb2c620893` |
| `src/presentation.rs` | `881b4ee1267fba4fa76c5e16a5a4ea09b0cb593448047eee004d45b4bdde2d44` |
| `src/series.rs` | `ff7484ad5304373d0255b9315e84b375e25dbcbd968e3418643b31ef1f359fae` |
| `src/test_fixture.rs` | `f4a1984539a4e992463df336b4a6b574c3288fe31d9c81696c48913cff339e72` |
| `src/ui.rs` | `a75eba4180e7659929b57464ad1691f9b06e051b3b79bde375066d45ed5ca576` |
| `Cargo.toml` | `41d8ef40faf832515541d53ac13e6dc6b35728f15cb683985f2cff20a410cfbf` |
| `Cargo.lock` | `c706b4158fa8e499c27c6a5602a782ab00d496a6e3d732d0cf449f64390c0e51` |
| `README.md` | `9743351fb8fd632ba2c24857487aff77aa18deaa045b8b14680a01ced26afec9` |
| `docs/dia-de-sorte-referencia.md` | `267bb703b2362bee77c41271e56555cd47a0753b1d95eeadeaebd1fd7f43f992` |
| `docs/Dia de Sorte-atual.xlsx` | `f80d8c3e71f7cda4a3c7061cc3009fa7b24226b6d355dd6770560d01e1182eab` |
| `openspec/changes/estrategy-create/specs/interactive-strategy-analysis/spec.md` | `43e2038362d41a3461ef962ef85c0365e3990a2e9eac647f4db5a34e3c56df23` |

Verdict: **No blocking findings after remediation and focused re-review** within this scope. Missing evidence remains explicit: no CI workflow, independent sign-off on the final fixes, exhaustive malformed-XLSX equivalence, full historical rule verification, human accessibility/usability acceptance, installed distribution, other platforms or predictive performance. The earlier forged-ZIP coverage remains a counting-reader check rather than a crafted lying archive. The prompt-capture plugin is user-local and its historical OpenCode tests were not rerun by this Codex session; manual append and Git exclusion were verified here. Logo/ticket origin and redistribution terms remain Pending. None of these records constitutes release or archive acceptance.
