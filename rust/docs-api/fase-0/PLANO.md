# Plano da Fase 0 - Fundamentos da linguagem

Referência: [PRD](../PRD.md), Fase 0.

## Conceitos centrais

1. **Ownership:** cada valor tem um único dono. Atribuir ou passar para uma função **move** o valor, a menos que o tipo seja `Copy` (inteiros, `bool`, `char`).
2. **Empréstimo:** `&T` permite muitos leitores ao mesmo tempo. `&mut T` permite um único escritor, sem nenhum leitor junto. O compilador checa isso em tempo de compilação.
3. **`String` vs `&str`:** a `String` é dona do texto e pode crescer. O `&str` é uma visão emprestada de um texto que pertence a outro.
4. **Expressões:** quase tudo devolve valor (`if`, `match`, blocos). Um `;` no fim transforma a expressão em instrução, que devolve `()`.
5. **Structs e `impl`:** dados e métodos ficam separados. O primeiro parâmetro do método diz o que ele faz com a instância: `&self` lê, `&mut self` altera e `self` consome.
6. **Enums com dados e `match`:** cada variante pode carregar dados diferentes, e o `match` precisa cobrir todas. `Option` substitui `null`/`undefined`.

## Ordem de leitura e prática

| #   | Leitura                                 | Prática                                                                                                                                                         |
| --- | --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | Cap. 1 inteiro                          | Instalar com `rustup`. A edição 2024 exige Rust 1.85 ou mais novo; confira com `rustc --version`. Rodar `rustlings init`                                        |
| 2   | Cap. 2                                  | Digitar o jogo de adivinhação **sem copiar e colar**. Depois, quebrar de propósito: tirar o `mut`, tirar o `.trim()`, trocar o tipo do `parse`, e ler cada erro |
| 3   | Cap. 3 (3.1 a 3.5)                      | Rustlings `01_variables` a `04_primitive_types`                                                                                                                 |
| 4   | Cap. 4 (4.1 a 4.3), **lido duas vezes** | Rustlings `06_move_semantics`. Para o `05_vecs`, leia antes só a 8.1                                                                                            |
| 5   | Cap. 5 (5.1 a 5.3)                      | Rustlings `07_structs`                                                                                                                                          |
| 6   | Cap. 6 (6.1 a 6.3)                      | Rustlings `08_enums`                                                                                                                                            |
| 7   | (nenhuma)                               | CLI de tarefas, em passos abaixo                                                                                                                                |
| 8   | (nenhuma)                               | Responder os critérios por escrito                                                                                                                              |

O jogo de adivinhação usa o crate `rand`. Ele é exercício do livro, não do projeto, então a lista de crates permitidos do PRD não se aplica aqui.

## CLI de tarefas, passo a passo

Pastas: `rust/exercicios/advinhacao` e `rust/exercicios/tarefas-cli`.

Escopo: um loop que lê comandos do terminal (adicionar com título, listar, iniciar, concluir, remover pelo número, sair). Tudo fica em memória, num `Vec`. Nada de `HashMap` nem traits ainda: isso é das fases seguintes.

1. **Modelo:** `enum Status` com as três variantes do contrato e `struct Tarefa` com id numérico, título, descrição opcional e status. Crie algumas tarefas fixas no `main` e imprima.
2. **Exibição:** escreva uma função que transforma um `Status` em texto usando `match`. Decida se ela recebe o status emprestado ou por valor, e saiba justificar.
3. **Comandos como enum com dados:** crie um `enum Comando` em que cada variante carrega o que precisa, por exemplo o título ou o número. Escreva uma função que transforma a linha digitada num `Comando`, com uma variante para entrada inválida. Esta é a peça que cumpre o critério do `match` com dados.
4. **Loop:** leia do terminal como no capítulo 2, converta para `Comando` e despache com `match`.
5. **Mutação:** agrupe a lista e o próximo id numa struct com métodos. Escolha com cuidado entre `&self` e `&mut self`. A busca por número devolve `Option`, e "tarefa não encontrada" é uma mensagem, não um `panic!`.
6. **Regras de status:** pense no que acontece ao concluir uma tarefa já concluída e trate o caso com `match`.
7. **Limpeza:** rode `cargo fmt` e `cargo clippy` até zerar os avisos.

## Armadilhas comuns para quem vem de TypeScript

- **`read_line` acrescenta ao buffer, não substitui.** Num loop, sem limpar a `String`, o segundo comando vem grudado no primeiro. Essa armadilha pega quase todo mundo.
- **O `\n` no fim da linha:** sem `trim`, `"listar\n"` não casa com `"listar"`.
- **`let b = a` com `String` invalida `a`.** No TS as duas variáveis apontam para o mesmo objeto; aqui o valor muda de dono.
- **Alterar o `Vec` enquanto itera sobre ele** dá erro de empréstimo. É o compilador impedindo um bug que no JS passaria em silêncio.
- **Variante de enum não é string.** Não existe `Status.Pendente === "pendente"`; a conversão para texto é um `match` escrito por você.
- **`_ =>` em enum seu esconde variantes novas.** Prefira listar todas, para o compilador avisar quando você acrescentar uma.
- **Não há conversão implícita de números.** Índice é `usize`, e o `parse` precisa saber o tipo de destino.
- **Ponto e vírgula na última linha de uma função** que deveria devolver valor gera um erro de tipo esquisito (`expected X, found ()`).
- **`println!("{}", tarefa)` não compila numa struct sua.** Por enquanto use `#[derive(Debug)]` com `{:?}`, ou formate campo a campo; o `Display` vem no capítulo 10.
- **Tentação de `clone()` para calar o compilador.** É aceito nesta fase, mas anote onde usou. Na revisão vamos ver quais dava para trocar por um empréstimo.

## Para se testar antes de marcar os critérios

Responda por escrito em `rust/docs-api/fase-0/NOTAS.md`:

1. Por que `let s2 = s1;` invalida `s1` quando é `String`, mas não quando é `i32`?
2. Uma função que só imprime o título deveria receber `String`, `&String` ou `&str`? Por quê?
3. Por que o Rust proíbe ter um `&mut` e um `&` ao mesmo tempo? Que bug do JS isso evita?
4. Em que difere um `enum` do Rust de um union type discriminado do TS? E em que se parece?
5. O que muda se o seu método de concluir tarefa receber `self` em vez de `&mut self`?

## Critérios para avançar (do PRD)

- [ ] Explico com minhas palavras a diferença entre mover, emprestar (`&`, `&mut`) e clonar.
- [ ] Sei quando usar `String` e quando usar `&str`.
- [ ] Uso `match` em um enum que carrega dados.
- [ ] `cargo clippy` roda sem avisos na CLI.

Quando terminar, mande o código com o modelo de revisão do PRD.
