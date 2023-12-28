use log::{info, warn, error};
use env_logger::Env;

fn main() {
    // Configurar o logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Exemplos de mensagens de log
    info!("Esta é uma mensagem de informação.");
    warn!("Este é um aviso.");
    error!("Este é um erro.");
}


[dependencies]
log = "0.4"
env_logger = "0.10.1"
