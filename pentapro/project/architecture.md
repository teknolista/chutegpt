# Architecture

The current Rust binary in `src/main.rs` uses a local XLSX adapter, independent numerical modules and a Ratatui terminal interface; see [interactive analysis design](interactive-strategies.md) for boundaries and dependencies. The bootstrap previously had no dependencies. Release distribution and future capabilities remain pending.

Development utility: `scripts/atualizar_resultados.py` uses the Caixa page download endpoint via Python standard library and stores a separate current workbook and local state under `docs/`. This is not an application feature; see the [OpenSpec change](../../openspec/changes/atualizar-resultados-diarios/proposal.md).
