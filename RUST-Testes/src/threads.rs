use std::thread;

fn main() {
    // Crie um vetor de números para calcular a soma
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Divida o vetor em duas partes e faça a clonagem
    let mid = numbers.len() / 2;
    let left = numbers[..mid].to_vec(); //to_vec( pra clonar pra resolver um erro )
    let right = numbers[mid..].to_vec();

    // Crie duas threads para calcular a soma em paralelo
    let handle1 = thread::spawn(move || {
        let sum = left.iter().sum::<i32>();
        sum
    });

    let handle2 = thread::spawn(move || {
        let sum = right.iter().sum::<i32>();
        sum
    });

    // Aguarde as threads terminarem e obtenha os resultados
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();

    // Calcule a soma total
    let total_sum = result1 + result2;

    println!("Soma total: {}", total_sum);
}


Neste exemplo:

    Criamos um vetor de números.
    Dividimos o vetor em duas partes, representando duas tarefas separadas.
    Criamos duas threads, cada uma calculando a soma de uma parte do vetor.
    Aguardamos a conclusão das threads usando join() para obter os resultados individuais.
    Somamos os resultados das threads para obter a soma total.

Isso demonstra como você pode usar threads para realizar tarefas em paralelo e, em seguida, combinar os resultados. Lembre-se de que, em programas mais complexos, 
é importante gerenciar a sincronização e a comunicação entre as threads adequadamente para evitar problemas como condições de corrida.
Rust fornece mecanismos como mutexes e canais para facilitar a sincronização entre threads.


*/



// exemplo 2:
use std::thread;

fn main() {
    // Crie uma nova thread que executa a função anônima
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: {}", i);
        }
    });

    // Aguarde a thread terminar
    handle.join().unwrap();

    // Thread principal continua a execução
    for i in 6..=10 {
        println!("Main Thread: {}", i);
    }
}

