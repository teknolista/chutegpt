# Environments

Development requires Rust and Cargo with rustfmt and clippy. Historical Linux bootstrap checks are in [verification](../proof/verification.md); current interactive checks are in [interactive verification](../proof/interactive-strategies.md). Linux CI and specific supported distributions are pending; Windows and macOS are deferred to a possible future stage.

Interactive analysis was built and exercised on Linux x86_64 with rustc/Cargo 1.100.0. Windows and macOS targets were not installed and their builds and runtime behavior remain unverified; see [platform evidence](../proof/interactive-strategies.md#integrated-linux-verification--2026-11-16).
