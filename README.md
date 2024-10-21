# scclouds-2024
Códigos relacionados a processo seletivo.

## O que tem aqui
O código consiste em três módulos para realizar algumas operações matemáticas, sendo eles:
- `fibonacci.rs`: Para achar o enésimo termo da sequência de fibonacci;
- `prime_find.rs`: Para encontrar todos os números primos até o número informado;
- `prime_check.rs` (extra): Para determinar se um número é primo ou não.

Todos os módulos foram pensados para valores pequenos e possuem mais de uma implementação para resolver as requisições. No caso das operações com primos o limitante principal para operar valores grandes é o tempo, já no caso de Fibonacci o termo 93 já causa overflow no tipo básico u64 e portanto é o limite da ferramenta.

Cada módulo possue alguns testes básicos que orientaram a implementação, e podem ser rodados via `cargo test`.

## Como utilizar

Para utilizar a ferramenta rode o seguinte comando:

    cargo run -- <argumentos>
    
Caso você não possua a toolchain de Rust na sua máquina também está incluso um binário pré-compilado em Linux:

    ./scclouds-2024 <argumentos>

Existem 3 funcionalidades na CLI da ferramenta:

    ./scclouds-2024 fibonacci -n <nth> -f <function>
    ./scclouds-2024 prime-finder -n <nth> -f <function>
    ./scclouds-2024 prime-checker -n <nth> -f <function>

Você pode utilizar a flag `-h` para receber auxílio com o uso da ferramenta ou com cada funcionalidade em específico.

### Exemplos de uso

O seguinte comando irá encontra o 5º elemento da sequência de Fibonacci utilizado uma função recursive para isso:

    ./scclouds-2024 fibonacci -n 5 -f recursive

O seguinte comando irá mostrar em tela todos os primos encontrados até 10 (inclusive) utilizando a função funcional por padrão para isso:

    ./scclouds-2024 prime-finder -n 10

