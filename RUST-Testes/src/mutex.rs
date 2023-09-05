use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Cria um Mutex que envolve um valor compartilhado (no caso, um contador).
    let contador = Arc::new(Mutex::new(0));

    // Cria um vetor de threads.
    let mut threads = vec![];

    for _ in 0..10 {
        // Clona o Mutex para cada thread, de modo que cada uma delas tenha acesso exclusivo.
        let contador_clone = Arc::clone(&contador);

        // Cria uma nova thread.
        let thread = thread::spawn(move || {
            // Bloqueia o Mutex para acessar a seção crítica de forma segura.
            let mut valor = contador_clone.lock().unwrap();

            // Incrementa o contador.
            *valor += 1;
        });

        threads.push(thread);
    }

    // Aguarda todas as threads terminarem.
    for thread in threads {
        thread.join().unwrap();
    }

    // Lê o valor final do contador.
    let valor_final = *contador.lock().unwrap();
    println!("Valor final do contador: {}", valor_final);
}
