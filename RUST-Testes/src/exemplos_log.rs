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


//////////////////////-- SEGUNDA OPÇÃO(tive q usar num projeto pq o de cima nao funcionava)////

use log::{info, warn, error};
use simple_logger::SimpleLogger;

fn main() {
    // Configurar o logger
    SimpleLogger::new().init().unwrap();

    // Exemplos de mensagens de log
    info!("Esta é uma mensagem de informação.");
    warn!("Este é um aviso.");
    error!("Este é um erro.");
}

DEPENDENCIAS:
log = "0.4"
simple_logger = "1.11.0"
