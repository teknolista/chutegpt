## Why

Development requires a recent official Dia de Sorte workbook at the start of each working day.

## What Changes

- Provide an on-demand daily updater for the workbook distributed by the Caixa page's download service.
- Preserve the user's original workbook and keep the last valid downloaded copy when a download fails.
- Instruct agents to run the updater at the start of a working day and report errors.

## Capabilities

### New Capabilities
- `atualizacao-resultados`: refresh and validate the local official workbook.

### Modified Capabilities

None.

## Impact

Adds a Python standard-library development utility, local ignored data files, agent guidance and PentaPRO records. It does not add application features, a background scheduler or guarantees about Caixa's publication time.
