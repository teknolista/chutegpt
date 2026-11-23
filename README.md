<p align="center">
  <img src="chutegpt.png" alt="Logo do ChuteGPT" width="200"/>
</p>

<h1 align="center">ChuteGPT</h1>

<p align="center">
  <strong>🍀 Aplicativo de linha de comando em Rust para explorar estatísticas da loteria Dia de Sorte.</strong><br>
  <em>Em desenvolvimento para desktop Linux</em>
</p>

---

O **ChuteGPT** é um aplicativo CLI (para terminais) dedicado à análise de dados e estatísticas da loteria Dia de Sorte, da Caixa Econômica Federal. O projeto explora matemática, combinatória, computação numérica e estratégias de análise histórica; novos métodos e funcionalidades continuarão sendo especificados e validados.

O objetivo do ChuteGPT é aplicar processamento numérico, combinatória e técnicas estatísticas aos resultados anteriores da loteria Dia de Sorte, identificando padrões nas dezenas para investigar formas de estimar o próximo resultado com a maior acurácia possível. As análises devem utilizar a versão mais recente disponível dos resultados oficiais da Caixa e avaliar suas estratégias com dados históricos e métodos de validação apropriados.

> 📚 **Evolução do projeto:** a base Rust e o fluxo de especificação foram inicializados. A versão em desenvolvimento apresenta um menu interativo com análises históricas; geração de apostas e simulações permanecem para uma etapa futura. A integração e a compatibilidade com outras plataformas ainda estão em verificação.

## 📂 Documentação

- [Pesquisa sobre o Dia de Sorte e fontes oficiais](docs/dia-de-sorte-referencia.md)
- [Análise das estratégias das aplicações legadas](docs/analise-estrategias-legado.md)
- [Especificações e situação do projeto (PentaPRO)](pentapro/process/index.md)
- [Cobertura da especificação PentaPRO](pentapro/process/specification-coverage.md)
- [Orientações para agentes](AGENTS.md)

## 🎓 Funcionalidades Principais

A versão em desenvolvimento oferece 22 análises individuais: paridade, repetição, recorrência histórica, sequências adjacentes, maior sequência consecutiva, soma, raiz digital, dispersão das dezenas, espaçamento, decenário, colunas, matriz, ciclos de 27 e 31 dezenas, ordinal, dezenas individuais, mês da sorte, frequência e ausência, distância, média das raízes, dispersão entre frequência e atraso, premiação por dia da semana, séries de mês e premiação.

Os resultados são apresentados em gráficos e tabelas navegáveis, com detalhes estatísticos das séries quando aplicáveis. A aplicação informa o intervalo de concursos, os dados ausentes e o progresso; é possível cancelar o processamento e manter o resultado na tela para análise. Os resultados descrevem o histórico e não garantem ganhos ou a previsão de sorteios futuros.

## 🛠️ Tecnologias Utilizadas

- **Linguagem:** Rust, edição 2024.
- **Compilação e gerenciamento do projeto:** Cargo.
- **Especificação:** PentaPRO.
- **Bibliotecas externas:** Ratatui e Crossterm para o terminal; Calamine para leitura Excel; Chrono, serde_json, SHA-2 e zip para datas, metadados e validação dos dados.
- **Sistema pretendido nesta etapa:** desktop Linux; versões e arquiteturas suportadas ainda serão definidas. Windows e macOS poderão ser considerados em uma etapa futura.

### Atualização dos resultados para desenvolvimento

No início do trabalho em cada dia, execute `python3 scripts/atualizar_resultados.py` no Linux. O script requer Python 3 e consulta o serviço usado pelo botão da Caixa; salva a cópia em `docs/Dia de Sorte-atual.xlsx` sem substituir `docs/Dia de Sorte.xlsx`. Ele evita novo download no mesmo dia; use `--force` após um sorteio para consultar novamente. Uma consulta bem-sucedida não garante que a Caixa já tenha publicado o resultado mais recente. Se o download ou a validação falhar, a cópia anterior é preservada e o erro é informado. O agente inicia essa rotina ao começar o trabalho do dia, sem agendamento em segundo plano. Antes de analisar os dados, confira o número do último concurso informado pelo script. Se a atualização falhar, uma cópia anterior pode estar desatualizada.

## 📖 Como Usar e Instalar

Ainda não há versão distribuída nem procedimento de instalação para usuários finais. Para executar a versão em desenvolvimento, instale Rust e Cargo, atualize a cópia local conforme a seção anterior e, na raiz do repositório, execute:

```sh
cargo run -- --planilha "docs/Dia de Sorte-atual.xlsx"
```

Use `cargo run -- --help` para consultar a opção disponível. Em um terminal interativo, use **↑/↓** para escolher uma estratégia e **Enter** para executá-la. A descrição detalhada da estratégia selecionada aparece à direita, em três parágrafos sobre cálculo, dados e seções de resultado; use **Page Up/Page Down** ou **H/L** para rolá-la. O nome completo da estratégia aparece nessa área; o caminho da planilha e o estado de verificação ficam no cabeçalho, sem repetição na descrição. O cabeçalho mostra o nome do aplicativo, a fonte e seu estado de verificação nessa ordem: em uma única linha quando o texto cabe e, em terminais estreitos, quebrado em mais linhas para que o caminho completo permaneça visível. Se o caminho exceder o espaço reservado do cabeçalho, ele informa o limite; pressione **P** para abrir a visualização rolável do caminho completo, use **↑/↓**, **Page Up/Page Down** ou **Home/End** para navegar e **Esc** ou **P** para voltar sem perder a tela atual. Enquanto a verificação inicial em segundo plano não termina, o cabeçalho informa que a fonte ainda não foi verificada; depois, mostra o estado real da cópia selecionada. Se o arquivo for lido e rejeitado, o cabeçalho informa **falha na verificação**, o motivo aparece na descrição da estratégia e **Enter** tenta carregar novamente. No resultado, use **Tab/Shift+Tab** para mudar de seção, **←/→** para escolher uma categoria, **S** ou **Enter** para ver os detalhes da série e **↑/↓** para rolar. Em terminais baixos com cabeçalhos de várias linhas, o gráfico reduz sua altura para manter os detalhes legíveis. **Esc** volta à tela anterior ou cancela o processamento; **Q** encerra no menu ou no resultado. A tela de resultados permanece disponível até a navegação. O aplicativo requer um terminal de pelo menos 55 colunas por 19 linhas; abaixo disso, uma mensagem solicita o redimensionamento e preserva o estado, restaurando a visão quando o tamanho voltar ao mínimo.

O aplicativo lê a planilha local e não faz download ao iniciar. Se o registro de atualização estiver ausente, não corresponder à planilha ou for anterior ao dia local, ele avisa e permite continuar ou voltar. Uma verificação no mesmo dia não garante que um sorteio posterior já tenha sido publicado pela Caixa.

## 🌐 Personalização

Além do caminho da planilha, outras opções de personalização e configuração ainda serão definidas.

### Métodos de configuração

O caminho da planilha pode ser informado com `--planilha CAMINHO`. Caminhos relativos são resolvidos a partir do diretório de execução.

### Opções suportadas

Sem `--planilha`, o programa procura `ChuteGPT/Dia de Sorte-atual.xlsx` no diretório de dados do usuário: `$XDG_DATA_HOME` absoluto ou `$HOME/.local/share` no Linux; `$HOME/Library/Application Support` no macOS; e `%LOCALAPPDATA%` no Windows. Os dois últimos caminhos registram o comportamento previsto anteriormente e não constituem suporte validado nesta etapa. Para preservar o registro de atualização ao copiar a planilha do projeto, copie também `docs/.dia-de-sorte-atualizacao.json` para o mesmo diretório da cópia. Sem um registro correspondente, a atualização é informada como desconhecida.

## 💻 Para Desenvolvedores

Instale o ambiente Rust com o Cargo e os componentes `rustfmt` e `clippy`. Na raiz do repositório, execute:

```sh
cargo run -- --planilha "docs/Dia de Sorte-atual.xlsx"
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test -- --test-threads=1
cargo build --release
```

A inicialização foi verificada em Linux, conforme [registro de verificação](pentapro/proof/verification.md). A evidência da implementação interativa está em [verificação das estratégias](pentapro/proof/interactive-strategies.md). Windows e macOS poderão ser considerados em uma etapa futura.

## 📚 Para quem é o ChuteGPT

O público, os casos de uso e as jornadas ainda serão definidos. O objetivo inicial é apoiar a exploração de estatísticas relacionadas ao Dia de Sorte por meio de um terminal.

## 🤝 Contribuindo

O fluxo de contribuição e as orientações para envio de alterações ainda serão definidos. O projeto utiliza especificações em `pentapro/` e alterações acompanhadas em `pentapro/changes/`, conforme as [orientações para agentes](AGENTS.md).

## 🔒 Privacidade dos Dados

A aplicação lê a planilha local selecionada e seus metadados de atualização; o processamento das estratégias ocorre localmente. O atualizador de desenvolvimento consulta a fonte oficial da Caixa e salva uma cópia em `docs`. A política de privacidade e eventuais formas adicionais de armazenamento ainda serão definidas.

## 📄 Licença

O repositório inclui o texto da GNU General Public License versão 3. Consulte [LICENSE](LICENSE). A identificação SPDX e eventuais condições adicionais estão pendentes de confirmação.

---

<p align="center"><small>
Copyleft © 2026 <a href="https://x.com/teknolista">Raul Bras</a> 🇧🇷<br>
Feito no Brasil, em Osasco (SP), com 🌭 🍕 e IA 🤖.
</small></p>


## Fluxo de desenvolvimento com IA

O projeto utiliza skills e perfis de agentes PentaPRO locais com o fluxo nativo em [PentaPRO](pentapro/README.md). Consulte as [orientações para agentes](AGENTS.md) para o fluxo do projeto. O [protocolo de melhoria contínua](docs/agents/continuous-improvement.md) descreve o ciclo PDSA e o [registro de lições aprendidas](docs/agents/lessons-learned.md); execuções relevantes da fábrica seguem o [guia de registros de execução](docs/agents/runs/README.md). As lições são registradas em inglês.
