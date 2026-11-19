## Context

The Caixa page's download button calls its public results download endpoint. The originally supplied workbook remains untouched.

## Goals / Non-Goals

Download at the beginning of work, cache one successful update per local day, allow forced refresh, reject invalid or older files, and preserve previous data on network or validation failure. No unattended scheduler or application data import is included.

## Decisions

Use Python's standard library for HTTP, Excel container validation, hashing and atomic replacement. Keep the latest downloaded copy and successful-update metadata in ignored files under docs. Do not treat a same-day download as proof that Caixa has published a later draw. Check ZIP integrity and bounded size, chronological draw dates, valid months, sorted distinct numbers and contiguous IDs from contest 1. Check the prior contest frontier before replacement. These checks establish structural consistency within the download; publisher freshness still depends on Caixa. Serialize updater runs with an OS file lock. The metadata hash identifies the exact validated workbook and a failed metadata write requires a retry.
