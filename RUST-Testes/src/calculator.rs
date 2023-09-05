// src/minha_classe.rs
use std::io;
pub fn minha_funcao() {
    println!("Esta é a minha função em minha_classe.rs");





    let mut operador: i32;

    loop {
        let mut it1 = 0;
        let mut it2 = 0;
        let mut res = 0;
        let mut div = 0.0;

        let mut op = String::new();

        println!("Digite uma operação: ");
        println!("1 - SOMAR");
        println!("2 - DIMINUIR");
        println!("3 - MULTIPLICAR");
        println!("4 - DIVISÃO");
        println!("5 - SAIR");

        io::stdin().read_line(&mut op).expect("Erro ao ler a linha");

        operador = match op.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Operação inválida");
                continue;
            }
        };

        match operador {
            1 => {
                let mut item = String::new();
                let mut item2 = String::new();

                println!("Digite um valor 1: ");
                io::stdin()
                    .read_line(&mut item)
                    .expect("Erro ao ler a linha");
                it1 = match item.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Valor inválido");
                        continue;
                    }
                };

                println!("Digite um valor 2: ");
                io::stdin()
                    .read_line(&mut item2)
                    .expect("Erro ao ler a linha");
                it2 = match item2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Valor inválido");
                        continue;
                    }
                };

                res = it1 + it2;
                println!("RESPOSTA: {}", res);
            }
            2 => {
                let mut item = String::new();
                let mut item2 = String::new();

                println!("Digite um valor 1: ");
                io::stdin()
                    .read_line(&mut item)
                    .expect("Erro ao ler a linha");
                it1 = match item.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Valor inválido");
                        continue;
                    }
                };

                println!("Digite um valor 2: ");
                io::stdin()
                    .read_line(&mut item2)
                    .expect("Erro ao ler a linha");
                it2 = match item2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Valor inválido");
                        continue;
                    }
                };

                res = it1 - it2;
                println!("RESPOSTA: {}", res);
            }
            3 => {
                let mut item = String::new();
                let mut item2 = String::new();

                println!("Digite um valor 1: ");
                io::stdin()
                    .read_line(&mut item)
                    .expect("Erro ao ler a linha");
                it1 = match item.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Valor inválido");
                        continue;
                    }
                };

                println!("Digite um valor 2: ");
                io::stdin()
                    .read_line(&mut item2)
                    .expect("Erro ao ler a linha");
                it2 = match item2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Valor inválido");
                        continue;
                    }
                };

                res = it1 * it2;
                println!("RESPOSTA: {}", res);
            }
            4 => {
                let mut item = String::new();
                let mut item2 = String::new();

                println!("Digite um valor 1: ");
                io::stdin()
                    .read_line(&mut item)
                    .expect("Erro ao ler a linha");
                it1 = match item.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Valor inválido");
                        continue;
                    }
                };

                println!("Digite um valor 2: ");
                io::stdin()
                    .read_line(&mut item2)
                    .expect("Erro ao ler a linha");
                it2 = match item2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Valor inválido");
                        continue;
                    }
                };

                if it2 == 0 {
                    println!("Não é possível dividir por zero.");
                    continue;
                }

                div = it1 as f64 / it2 as f64;
                println!("RESPOSTA: {}", div);
            }
            5 => {
                println!("SAINDO");
                break;
            }
            _ => {
                println!("Operação Inválida");
            }
        }
    }
}
