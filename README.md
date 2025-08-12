# 🔐 hmacgen-cli

Gerador de HMAC-SHA256 em Rust para strings via linha de comando.

![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## 🚀 Funcionalidades

- Gera HMAC-SHA256 a partir de strings
- Saída em hexadecimal
- Binário leve e rápido

## 📦 Instalação

### Via Cargo:

````bash
cargo install --git https://github.com/joserobertoferreira/rust-hmac-has256-generator.git

## 🛠️ Uso

Execute o binário passando a mensagem e a chave secreta:

```bash
hmacgen-cli "mensagem" --key "sua_chave_secreta"
````

Ou usando o atalho:

```bash
hmacgen-cli "mensagem" -k "sua_chave_secreta"
```

O programa irá imprimir o HMAC-SHA256 da mensagem em formato hexadecimal.

## 📖 Exemplo

```bash
hmacgen-cli "exemplo" -k "minha-chave"
# Saída: 6e3b2b8e2a8c2e... (hash em hexadecimal)
```
