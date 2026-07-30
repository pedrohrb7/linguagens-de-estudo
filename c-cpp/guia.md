# Guia de Estudo — C e C++

> Referência pessoal. Contexto: já programo com **TypeScript** e **Java**, então
> o foco não é lógica de programação, e sim o que C/C++ fazem de diferente
> (memória manual, ponteiros, compilação em etapas).

---

## 1. Stack vs Heap

Dois lugares onde dados vivem na memória.

- **Stack** — memória automática. Variáveis locais nascem no escopo e **morrem
  sozinhas** quando a função retorna. Rápida, mas com tempo de vida amarrado ao escopo.
- **Heap** — memória que *você* pede e *você* devolve. Sobrevive além do escopo,
  mas é sua responsabilidade liberar.

```c
void funcao() {
    int x = 42;                    // stack: some sozinho no fim da função
    int *p = malloc(sizeof(int));  // heap: você pediu
    *p = 42;
    free(p);                       // heap: você TEM que devolver
}
```

> Em Java todo `new` vai pra heap e o GC limpa. Em C **não há faxineiro** —
> esquecer o `free` = **memory leak**.

---

## 2. Ponteiros

Uma variável que guarda um **endereço de memória** em vez de um valor.

```c
int x = 10;
int *p = &x;        // & = "endereço de"  → p aponta pra x
printf("%d", *p);   // * = "valor no endereço" (dereferência) → 10
*p = 20;            // muda x através do ponteiro → x agora é 20
```

- `&x` → me dê o **endereço** de x
- `*p` → me dê o **valor** guardado nesse endereço

> Java tem referências, mas opacas. Em C o ponteiro é um número que você pode
> inspecionar, incrementar e usar errado. Poder e perigo no mesmo lugar.

---

## 3. Passagem por valor vs por referência

**Em C, tudo é passado por cópia por padrão.**

```c
void tenta_mudar(int n) { n = 100; }   // muda só a cópia local
void muda_mesmo(int *n) { *n = 100; }  // muda o original via ponteiro

int a = 5;
tenta_mudar(a);   // a continua 5
muda_mesmo(&a);   // a vira 100
```

> Java passa a *referência por valor* — o que confunde. C te obriga a entender
> o mecanismo porque você o faz na mão (passando `&a`).

---

## 4. Strings são arrays de bytes

Não existe tipo `String` rico. Uma string é um array de `char` terminado com
um byte zero (`\0`), o *null terminator*.

```c
char nome[] = "Ana";   // memória: 'A' 'n' 'a' '\0'  → 4 bytes, não 3
```

- Não existe `.length`. `strlen` **anda pela memória até achar o `\0`**.
- Se o `\0` sumir (sobrescrito por acidente), a função lê memória alheia → bug.

> Reimplementar `strlen`/`strcpy` na mão ensina metade de C.

---

## 5. Undefined Behavior (UB)

Muita coisa ilegal **não é checada** — sem exceção, sem erro.

```c
int arr[3];
arr[10] = 5;        // fora dos limites: SEM erro, SEM exceção
int y;
printf("%d", y);    // variável não inicializada: lixo aleatório
```

Resultado possível: funcionar por acaso, imprimir lixo, corromper outra
variável, ou crashar — às vezes só em produção.

> É o que torna C rápido (nenhuma checagem em runtime) e perigoso.
> Java te daria `ArrayIndexOutOfBoundsException`; C não te dá nada.

---

## 6. Modelo de compilação

Processo explícito em duas fases (nada de `node arquivo.ts`):

1. **Compilação** — cada `.c` vira um objeto `.o` (código de máquina com
   "buracos" onde chama coisas de outros arquivos).
2. **Linkagem** — o *linker* junta os `.o` e resolve os buracos → executável.

**Headers (`.h`)** declaram *que* uma função existe; o `.c` a implementa.

```c
// mat.h  — a promessa (assinatura)
int soma(int a, int b);

// mat.c  — a implementação
int soma(int a, int b) { return a + b; }

// main.c — o uso
#include "mat.h"
int r = soma(2, 3);
```

> Erro `undefined reference` no linker = você prometeu no header mas esqueceu
> de implementar ou de incluir o `.c` na compilação.

---

## Roteiro de projetos (C puro primeiro)

Ordem pensada pra drillar um conceito novo de cada vez:

| # | Projeto | Conceito principal |
|---|---------|--------------------|
| 1 | Calculadora de linha de comando | fluxo compilar/linkar, `printf`/`scanf`, tipos e overflow |
| 2 | Conversor de bases (dec ↔ bin ↔ hex) | bits e operadores bit a bit `& \| << >>` |
| 3 | `strlen`/`strcpy`/reverter string na mão | ponteiros e o `\0` — o coração de C |
| 4 | To-do em terminal com salvar em arquivo | structs, arrays de structs, `malloc`/`free`, I/O de arquivo |

Depois desses quatro → migrar pra **C++** e aprender as abstrações
(RAII, classes, STL) *entendendo* o que elas automatizam.

---

## Ferramenta que salva vidas

Compile sempre com sanitizers durante o aprendizado:

```bash
gcc -Wall -Wextra -fsanitize=address arquivo.c -o programa
```

- `-Wall -Wextra` → liga quase todos os avisos do compilador.
- `-fsanitize=address` → **AddressSanitizer**: aponta exatamente onde você
  acessou memória inválida ou vazou. Um debugger de memória de graça.

---

## Próximos conceitos (depois que os 6 assentarem)

- `struct` e `typedef`
- Alocação de arrays dinâmicos (`malloc` + tamanho)
- Lista ligada (une ponteiros + heap + structs)
- Aritmética de ponteiros
- Em C++: RAII, construtores/destrutores, referências (`&`), `new`/`delete`,
  templates e a STL (`std::vector`, `std::string`)
