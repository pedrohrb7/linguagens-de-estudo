# Kotlin

Os projetos são feitos em ordem: o Projeto 2 só começa quando o Projeto 1 tiver todas as fases concluídas.

| #   | Projeto                                                        | Fases  | Código              | Fase atual |
| --- | -------------------------------------------------------------- | ------ | ------------------- | ---------- |
| 1   | [Servidor de chat em tempo real](docs-servidor/PRD.md)         | 0 a 13 | `kotlin/servidor/`  | -          |
| 2   | [App Multiplatform offline-first](docs-app/PRD.md)             | 0 a 12 | `kotlin/app/`       | -          |

Os dois projetos formam o produto **Papo**: o Projeto 1 é o servidor, e o Projeto 2 é o app de desktop e Android que conversa com ele.

Exercícios que não pertencem a nenhum projeto (Kotlin Koans, CLI da Fase 0) ficam em `kotlin/exercicios/`.

## Estrutura de cada projeto

- `docs-<projeto>/PRD.md`: visão, contrato, fases e critérios.
- `docs-<projeto>/fase-N/PLANO.md`: plano detalhado da fase, pedido ao Claude no início dela.
- `docs-<projeto>/fase-N/NOTAS.md`: minhas respostas, por escrito, aos critérios da fase.
- `<projeto>/`: código.
