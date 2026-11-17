# Operation and distribution

IA-01 reads a local snapshot. The existing development updater is separate; launching the app does not invoke it automatically. An unavailable network must not silently mark an old file current. Product policy allows continuation on known-stale or unknown-freshness valid data with a clear warning and explicit user choice. Error reporting identifies source and latest contest where known.

Linux desktop is the target for this stage; Windows and macOS may be revisited later. Supported versions, packaging, installation outside the repository, accessibility environment and release process remain pending. Validate terminal handling and workbook availability on each target before a release claim. Terminal recovery after interrupted execution must be exercised. There is no external service deployment in this scope.

Planned setup: provide the workbook at the platform-specific default path documented in [PROJECT](../project/interactive-strategies.md), or launch with `--planilha PATH`. From the repository root, `cargo run -- --planilha "docs/Dia de Sorte-atual.xlsx"` selects the updater's workbook and its adjacent sidecar. For installed use, create the appropriate user data directory and copy both that workbook and `.dia-de-sorte-atualizacao.json` into it when preserving provenance; there is no automatic synchronization. Partial or mismatched copies are treated as freshness unknown. These invocations have been exercised on Linux with recorded results in PROOF; installation in an end-user environment, packaging and release remain pending.

Planned series analysis is entirely local to the selected snapshot. The interface must expose sample sizes, missing observations, mathematical preconditions and right-censored waits so that an operator can interpret unavailable values; cancel remains available while long category series are processed. No additional service operation or network dependency is introduced.

The Portuguese root README now documents the development invocation, local data locations, updater sidecar, freshness warning and keyboard navigation. It distinguishes development use from distribution and preserves all existing README sections. The `AGENTS.md` bootstrap-only description was updated to point to the actual development binary and remaining evidence.

2026-11-16: Linux implementation and integration checks are recorded in PROOF. Packaging, installation in a user environment and release have not been performed.

The implemented Linux menu revision has local pseudo-terminal and automated evidence in PROOF. Installation and release remain separate.

Historical: the single-line menu-header correction was formerly pending. It is now implemented and checked on Linux; no new installation or release behavior is specified. The startup preflight only reads the selected local workbook and its adjacent sidecar; it does not download or install data. Final evidence and limits are in [PROOF](../proof/interactive-strategies.md#peer-review-follow-up-closure--2026-11-16).

The single-line header correction was exercised on Linux; release and distribution remain separate.
