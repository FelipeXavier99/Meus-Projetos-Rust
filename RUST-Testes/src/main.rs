// rascunho rust



use std::io;
use std::io::Error;

//mod calculator;
struct Pessoa{
    nome:String,
    idade: u32,

}
fn main() {
    // Criando uma Option que contém um valor inteiro
    let some_number: Option<i32> = Some(42);

    // Criando uma Option que não contém um valor
    let no_number: Option<i32> = None;

    // Usando match para lidar com Option
    match some_number {
        Some(value) => println!("Valor existente: {}", value),
        None => println!("Nenhum valor."),
    }

   
}


    /* 
    //importando Classe
    calculator::minha_funcao();

let pessoa1 =Pessoa {

    nome: String::from("Zé"),
    idade:30,

};

print!("Nome= {}",pessoa1.nome);
print!("Idade= {}",pessoa1.idade);
}
*/

/*
//exemplo funcao
fn teste(a: i32) {
    println!("Função: {}", a);

 */
