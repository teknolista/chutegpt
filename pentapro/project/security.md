# Security and privacy

The application reads a user-selected local workbook and sidecar as untrusted input; the current contract bounds file/decompression sizes, validates required fields and never executes workbook contents. The sidecar hash identifies bytes but does not authenticate the source. The development updater separately accesses the official endpoint. A dedicated security review, data retention, personal-data handling and legal applicability remain Pending.
