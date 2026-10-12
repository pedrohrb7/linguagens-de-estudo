# Rust

Os projetos são feitos em ordem: o Projeto 2 só começa quando o Projeto 1 tiver todas as fases concluídas.

| #   | Projeto                                       | Fases  | Código          | Fase atual |
| --- | --------------------------------------------- | ------ | --------------- | ---------- |
| 1   | [API REST de tarefas](docs-api/PRD.md)             | 0 a 13 | `rust/api/`     | 0          |
| 2   | [TUI cliente da API](docs-tui/PRD.md)              | 0 a 12 | `rust/tui/`     | —          |

Exercícios que não pertencem a nenhum projeto (jogo de adivinhação, CLI da Fase 0) ficam em `rust/exercicios/`.

## Estrutura de cada projeto

- `docs-<projeto>/PRD.md`: visão, contrato, fases e critérios.
- `docs-<projeto>/fase-N/PLANO.md`: plano detalhado da fase, pedido ao Claude no início dela.
- `docs-<projeto>/fase-N/NOTAS.md`: minhas respostas, por escrito, aos critérios da fase.
- `<projeto>/`: código.
