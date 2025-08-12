use clap::Parser;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::fs;
use std::io::{self, IsTerminal};

type HmacSha256 = Hmac<Sha256>;

#[derive(Parser)]
#[command(author, version, about = "Gerador HMAC-SHA256 para strings")]
struct Args {
    /// String de entrada para calcular o HMAC
    mensagem: String,

    /// Chave secreta para o HMAC
    #[arg(short = 'k', long = "key", env = "HMAC_KEY", hide_env_values = true)]
    key: Option<String>,

    /// Caminho para um ficheiro com a chave secreta
    #[arg(long = "key-file")]
    key_file: Option<String>,
}

fn main() {
    // Busca a chave secreta do ficheiro .env
    dotenvy::dotenv().ok();

    let args = Args::parse();

    // Determina a chave secreta com base na prioridade
    let secret_key = match get_secret_key(&args) {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Erro ao obter a chave secreta: {}", e);
            std::process::exit(1);
        }
    };

    // Calcula o HMAC
    let mut mac = HmacSha256::new_from_slice(secret_key.trim().as_bytes())
        .expect("Chave HMAC inválida (tamanho inesperado)");
    mac.update(args.mensagem.as_bytes());
    let resultado = mac.finalize();

    // Exibe o hash em hexadecimal
    println!("{}", hex::encode(resultado.into_bytes()));
}

/// Obtém a chave secreta da fonte apropriada com base nos argumentos.
/// Ordem de prioridade:
/// 1. --key
/// 2. --key-file
/// 3. Variável de ambiente HMAC_KEY
/// 4. Prompt interativo (se o stdin for um terminal)
fn get_secret_key(args: &Args) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(key) = &args.key {
        // Prioridade 1 e 3 combinadas pelo clap (flag --key ou env var)
        Ok(key.clone())
    } else if let Some(path) = &args.key_file {
        // Prioridade 2: Ler do ficheiro
        Ok(fs::read_to_string(path)?)
    } else if io::stdin().is_terminal() {
        // Prioridade 4: Prompt interativo, mas só se estivermos em um terminal
        println!("Por favor, digite a chave secreta HMAC:");
        let key = rpassword::read_password()?;
        Ok(key)
    } else {
        // Se não for interativo e nenhuma chave foi fornecida, é um erro.
        Err("Nenhuma chave fornecida (use --key, --key-file, var HMAC_KEY, ou um terminal interativo)".into())
    }
}
