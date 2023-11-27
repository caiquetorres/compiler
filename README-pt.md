# Compilador

Esse repositório contém um compilador feito em Rust, como parte de um projeto de graduação. O compilador, atualmente, está em desenvolvimento.

Ele foi projetado para converter códigos inseridos da nossa linguagem de programação (que ainda não possui nome) para a linguagem C.

> Obs: O repositório contém a pasta [samples](/samples), que possui alguns algorítimos feitos na linguagem de programação, eles pode ser usados para melhor entendimento do funcionamento dela, como declaração de variáveis, funções, estruturas de repetição, etc.

## Instalação

Para começar a usar o compilador, siga estas etapas simples:

Clonar o Repositório: Primeiro, clone o repositório usando o seguinte comando:

```sh
git clone https://github.com/caiquetorres/compiler.git
```

Compilação do Código: Em seguida, vá para o diretório do compilador e construa-o usando Cargo:

```sh
cd compiler
cargo build
```

Compilar um Arquivo: Agora, para compilar um arquivo específico, execute o comando a seguir, informando o caminho do arquivo:

```sh
cargo run -- --compile samples/binary_search.x
```

Isso gerará o arquivo `output.c`, que será criado na pasta principal do projeto.

## Exemplo

Aqui está um exemplo simples de um programa escrito na linguagem que este compilador suporta:

```x
fun main() {
    let visited: [bool; 100];
    let graph: [[i32; 100]; 100];

    let nodesAmount = 4;
    let edges = [
        [0, 1, 1, 0],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [0, 1, 1, 0]
    ];

    for i in 0..nodesAmount {
        for j in 0..nodesAmount {
            graph[i][j] = edges[i][j];
        }
    }

    dfs(0, nodesAmount, visited, graph);
}

fun dfs(node: i32, nodesAmount: i32, visited: [bool; 100], graph: [[i32; 100]; 100]) {
    visited[node] = true;
    println node;

    for i in 0..nodesAmount {
        if graph[node][i] == 1 && !visited[i] {
            dfs(i, nodesAmount, visited, graph);
        }
    }
}
```

O resultado convertido para C pode ser visto abaixo:

```c
#include <stdio.h>
void dfs(signed int node, signed int nodesAmount, unsigned char(*),
         signed int (*)[100]);
signed int main() {
  unsigned char(*visited) = (unsigned char[100]){};
  signed int(*graph)[100] = (signed int[100][100]){};
  signed int nodesAmount = 4;
  signed int(*edges)[4] =
      (signed int[4][4]){0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0};
  for (signed int i = 0; i < nodesAmount; i++) {
    for (signed int j = 0; j < nodesAmount; j++) {
      graph[i][j] = edges[i][j];
    }
  }
  dfs(0, nodesAmount, visited, graph);
  return 0;
}
void dfs(signed int node, signed int nodesAmount, unsigned char(*visited),
         signed int (*graph)[100]) {
  visited[node] = 1;
  printf("%d", node);
  printf("\n");
  for (signed int i = 0; i < nodesAmount; i++) {
    if (graph[node][i] == 1 && !visited[i]) {
      dfs(i, nodesAmount, visited, graph);
    }
  }
}
```

## Inicio

Para iniciar a execução do código, é fundamental criar uma função chamada main. Essa função é crucial, pois representa o ponto de partida do programa.

```x
fun main() {
    // Código
}
```

É importante notar que ela não possui retorno e não aceita parâmetros. Ela é estruturada apenas com o nome `main`.

Todo o código que se deseja executar deve estar contido dentro desta função.

### Variáveis

Na linguagem, a declaração de variáveis é simples. Para criar uma, utilize a palavra-chave `let`, seguida pelo nome da variável e seu valor, como demonstrado abaixo:

```x
fun main() {
    let x = 2;
}
```

Uma característica interessante é que a linguagem não exige a especificação do tipo de dado durante a declaração da variável. No entanto, é possível adicionar o tipo após o nome da variável, como exemplificado a seguir:

```x
fun main() {
    let x: i32 = 2;
}
```

Essa flexibilidade permite que a variável seja iniciada sem indicar explicitamente o tipo de dado, mas também oferece a opção de especificar, se necessário.

As variáveis podem adotar qualquer tipo de dado presente na linguagem, incluindo tipos primitivos, funções e vetores.

Os tipos primitivos são:

-   char
-   u8
-   i8
-   u16
-   i16
-   u32
-   i32
-   u64
-   i64
-   f32
-   f64

Estes tipos primitivos oferecem diferentes representações de dados, variando em tamanho para armazenar informações de acordo com as necessidades do programa.

#### Escopos de variáveis

O escopo de uma variável determina onde ela pode ser acessada dentro do código. Na linguagem, o escopo é definido por blocos delimitados por chaves {}. Variáveis declaradas dentro de um bloco só são acessíveis dentro desse bloco específico.

Exemplo de escopo:

```x
fun main() {
    let x = 10; // Variável x é acessível dentro deste bloco main()

    {
        let y = 20;
        println y; // Variável y é acessível apenas dentro deste bloco interno
    }

    // println y; // Isso resultaria em um erro, já que y não é acessível fora do bloco acima
    println x; // A variável x definida neste bloco, portanto acessível aqui
}
```

Em alguns casos, as variáveis aceitam conversões implícitas entre tipos, como demonstrado no exemplo a seguir:

```x
fun main() {
    let a: i32 = 2;
    let b: f32 = 3.5;

    a = b;

    println a; // 3
}
```

Observação: No exemplo fornecido, o valor `3.5` do tipo `f32` é atribuído a `a`, que é do tipo `i32`. Essa conversão implícita de `f32` para `i32` resulta na atribuição de `3` a `a`.

Nota-se que variaveis também pode assumir valores de função, que serão abordadas mais a frente, como no exemplo abaixo:

```x
fun main() {
    let x = say;
    x(); // Hello world!
}

fun say() {
    println "Hello world!";
}
```

### Vetores

A linguagem oferece suporte para vetores. Eles são criados utilizando colchetes ([]), como mostrado no exemplo abaixo:

```x
fun main() {
    let x = [1, 2, 3];
}
```

Além disso, é possível especificar explicitamente o tipo e o tamanho do vetor utilizando a sintaxe [tipo; tamanho_do_vetor], como demonstrado abaixo:

```x
fun main() {
    let x: [i32; 3] = [1, 2, 3];
}
```

Os vetores podem ser modificados e acessados por índices, como ilustrado abaixo:

```x
fun main() {
    let x = [1, 2, 3];

    println x[0]; // 1

    x[0] = 0;

    println x[0]; // 0
}
```

Vetores também podem conter mais dimensões, ou seja, vetores de vetores. Como demonstrado abaixo:

```x
fun main() {
    let m = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    println m[1][1]; // 5
}
```

Os vetores não podem ser de vários tipos portanto:

```x
fun main() {
    let m = [2, 'c']; // Inválido, será considerado o tipo referente ao primeiro elemento do vetor, nesse caso 2 (i32).
}
```

### Strings

A linguagem também oferece suporte para strings. Embora não sejam consideradas tipos primitivos, são compostas por caracteres e se assemelham bastante aos vetores, divergindo apenas na dispensa da necessidade de informar explicitamente seu comprimento. Para declarar uma string, basta utilizar aspas ("), como exemplificado abaixo:

```x
fun main() {
    let nome = "Caique";
}
```

Assim como vetores, strings podem ser manipuladas usando os indexadores, como demonstrado abaixo:

```x
fun main() {
    let nome = "Caique";

    nome[0] = 'c';

    println nome; // caique
}
```

### Funções

Para criar uma função, utiliza-se a palavra-chave `fun` seguida do nome da função e de seus parâmetros, conforme exemplificado abaixo:

```x
fun multiply(a: i32, b: i32) -> i32 {
    return a * b;
}
```

Neste exemplo, a função `multiply` recebe dois parâmetros do tipo `i32` e retorna um valor do mesmo tipo, representando a multiplicação entre esses parâmetros.

Observação: Atualmente a linguagem não suporta o retorno de vetores e funções.

```x
// Inválido
fun createArray() -> [i32; 2] {
    return [1, 2];
}
```

```x
fun say() {
    println "Hello world!";
}

// Inválido
fun execute() -> () -> void {
    return say;
}
```

### Outras estruturas

#### Loops

```x
fun main() {
    for i in 0..10 { }
}
```

```x
fun main() {
    for i in 0..=10 { }
}
```

```x
fun main() {
    let condition = true;

    while condition { }
}
```

```x
fun main() {
    let condition = true;

    do { } while condition;
}
```

```x
fun main() {
    let condition = true;

    if condition { } else { }
}
```
