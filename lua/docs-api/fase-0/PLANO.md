# Plano da Fase 0 - Fundamentos

Referência: [PRD](../PRD.md), Fase 0.

## Conceitos centrais

1. **Oito tipos, um só para estruturas:** `nil`, `boolean`, `number`, `string`, `function`, `table`, `userdata` e `thread`. Array, objeto, `Map` e `Set` são todos tabelas.
2. **Verdade e falsidade:** só `nil` e `false` são falsos. `0`, `""` e a tabela vazia são verdadeiros.
3. **`nil` apaga:** atribuir `nil` a uma chave remove a chave da tabela. Não existe "chave presente com valor `undefined`".
4. **Números com dois subtipos:** inteiro (64 bits) e float. `/` sempre devolve float; `//` é a divisão inteira (com floor). `math.type` diz qual é o subtipo.
5. **Sequências começam em 1:** `#t` só tem resposta confiável quando a tabela é uma sequência, sem buracos entre `1` e `n`.
6. **Global por padrão:** variável sem `local` vai parar na tabela global. Em Lua idiomático, tudo é `local`.
7. **Funções são valores com vários retornos:** uma função pode devolver mais de um valor, e o costume para falhas esperadas é devolver `nil, mensagem`.

## Ordem de leitura e prática

| #   | Leitura                     | Prática                                                                                                                                      |
| --- | --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | Cap. 1                      | Instalar Lua 5.4 e conferir com `lua -v`. Usar o modo interativo (`lua -i`) para testar cada exemplo do capítulo. Instalar luacheck e StyLua |
| 2   | Cap. 2                      | Digitar as oito rainhas **sem copiar e colar**. Depois, quebrar de propósito (abaixo) e ler cada erro                                        |
| 3   | Cap. 3                      | No `lua -i`: comparar `3 == 3.0`, `math.type(3)` e `math.type(3.0)`, `7 / 2` e `7 // 2`, `-7 // 2`, `math.maxinteger + 1`                    |
| 4   | Cap. 4                      | No `lua -i`: `#"é"`, `utf8.len("é")`, `"10" + 1`, `"10" .. 1`, `10 == "10"`                                                                  |
| 5   | Cap. 5, **lido duas vezes** | No `lua -i`: montar uma sequência, criar um buraco no meio com `nil` e observar `#`; comparar `table.remove` com atribuir `nil`              |
| 6   | Cap. 6                      | Escrever uma função que devolve dois valores e chamá-la no meio e no fim de uma lista de argumentos                                          |
| 7   | Cap. 7 e 8                  | Ler uma linha com `io.read` e testar o que ela devolve quando você aperta Ctrl+D                                                             |
| 8   | Exercícios                  | Pelo menos três exercícios do fim de cada capítulo de 3 a 6                                                                                  |
| 9   | (nenhuma)                   | CLI de tarefas, em passos abaixo                                                                                                             |
| 10  | (nenhuma)                   | Responder os critérios por escrito                                                                                                           |

**Quebrar as oito rainhas de propósito:**

- Tirar o `local` de uma variável e rodar o luacheck. O programa continua funcionando; é exatamente por isso que o luacheck existe.
- Trocar o laço que começa em `1` para começar em `0` e ver o que acontece com o tabuleiro.
- Chamar uma função com um argumento a menos. Observe que não há erro na chamada: o parâmetro que falta chega como `nil`, e o erro aparece mais adiante.

Não existe um Rustlings oficial para Lua. Os exercícios do fim dos capítulos do livro fazem esse papel.

## CLI de tarefas, passo a passo

Pastas: `lua/exercicios/oito-rainhas` e `lua/exercicios/tarefas-cli`.

Escopo: um laço que lê comandos do terminal (`adicionar <título>`, `listar`, `iniciar <n>`, `concluir <n>`, `remover <n>`, `sair`). Tudo fica em memória, num único arquivo. Ainda sem módulos, metatables e padrões elaborados: isso é das fases seguintes.

1. **Modelo:** cada tarefa é uma tabela com `id`, `titulo`, `descricao` (opcional) e `status`. O status é uma string com um dos três valores do contrato. Crie uma tabela que funcione como conjunto dos status válidos (status como chave, `true` como valor) e use-a para validar. Crie algumas tarefas fixas e imprima.
2. **Exibição:** escreva uma função que formata uma tarefa numa linha com `string.format`. Decida o que mostrar quando a descrição for `nil`.
3. **Leitura de comandos:** separe o comando do argumento usando só `string.find` e `string.sub` (cap. 4). Os padrões de verdade ficam para a Fase 1. Converta o número com `tonumber` e rejeite o que não for inteiro positivo (pense no que `tonumber("1.5")` e `tonumber("1e2")` devolvem).
4. **Laço:** leia linhas com `io.read`, despache cada comando com uma cadeia de `if`/`elseif` e trate o fim da entrada (Ctrl+D) como `sair`.
5. **Ids e remoção:** decida se a lista guarda as tarefas em sequência (e o id é um campo) ou se o id é a chave da tabela. Justifique a escolha por escrito, pensando no que acontece com `#` e com `ipairs` depois de uma remoção. O próximo id nunca é reaproveitado.
6. **Falhas como valores:** as funções que buscam ou alteram uma tarefa devolvem `nil, mensagem` quando algo dá errado. "Tarefa não encontrada" é uma mensagem para o usuário, não um erro de Lua.
7. **Regras de status:** pense no que acontece ao iniciar uma tarefa já concluída ou concluir uma já concluída, e trate cada caso.
8. **Limpeza:** rode `stylua` e `luacheck` até zerar os avisos.

## Armadilhas comuns para quem vem de TypeScript

- **Esquecer o `local`.** A variável vira global sem nenhum aviso do interpretador, e duas funções passam a mexer na mesma variável sem você perceber.
- **`!=` não existe.** Em Lua é `~=`. E `!` também não: a negação é `not`.
- **`+` não concatena.** `"10" + 1` dá `11` (a string é convertida em número) e `"10" .. 1` dá `"101"`. O contrário do JS.
- **`if lista then` é sempre verdadeiro para uma tabela**, mesmo vazia. Para saber se está vazia, use `#lista == 0` (numa sequência) ou `next(lista) == nil`.
- **`#` com buracos.** Depois de `t[2] = nil` numa lista de 3 itens, `#t` pode devolver 3 ou 1. As duas respostas são válidas pela especificação.
- **`ipairs` para no primeiro `nil`.** Os itens depois de um buraco simplesmente não aparecem, sem erro.
- **`io.read` devolve `nil` no fim da entrada.** Chamar um método de string nesse `nil` dá `attempt to index a nil value`.
- **`/` sempre dá float.** Um id calculado com `/` aparece como `2.0`. Para inteiros, use `//` ou mantenha um contador.
- **`#"ação"` conta bytes, não caracteres.** Dá 6, não 4. Por enquanto só observe; a contagem certa com `utf8.len` vem na Fase 8.
- **Tabelas são comparadas por identidade.** Duas tabelas com o mesmo conteúdo não são `==`, como objetos no JS.
- **Vários retornos são cortados no meio de uma lista.** Em `f(), 10` só o primeiro valor de `f()` sobrevive; em `10, f()` todos sobrevivem.
- **Não existe `continue`.** O livro mostra o padrão com `goto` no cap. 8; também dá para reorganizar o `if`.
- **Argumento a menos não é erro.** O parâmetro chega como `nil` e o problema estoura longe da chamada. Na dúvida, valide na entrada da função.

## Para se testar antes de marcar os critérios

Responda por escrito em `lua/docs-api/fase-0/NOTAS.md`:

1. Por que `if 0 then print("sim") end` imprime "sim"? Que bug isso causaria num código traduzido direto de JS?
2. O que `3 == 3.0` devolve, e o que `math.type` diz de cada lado? Por que `7 // 2` e `-7 // 2` não são simétricos?
3. Mostre, com uma sessão do `lua -i`, uma tabela em que `#` devolve um valor surpreendente, e explique por quê.
4. O que acontece se você esquecer o `local` dentro de uma função que é chamada duas vezes? Que tipo de bug isso gera?
5. Por que a busca de tarefa devolve `nil, mensagem` em vez de chamar `error`? Quando `error` faria mais sentido?
6. Que forma você escolheu para guardar as tarefas (sequência ou id como chave) e o que mudaria se escolhesse a outra?

## Critérios para avançar (do PRD)

- [ ] Explico por que `0` e `""` são verdadeiros em Lua e só `nil` e `false` são falsos.
- [ ] Explico a diferença entre `3` e `3.0` (subtipos inteiro e float), e entre `/` e `//`.
- [ ] Explico o que o operador `#` devolve numa tabela com buracos e por que não confio nele nesse caso.
- [ ] Explico por que toda variável sem `local` é global e o que isso causa.
- [ ] luacheck roda sem avisos na CLI.

Quando terminar, mande o código com o modelo de revisão do PRD.
