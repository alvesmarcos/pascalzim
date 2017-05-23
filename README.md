pascalzim
=====

Versão simplificada do compilador pascalzim escrito em rust

### Instalando Rust

Para instalar o Rust, rode o seguinte comando no terminal e siga as instruções na tela:

```console
curl https://sh.rustup.rs -sSf | sh
```

### Testando

Navegue até a pasta `src` e no terminal, digite: 

```console
$ rustc --crate-type=lib lib.rs
```

Esse comando vai gerar um arquivo `libpascalzim.rlib`

Compile o arquivo *pascalzim.rs* que possui o conteúdo abaixo:

```rs
extern crate lib;
use lib::parser::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut p1: Parser = Parser::new();
    p1.build_ast(&args[1]);
}
```

Para compilar:

```console
rustc pascalzim.rs --extern lib=libpascalzim.rlib
```

Esse comando deverá gerar um arquivo compilado `pascalzim` no diretório corrente.

Para rodar:

```console
./pacalzim ../files/program6.txt
```

##### Qualquer arquivo pode ser passado como argumento para a execução do compilador.
