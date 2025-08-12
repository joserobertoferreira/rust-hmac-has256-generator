# HMAC-SHA256 Generator

![Linguagem](https://img.shields.io/badge/language-Rust-orange.svg)
![Licença](https://img.shields.io/badge/license-MIT-blue.svg)
![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)

Um utilitário de linha de comando (CLI) seguro e flexível, escrito em Rust, para gerar códigos de autenticação de mensagem (HMAC) utilizando o algoritmo SHA-256.

Esta ferramenta foi projetada com a segurança em mente, oferecendo múltiplas formas de fornecer a chave secreta para evitar sua exposição em históricos de shell ou listas de processos.

## Funcionalidades

- **Geração de HMAC-SHA256**: Calcula o HMAC para qualquer string de entrada.
- **Flexibilidade na Chave Secreta**: A chave pode ser fornecida através de:
  - Um prompt interativo e seguro (não exibe a chave na tela).
  - Um ficheiro local.
  - Uma variável de ambiente (`HMAC_KEY`).
  - Diretamente como argumento (não recomendado para produção).
- **Multiplataforma**: Compila e roda nativamente em Linux, macOS e Windows.
- **Leve e Rápido**: Escrito em Rust para máxima performance e segurança.

## Instalação

### Pré-requisitos

Você precisa ter o [Rust e o Cargo instalados](https://www.rust-lang.org/tools/install).

### Compilando a partir do código-fonte

1.  Clone este repositório:

    ```sh
    git clone https://github.com/joserobertoferreira/rust-hmac-has256-generator.git
    cd rust-hmac-has256-generator
    ```

2.  Compile o projeto em modo otimizado:

    ```sh
    cargo build --release
    ```

3.  O executável estará em `./target/release/hmac_generator`. Para facilitar o uso, você pode movê-lo para um diretório no seu `PATH`:
    ```sh
    sudo mv ./target/release/hmac_generator /usr/local/bin/
    ```

## Como Usar

A sintaxe básica é:

```
hmac_generator [MENSAGEM] [OPÇÕES_DE_CHAVE]
```

### Exemplos de Uso

A seguir estão exemplos das diferentes maneiras de fornecer a chave secreta, da mais segura para a menos segura.

#### 1. Modo Interativo (Recomendado para uso manual)

Se nenhuma chave for fornecida via flags ou variáveis de ambiente, a ferramenta solicitará que você a digite de forma segura:

```sh
$ hmac_generator "A mensagem que eu quero autenticar"

Por favor, digite a chave secreta HMAC:
(sua chave digitada aqui não será exibida)
```

**Saída:**

```
4e1a72d57e397554911e9a2b25684e2a149a4e3b1c6d8f8a85f4b4f1b4a9a9a5
```

#### 2. Usando um Ficheiro de Chave

Esta é uma optima opção para automação e scripts.

1.  Crie um ficheiro com sua chave secreta:
    ```sh
    echo "minha-chave-guardada-em-ficheiro" > secret.key
    ```
2.  (Importante) Restrinja as permissões do ficheiro:
    ```sh
    chmod 600 secret.key
    ```
3.  Execute o comando:
    ```sh
    hmac_generator "Esta mensagem usa uma chave de ficheiro" --key-file ./secret.key
    ```

#### 3. Usando uma Variável de Ambiente

Ideal para ambientes de CI/CD e scripts de deploy.

```sh
# Linux/macOS
export HMAC_KEY="minha-chave-super-secreta"
hmac_generator "Autenticado via variável de ambiente"

# Windows (PowerShell)
$env:HMAC_KEY="minha-chave-super-secreta"
.\hmac_generator.exe "Autenticado via variável de ambiente"
```

#### 4. Passando a Chave como Argumento (Não Seguro)

**Atenção**: Use este método apenas para testes rápidos, pois a chave ficará visível no histórico do seu shell.

```sh
hmac_generator "Olá, Mundo!" --key "chave-insegura"
```

### Ajuda

Para ver todas as opções disponíveis, use a flag `--help`:

```sh
hmac_generator --help
```

## Compilação Cruzada (Cross-Compilation) para Windows

É possível compilar um executável para Windows a partir de um ambiente Linux ou macOS.

1.  Adicione o target do Windows via `rustup`:

    ```sh
    rustup target add x86_64-pc-windows-gnu
    ```

2.  Instale o linker MinGW-w64 (exemplo para Debian/Ubuntu):

    ```sh
    sudo apt-get install mingw-w64
    ```

3.  Configure o Cargo para usar o linker correto. Crie o ficheiro `.cargo/config.toml` com o seguinte conteúdo:

    ```toml
    [target.x86_64-pc-windows-gnu]
    linker = "x86_64-w64-mingw32-gcc"
    ```

4.  Compile para o target do Windows:
    ```sh
    cargo build --release --target x86_64-pc-windows-gnu
    ```

O executável `.exe` estará disponível em `target/x86_64-pc-windows-gnu/release/`.

## Licença

Este projeto está licenciado sob a [Licença MIT](LICENSE).
