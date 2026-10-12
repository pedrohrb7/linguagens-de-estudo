# Plano da Fase 0 - O terminal na unha

Referência: [PRD](../PRD.md), Fase 0.

## Conceitos centrais

1. **O terminal é um protocolo de bytes.** O programa escreve texto misturado com sequências de controle (quase todas começando com `ESC [`, chamadas de CSI), e o terminal as interpreta para mover o cursor, mudar cores e limpar a tela. Na outra direção, cada tecla chega como um ou mais bytes.
2. **Modo canônico vs. modo cru.** Por padrão, o driver do terminal (o "tty" do kernel) guarda a linha até o Enter, ecoa o que é digitado e transforma Ctrl+C em SIGINT. No modo cru, cada byte chega na hora, sem eco nem tradução, e o programa decide tudo.
3. **O estado do terminal pertence ao terminal, não ao programa.** Se o programa morrer sem restaurar, o shell herda um terminal sem eco e sem quebra de linha. Restaurar é obrigação de todo caminho de saída.
4. **Tela alternativa.** Um segundo buffer de tela, sem histórico de rolagem, que some ao sair e devolve o conteúdo anterior. É o que vim, less e lazygit usam.
5. **Saída com buffer.** `io.write` acumula num buffer da libc; nada aparece até um `io.flush` (ou o buffer encher). Um quadro inteiro deve sair numa escrita só.
6. **Teclas com vários bytes.** Setas e teclas de função são sequências que começam com o mesmo byte do `Esc`. Sem um jeito de esperar "um pouco" pelo próximo byte, `Esc` sozinho é indistinguível do início de uma seta.

## Ordem de leitura e prática

| #   | Leitura                                                                                    | Prática                                                                                                                          |
| --- | ------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------- |
| 1   | XTerm Control Sequences: introdução e seção "C1 (8-Bit) Control Characters"                | No shell, com `printf`: limpar a tela, mover o cursor para a linha 10, coluna 20, escrever em vermelho e voltar à cor normal     |
| 2   | Seção "Functions using CSI": CUP, ED, EL e SGR                                             | Repetir o item 1 com `lua -e` e `io.write`, primeiro sem e depois com `io.flush`                                                 |
| 3   | Modos privados (DECSET/DECRST): `1049` e `25`                                              | Entrar na tela alternativa, esconder o cursor, esperar um Enter e sair. Depois sair sem desfazer e observar o estrago            |
| 4   | `man stty`, em especial `raw`, `-echo`, `-g`, `size` e `sane`                              | No shell: `stty -g` para guardar o estado, `stty raw -echo`, digitar algumas teclas, e restaurar com o valor guardado (às cegas) |
| 5   | Revisão do cap. 7 do _Programming in Lua_ (`io.read` com número, `io.popen`, `os.execute`) | Ler um byte por vez com `io.read(1)` em modo cru e imprimir o código de cada um                                                  |
| 6   | (nenhuma)                                                                                  | Os passos abaixo                                                                                                                 |
| 7   | (nenhuma)                                                                                  | Responder os critérios por escrito                                                                                               |

Dica para o item 4: se o terminal ficar quebrado, digite `reset` ou `stty sane` e aperte Ctrl+J (o Enter pode não funcionar).

## Passo a passo

Pasta: `lua/tui/`. Os arquivos desta fase já nascem no lugar definitivo: `src/ui/terminal.lua`, `src/ui/teclas.lua`, `demo/caixa.lua` e `testes/`.

1. **Sequências como dados:** no módulo `terminal`, escreva funções que **devolvem strings** de controle: mover o cursor, limpar a tela, entrar e sair da tela alternativa, mostrar e esconder o cursor, aplicar e zerar estilos. Elas não escrevem nada. Assim dá para testá-las e juntar várias numa única escrita.
2. **Modo cru com `stty`:** guarde o estado atual com `stty -g` (via `io.popen`), ligue o modo cru com `os.execute`, e escreva a função de restauração. Ligue o modo cru e imprima duas linhas com `print`: observe o efeito de escada e descubra qual opção do modo cru o causou.
3. **Modo diagnóstico:** um programa que, em modo cru, mostra os bytes de cada tecla pressionada (em decimal e em hexadecimal) até você apertar `q`. Use-o para descobrir o que chega com Enter, Backspace, Tab, Ctrl+A, Ctrl+C, as quatro setas, Home, End e uma letra acentuada. Anote numa tabela nas suas notas.
4. **Decodificador de teclas:** uma função pura que recebe bytes e devolve teclas com nome (`"a"`, `"enter"`, `"ctrl+c"`, `"cima"`, `"esc"`, ...). Ela deve lidar com sequências incompletas, devolvendo o que já sabe e guardando o resto. Escreva os testes com busted antes de ligar no programa.
5. **Esc isolado:** com `io.read(1)` não há como esperar "até 50 ms" pelo próximo byte. Nesta fase, aceite a limitação: um `Esc` sozinho fica pendente até a próxima tecla. Escreva nas notas o que exatamente está faltando; a Fase 1 resolve com `poll`.
6. **A caixa:** leia o tamanho da tela uma vez com `stty size`. Entre na tela alternativa, esconda o cursor e desenhe uma caixa com bordas (`┌ ─ ┐ │ └ ┘`), um título e uma cor de fundo. As setas movem a caixa, sem deixá-la sair da tela. Cada quadro é montado numa tabela de pedaços, juntado com `table.concat` e escrito com um único `io.write` seguido de `io.flush`. `q` e Ctrl+C saem.
7. **Saída limpa em todo caminho:** o programa inteiro roda dentro de um `xpcall`. Na saída, normal ou por erro, a ordem é: zerar estilos, mostrar o cursor, sair da tela alternativa, restaurar o `stty` e só então imprimir o traceback, se houver. Teste com um `error` proposital disparado por uma tecla.
8. **Limpeza:** `stylua`, `luacheck` e `busted` sem avisos nem falhas.

## Armadilhas comuns para quem vem de TypeScript

- **Esquecer o `io.flush`.** A tela simplesmente não muda, e parece que a leitura de teclas travou.
- **Efeito de escada.** O modo cru desliga também o processamento da saída, e `\n` passa a só descer uma linha, sem voltar para a coluna 1. Na tela alternativa você vai posicionar o cursor de qualquer jeito, mas mensagens de debug com `print` saem tortas.
- **Traceback que some.** Se o erro for impresso antes de sair da tela alternativa, ele aparece na tela que vai ser descartada e você nunca o vê.
- **Restaurar fora de ordem.** Restaurar o `stty` antes de sair da tela alternativa funciona por acaso; esquecer de mostrar o cursor deixa o shell sem cursor.
- **Linhas e colunas começam em 1** nas sequências de posicionamento, e a ordem é linha primeiro, coluna depois, o contrário de `x, y`.
- **`#` nas bordas.** `#"─"` é 3, porque o caractere ocupa 3 bytes em UTF-8, mas ocupa uma coluna na tela. Calcular a largura da caixa com `#` desalinha tudo. Nesta fase, conte as colunas pelo número de repetições; a largura de verdade vem na Fase 2.
- **Enter não é `\n`.** Em modo cru, o Enter costuma chegar como `\r` (13).
- **Backspace varia.** A maioria dos terminais manda 127, alguns mandam 8. Trate os dois.
- **Setas têm duas formas.** `ESC [ A` é a mais comum, mas alguns terminais e modos mandam `ESC O A`. O modo diagnóstico do passo 3 mostra qual o seu manda.
- **Ctrl+C não é mais sinal.** Em modo cru ele chega como o byte 3. Se você não tratar, o programa não sai. E `kill` de outro terminal ainda mata o processo sem restaurar nada: isso só se resolve na Fase 1.
- **`stty` precisa do terminal como entrada.** Ele age sobre a entrada padrão; rodado num contexto em que a entrada não é o terminal, ele falha com "Inappropriate ioctl for device".
- **Tentar resolver o `Esc` com `stty min 0 time 1`.** Faz o `read` voltar vazio depois de um tempo, mas a libc trata isso como fim de arquivo, e a leitura seguinte pode continuar devolvendo `nil`. Não perca tempo: é a deixa para o `poll` da Fase 1.

## Para se testar antes de marcar os critérios

Responda por escrito em `lua/docs-tui/fase-0/NOTAS.md`:

1. Monte a tabela do passo 3: tecla, bytes recebidos e o nome que o seu decodificador devolve.
2. O que exatamente o `stty raw` desliga? Relacione cada efeito que você observou (sem eco, sem esperar Enter, Ctrl+C virando byte, efeito de escada) com uma opção do `stty`.
3. Por que uma seta chega como três bytes, e por que isso torna o `Esc` ambíguo? O que falta ao `io.read` para resolver?
4. Descreva a ordem de restauração que você usou e o que dá errado se trocar dois passos de lugar.
5. Por que montar o quadro numa tabela e escrever uma vez é melhor que várias chamadas a `io.write` com `io.flush` em cada uma?
6. O que acontece se você matar o programa com `kill -TERM` de outro terminal? E com `kill -9`? Como você recupera o terminal?

## Critérios para avançar (do PRD)

- [ ] Explico o que o modo cru desliga (eco, edição de linha, sinais por teclado) e por que Ctrl+C deixa de matar o programa.
- [ ] Explico por que uma tecla de seta chega como vários bytes e por que `Esc` sozinho é ambíguo.
- [ ] Explico por que sem `io.flush` a tela não atualiza.
- [ ] Um `error` proposital no meio do programa devolve o terminal em estado normal.

Quando terminar, mande o código com o modelo de revisão do PRD.
