use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Função assíncrona que espera por 2 segundos
    async fn wait_two_seconds() {
        sleep(Duration::from_secs(2)).await;
        println!("Passaram-se 2 segundos!");
    }

    // Execute a função assíncrona
    wait_two_seconds().await;
}

/*
Neste exemplo, a função wait_two_seconds é assíncrona e usa await para esperar por 2 segundos antes de imprimir uma mensagem. O #[tokio::main] é um atributo usado para definir a função main como uma função assíncrona
e inicia o runtime de Tokio para executar tarefas assíncronas.
*/
