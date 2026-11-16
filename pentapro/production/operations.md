# Operations

For development run `cargo run -- --planilha "docs/Dia de Sorte-atual.xlsx"`; build with `cargo build --release`. See [interactive operations](interactive-strategies.md) for installed paths and freshness handling. There is no configured service, installation package or telemetry. Distribution and support are Pending.

At the start of a local working day, agents run `python3 scripts/atualizar_resultados.py` on Linux; `--force` retries after a later draw. Network, Python 3 and write access to `docs/` are required. On failure report it and retain the previous copy; no background scheduler is configured. The user-supplied workbook is preserved.

The updater validates contiguous contests and draw fields before publication. The previous successful contest and readable local history prevent regression. Use the contest reported by the updater to identify the snapshot and treat fallback data as potentially stale. A process lock serializes updater executions. A failed metadata write can leave a validated workbook in place and is reported separately so the next run can retry.

The updater rejects duplicated shared-string header labels before replacing the current workbook. This is an additional validation, not a proof of equivalence between the Python and Rust validators for all XLSX inputs.
