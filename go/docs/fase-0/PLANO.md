# Plano da Fase 0 - Fundamentos

Referência: [PRD](../PRD.md), Fase 0.

## Conceitos centrais

1. **Estrutura de um programa:** `package main`, `func main`, imports. Import ou variável local sem uso é **erro de compilação**, não aviso.
2. **Declarações:** `var` (em qualquer lugar, com ou sem valor inicial), `:=` (só dentro de função, declara e infere o tipo) e `const` (valor de compilação, que pode ser "sem tipo"). Escopo de bloco e sombreamento.
3. **Valor zero:** toda variável nasce com um valor útil (`0`, `""`, `false`, `nil`). Não existe `undefined` nem "variável não inicializada".
4. **Tipos básicos:** inteiros com tamanho (`int`, `int64`, `uint8`...), `float64`, `bool`, `string`. Nenhuma conversão implícita entre eles.
5. **Strings, bytes e runes:** uma `string` é uma sequência imutável de bytes em UTF-8. Um `rune` é um ponto de código Unicode. `len` conta bytes, não caracteres.
6. **Ponteiros:** existem, sem aritmética. `&` pega o endereço e `*` lê através dele.
7. **Controle de fluxo:** só existe `for` (ele também faz o papel do `while`); `if` e `switch` aceitam uma instrução de inicialização; `switch` não "cai" para o próximo caso.

## Ordem de leitura e prática

| #   | Leitura                                                                       | Prática                                                                                                                                       |
| --- | ----------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | (instalação)                                                                  | Instalar a versão estável atual do Go; conferir com `go version`; instalar a extensão oficial de Go no editor (traz o gopls)                    |
| 2   | Tour of Go: "Packages, variables, and functions"                              | Fazer todos os exercícios do Tour nesta seção                                                                                                 |
| 3   | Livro 1.1 a 1.3 (Hello World, argumentos, linhas duplicadas)                  | Digitar o `echo` e as três versões do `dup` **sem copiar e colar**. O `dup` é o esqueleto do seu contador de palavras                          |
| 4   | Livro 1.4 a 1.8, passando rápido                                              | Só ler: GIF, fetch, servidor web. Concorrência (1.6) e HTTP (1.7) voltam nas Fases 4 e 5                                                      |
| 5   | Tour of Go: "Flow control statements"                                         | Exercício "Loops and Functions"                                                                                                               |
| 6   | Livro cap. 2 inteiro (Program Structure)                                      | Atenção a 2.3.2 (ponteiros e o pacote `flag`), 2.5 (declarações de tipo) e 2.7 (escopo)                                                       |
| 7   | Livro cap. 3 (Basic Data Types), com 3.3 (complexos) só de passagem           | **3.5 (Strings) lido duas vezes**: UTF-8, runes e os pacotes `strings`, `unicode` e `strconv`. Em 3.6, entender `iota` e constantes sem tipo   |
| 8   | Tour of Go: "More types: structs, slices, and maps"                           | Exercícios "Slices" e "Maps". Slices e maps são aprofundados no cap. 4 (Fase 1); aqui basta o uso básico                                      |
| 9   | (nenhuma)                                                                     | Contador de palavras, em passos abaixo                                                                                                        |
| 10  | (nenhuma)                                                                     | Responder os critérios por escrito                                                                                                            |

## Contador de palavras, passo a passo

Pasta: `go/fase-0/contador`. Rode `go mod init contador` uma vez para o `go build` funcionar; módulos são estudados de verdade na Fase 2.

Comportamento final: `contador [-n 10] [arquivo...]`. Sem arquivos, lê da entrada padrão. Imprime as N palavras mais frequentes, da mais para a menos frequente.

1. **Ler da entrada padrão:** como no `dup1`, use um `bufio.Scanner` e, por enquanto, conte linhas em vez de palavras. Confira se o programa termina com Ctrl+D.
2. **Quebrar em palavras:** troque a contagem de linhas pela de palavras. Pesquise as duas opções (`strings.Fields` em cada linha ou mudar a forma como o `Scanner` divide o texto) e escolha uma. Depois da leitura, cheque se o `Scanner` terminou com erro.
3. **Normalizar:** decida o que é "a mesma palavra". "Casa", "casa" e "casa," devem contar juntas. Use os pacotes `strings` e `unicode` e garanta que palavras acentuadas ("ação", "você") continuem inteiras.
4. **Ler de arquivos:** como no `dup2`, use os argumentos para abrir arquivos. Um arquivo que não existe gera mensagem em `stderr` e o programa segue para os próximos, mas termina com código de saída diferente de zero.
5. **Ordenar:** a ordem de um `map` não é garantida. Monte uma slice de pares (palavra, contagem), ordene pela contagem decrescente e, em caso de empate, por ordem alfabética, para a saída ser sempre a mesma.
6. **Opção `-n`:** use o pacote `flag` (seção 2.3.2) para o tamanho do ranking, com padrão 10. Valor menor que 1 é erro.
7. **Saída alinhada:** use os verbos de largura do `Printf` para alinhar palavra e contagem em colunas.
8. **Testes manuais:** texto vindo de pipe (`echo ... | go run .`), um e vários arquivos, arquivo inexistente, entrada vazia, texto com acentos e pontuação, `-n 0`.
9. **Limpeza:** `gofmt -l .` não lista nada e `go vet ./...` não gera avisos.

## Armadilhas comuns para quem vem de TypeScript

- **Import ou variável sem uso não compila.** Durante testes, a tentação de atribuir a `_` só para calar o compilador esconde código morto.
- **`:=` dentro de um bloco cria uma variável nova** com o mesmo nome, em vez de atribuir à de fora. É o bug de sombreamento clássico em Go, comum com `err`.
- **`len("ação")` é 6, não 4.** Indexar uma string devolve um byte, não um caractere. Para percorrer caracteres, use `range`, que entrega runes.
- **Escrever num `map` nil dá `panic`.** `var m map[string]int` declara, mas não cria o map; ler dele funciona, escrever não.
- **A ordem de `range` sobre um `map` muda a cada execução** de propósito. Se a sua saída mudar entre execuções iguais, faltou ordenar.
- **Divisão de inteiros trunca** e não há conversão implícita: um percentual exige converter os operandos para `float64` antes de dividir.
- **Não existe ternário nem `while`.** Use `if/else` e `for` com condição.
- **`switch` não precisa de `break`.** Cada caso termina sozinho; o contrário (`fallthrough`) é explícito e raro.
- **Erro ignorado não é erro tratado.** `os.Open` e o `Scanner` devolvem erros que precisam ser checados, não descartados.
- **Concatenar string em loop** cria uma string nova a cada vez. Para montar texto grande, a seção 3.5.4 mostra a alternativa.
- **Formatação não se discute:** o `gofmt` decide (tabs, chaves, espaços). Configure o editor para rodá-lo ao salvar.

## Para se testar antes de marcar os critérios

Responda por escrito em `go/docs/fase-0/NOTAS.md`:

1. Onde posso usar `var x int`, `x := 0` e `const x = 0`? O que cada um permite e proíbe? Por que `:=` não funciona fora de uma função?
2. Qual o valor zero de `string`, `int`, `*int` e `map[string]int`? O que acontece ao ler e ao escrever num `map` nil?
3. Por que `len("ação")` não é 4? Como contar caracteres de verdade?
4. O que é uma constante "sem tipo" e por que `const x = 1` pode ser usada tanto como `int` quanto como `float64`?
5. Num `if` que usa `:=` para receber `err`, o que acontece com uma variável `err` declarada antes do `if`?
6. Por que a saída do seu contador seria diferente a cada execução sem a etapa de ordenação?

## Critérios para avançar (do PRD)

- [ ] Explico a diferença entre `var x int`, `x := 0` e uma constante.
- [ ] Explico o que é o valor zero de `string`, `int`, ponteiro e `map`, e o que acontece ao escrever num `map` nil.
- [ ] `gofmt` e `go vet` passam sem alterações nem avisos.

Quando terminar, mande o código com o modelo de revisão do PRD.
