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

## Como Fornecer a Chave Secreta

A ferramenta é flexível e busca a chave secreta na seguinte **ordem de prioridade**:

1.  Flag de linha de comando (`--key` ou `--key-file`)
2.  Variável de Ambiente (`HMAC_KEY`, que pode ser carregada por um ficheiro `.env`)
3.  Prompt Interativo

Isto permite que você tenha uma configuração para desenvolvimento local e a possa sobrescrever facilmente para um uso específico.

### 1. Flags de Linha de Comando (Prioridade Máxima)

Estas opções são explícitas e sobrescrevem todas as outras.

#### Usando `--key-file` (Recomendado para Produção/Automação)

Lê a chave de um ficheiro. Ideal para ambientes de produção e automação, como Docker Secrets ou ficheiros de configuração geridos.

```sh
# Crie um ficheiro com a chave e restrinja as permissões
echo "chave-de-producao" > prod.key
chmod 600 prod.key

# Execute o comando
hmac_generator "Mensagem de produção" --key-file prod.key
```

#### Usando `--key` (Para Testes Rápidos)

Fornece a chave diretamente. **Não recomendado para produção**, pois a chave pode ficar no histórico do seu shell.

```sh
hmac_generator "Mensagem de teste" --key "chave-insegura"
```

---

### 2. Variável de Ambiente `HMAC_KEY`

Esta é a forma ideal para desenvolvimento local e ambientes de CI/CD.

#### Para Desenvolvimento Local com `.env`

A ferramenta carrega automaticamente variáveis de um ficheiro `.env` na raiz do projeto.

1.  **Crie o ficheiro `.env`** na raiz do seu projeto:

    ```
    # Ficheiro: .env
    HMAC_KEY="minha-chave-secreta-de-desenvolvimento"
    ```

2.  **(CRUCIAL) Adicione `.env` ao seu ficheiro `.gitignore`** para NUNCA enviar segredos para o repositório.

    ```
    # Ficheiro: .gitignore
    .env
    ```

3.  **Execute o programa sem flags.** Ele encontrará a chave no `.env`.

    ```sh
    cargo run -- "Minha mensagem de desenvolvimento"
    ```

4.  **(Opcional, mas recomendado)** Crie um ficheiro de exemplo `.env.example` para que outros saibam quais variáveis são necessárias.
    ```
    # Ficheiro: .env.example
    HMAC_KEY=
    ```

---

### 3. Prompt Interativo (Fallback Seguro)

Se nenhuma das opções acima for usada, a ferramenta solicitará a chave de forma segura, sem a exibir na tela. Perfeito para uso manual.

```sh
$ hmac_generator "Uma mensagem para uso manual"

Por favor, digite a chave secreta HMAC:
(sua chave digitada aqui não será exibida)
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
