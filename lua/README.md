# Lua

Os projetos são feitos em ordem: o Projeto 2 só começa quando o Projeto 1 tiver todas as fases concluídas.

| #   | Projeto                                              | Fases  | Código     | Fase atual |
| --- | ---------------------------------------------------- | ------ | ---------- | ---------- |
| 1   | [API de tarefas com regras em sandbox](docs-api/PRD.md)   | 0 a 15 | `lua/api/` | —          |
| 2   | [TUI de git no estilo lazygit](docs-tui/PRD.md)           | 0 a 12 | `lua/tui/` | —          |

Exercícios que não pertencem a nenhum projeto (oito rainhas, CLI da Fase 0) ficam em `lua/exercicios/`.

## Estrutura de cada projeto

- `docs-<projeto>/PRD.md`: visão, contrato, fases e critérios.
- `docs-<projeto>/fase-N/PLANO.md`: plano detalhado da fase, pedido ao Claude no início dela.
- `docs-<projeto>/fase-N/NOTAS.md`: minhas respostas, por escrito, aos critérios da fase.
- `<projeto>/`: código.
