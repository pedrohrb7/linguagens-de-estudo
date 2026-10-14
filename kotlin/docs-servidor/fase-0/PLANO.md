# Plano da Fase 0 - Fundamentos da linguagem

Referência: [PRD](../PRD.md), Fase 0.

## Conceitos centrais

1. **`val` e `var`:** `val` é uma referência que não muda de lugar, não um objeto imutável. Prefira `val` sempre; `var` é exceção que precisa de motivo.
2. **Null safety:** `String` nunca é nulo; `String?` pode ser. O compilador obriga a tratar o nulo com `?.` (chamada segura), `?:` (valor padrão) ou uma checagem com `if`, que faz o _smart cast_. O `!!` desliga a checagem e lança exceção: é o `as any` do Kotlin.
3. **Expressões:** `if`, `when` e `try` devolvem valor. Um `when` usado como expressão sobre uma `sealed` precisa cobrir todos os casos.
4. **Classes:** são `final` por padrão; `data class` gera `equals`, `hashCode`, `toString`, `copy` e `componentN`; `object` é um singleton; não existe `static`, existe `companion object` ou função no nível do arquivo.
5. **`sealed interface`:** uma hierarquia fechada, conhecida pelo compilador. Cada variante pode ser uma `data class` com dados ou um `object` sem dados.
6. **Funções de extensão:** acrescentam funções a um tipo sem herdar dele. São resolvidas em tempo de compilação pelo tipo declarado, não pelo tipo real.
7. **Lambdas e coleções:** `map`, `filter`, `groupBy`, `firstOrNull` e companhia; `it` como nome implícito; `List` (só leitura) e `MutableList` são interfaces diferentes do mesmo tipo de objeto.

## Ordem de leitura e prática

| #   | Leitura                                      | Prática                                                                                                                                    |
| --- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| 1   | Cap. 1 inteiro                               | Instalar JDK 21+ e IntelliJ IDEA. Criar um projeto com `gradle init` (aplicação Kotlin, DSL Kotlin) e rodar com `./gradlew run`            |
| 2   | Cap. 2                                       | Koans: "Introduction" até "String templates". Escrever um `main` que lê linhas com `readlnOrNull()` até o fim da entrada (Ctrl+D) e ecoa cada uma                |
| 3   | Cap. 3                                       | Koans: "Introduction" até o fim (inclui lambdas, `data class`, smart casts e funções de extensão)                                          |
| 4   | Cap. 4, **seções de `sealed` e `data class` lidas duas vezes** | Koans: "Classes". Ligar ktlint e detekt no `build.gradle.kts` e fazer o `./gradlew check` passar no projeto vazio      |
| 5   | Cap. 5                                       | Koans: "Collections" até "Filter map" (lambdas passadas para funções de coleção)                                                           |
| 6   | Cap. 6                                       | Koans: "Collections" até o fim, incluindo "Sequences"                                                                                      |
| 7   | Cap. 7                                       | Reler os exercícios dos Koans procurando onde dava para trocar um `if (x != null)` por `?.let` ou `?:`, e onde **não** valia a pena        |
| 8   | (nenhuma)                                    | CLI de chat, em passos abaixo                                                                                                              |
| 9   | (nenhuma)                                    | Responder os critérios por escrito                                                                                                         |

Os Koans ficam em `kotlin/exercicios/koans` (ou no navegador, em play.kotlinlang.org). Eles são exercício da linguagem, não do projeto, então a lista de dependências permitidas do PRD não se aplica a eles.

## CLI de chat, passo a passo

Pasta: `kotlin/exercicios/papo-cli`.

Escopo: um loop que lê comandos do terminal e simula um chat local, com vários usuários e salas, tudo em memória. Comandos: `/usuario <nome>` (troca quem está falando), `/criar <sala>`, `/entrar <sala>`, `/sair`, `/salas`, `/historico`, `/fim`; qualquer linha que não começa com `/` é uma mensagem para a sala atual. Nada de coroutines nem arquivos: isso é das fases seguintes.

1. **Modelo:** `data class Mensagem` com `seq`, autor, texto e horário (`java.time.Instant`), e `data class Sala` com id, nome e a lista de mensagens. Decida quais campos são `val`. Crie algumas salas fixas no `main` e imprima.
2. **Comandos como `sealed`:** crie uma `sealed interface Comando` com uma variante por comando, cada uma carregando o que precisa (`data class Entrar(val sala: String)`, `data object Sair`...), e uma variante para entrada inválida com o motivo. Escreva a função que transforma a linha digitada num `Comando`. Ela nunca lança exceção.
3. **Despacho com `when`:** use o `when` como expressão sobre o `Comando`, sem `else`. Acrescente uma variante nova depois e veja o compilador apontar onde falta tratar.
4. **Estado:** agrupe salas, usuário atual e sala atual numa classe. A sala atual é nula quando o usuário não entrou em nenhuma; mandar mensagem nesse estado é um aviso, não uma exceção. Resolva sem `!!` e sem `lateinit`.
5. **`seq` por sala:** cada sala numera as próprias mensagens a partir de 1. É o mesmo `seq` do contrato do servidor.
6. **Imutabilidade:** experimente guardar as mensagens numa `List` (só leitura) e criar uma nova lista a cada envio, e depois numa `MutableList` privada exposta como `List`. Anote o que cada escolha muda.
7. **Extensões e formatação:** uma função de extensão que formata uma `Mensagem` para o terminal (ex.: `pedro 14:03  texto`); `/historico` mostra as últimas 20 mensagens da sala atual usando operações de coleção, sem `for` com índice.
8. **Limpeza:** rode `./gradlew check` até zerar os avisos do ktlint e do detekt.

## Armadilhas comuns para quem vem de TypeScript

- **`==` é comparação estrutural (chama `equals`) e `===` é identidade.** É o oposto da intuição do JS, onde `===` é o "certo".
- **`List` não é imutável.** É só uma visão de leitura: o mesmo objeto pode ser uma `MutableList` em outro lugar e mudar debaixo dos seus pés.
- **`!!` para calar o compilador.** Funciona até o dia em que lança `NullPointerException`. Na CLI ele é proibido.
- **`lateinit var` como fuga de nullability.** É o `let x!: T` do TS; serve para injeção e testes, não para "vou preencher depois".
- **`readln()` lança exceção no fim da entrada.** Use `readlnOrNull()` e trate o `null` como "acabou".
- **`"abc".toInt()` lança exceção.** `toIntOrNull()` devolve `null`, que combina com `?:`.
- **`else` num `when` sobre `sealed` esconde variantes novas.** Liste todas para o compilador avisar quando você acrescentar uma.
- **Classes são `final` por padrão.** Herança precisa de `open`, e quase sempre o que você quer é uma `sealed` ou composição.
- **Não existe `static`.** Funções soltas no arquivo são o normal; `companion object` é para quando a função precisa mesmo morar na classe.
- **Extensão não sobrescreve membro.** Se a classe já tem um método com a mesma assinatura, o membro ganha, sem aviso.
- **`it` em lambdas aninhadas.** Dois `it` em níveis diferentes confundem quem lê; dê nome aos parâmetros.
- **`data class` com `var`.** Funciona, mas quebra a ideia de `copy` e deixa o `hashCode` mudar enquanto o objeto está dentro de um `HashSet`.

## Para se testar antes de marcar os critérios

Responda por escrito em `kotlin/docs-servidor/fase-0/NOTAS.md`:

1. Qual a diferença entre `val lista = mutableListOf<Int>()` e `var lista = listOf<Int>()`? O que cada um permite mudar?
2. O que o compilador faz com `if (sala != null) sala.nome` que ele não faz quando `sala` é uma `var` de uma classe?
3. Em que difere uma `sealed interface` do Kotlin de uma union discriminada do TS? E em que se parece?
4. O que uma `data class` gera, e por que não faz sentido ter uma `data class` sem nenhuma propriedade no construtor?
5. Por que uma função de extensão chamada sobre uma variável declarada como `Any` não enxerga a extensão definida para `String`, mesmo que o valor seja uma `String`?

## Critérios para avançar (do PRD)

- [ ] Explico o que `?.`, `?:` e `!!` fazem e por que a CLI não tem nenhum `!!`.
- [ ] Explico o que uma `data class` gera e por que `copy` combina com `val`.
- [ ] Uso `when` exaustivo sobre uma `sealed interface` sem `else`.
- [ ] Explico por que `List` do Kotlin é só leitura, mas não imutável.
- [ ] `./gradlew check` passa sem avisos do ktlint e do detekt.

Quando terminar, mande o código com o modelo de revisão do PRD.
