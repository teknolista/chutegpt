# Product overview

ChuteGPT is a terminal application for exploring historical results of Caixa's Dia de Sorte. The owner-confirmed long-term goal is to apply numerical, combinatorial and statistical methods to historical draws, identify patterns and investigate whether any approach can estimate future draws accurately. This is a research goal, not evidence that historical patterns predict future results.

## Current product scope

The implemented Stage 1 experience loads a selected local workbook and offers 22 individually selected descriptive historical analyses. It presents labeled results, sample context, progress, cancellation and keyboard navigation in Brazilian Portuguese. Its observable behavior and strategy contracts are specified in [interactive analysis](interactive-strategies.md), [numerical contracts](numerical-contracts.md) and [series statistics](series-statistics.md); implementation evidence and limits are in [PROOF](../proof/interactive-strategies.md).

The application does not currently generate bets or candidates, combine strategies, simulate returns, or claim predictive advantage. Predictive evaluation, user-value validation, distribution and release remain separate and unverified. Linux desktop is the target of this stage; Windows and macOS are deferred to a possible future stage.

The PRODUCT contracts describe user-visible behavior and domain rules independently of programming language or framework. Selected implementation technologies and architecture belong in PROJECT. Lottery rules, their dates and source limitations are documented in [the Dia de Sorte reference](../../docs/dia-de-sorte-referencia.md).
