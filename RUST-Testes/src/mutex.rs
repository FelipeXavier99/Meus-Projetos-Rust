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



Exemplo 2:
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Criar um Mutex compartilhado para um vetor de números
    let numbers = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for i in 0..5 {
        let numbers = Arc::clone(&numbers);

        // Crie uma nova thread
        let handle = thread::spawn(move || {
            // Bloqueie o Mutex para acessar o vetor de números
            let mut data = numbers.lock().unwrap();

            // Adicione um número ao vetor
            data.push(i);

            // O Mutex será automaticamente desbloqueado quando "data" sair do escopo
        });

        handles.push(handle);
    }

    // Aguarde todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }

    // Obtenha o vetor de números após todas as threads terem terminado
    let final_numbers = numbers.lock().unwrap();
    println!("Números finais: {:?}", *final_numbers);
}


/*
Neste exemplo, estamos criando um Mutex compartilhado para um vetor de números. Cada thread bloqueia o Mutex antes de adicionar um número ao vetor. Isso garante que apenas uma thread por vez possa modificar o vetor, evitando possíveis condições de corrida.

Ao final, obtemos o vetor de números após todas as threads terem terminado e imprimimos os números finais. O uso do Mutex garante que a modificação do vetor seja feita de forma segura, mesmo quando várias threads estão acessando o recurso compartilhado.

*/
