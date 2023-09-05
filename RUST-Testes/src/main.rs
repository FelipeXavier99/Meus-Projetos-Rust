// rascunho rust



use std::io;
use std::io::Error;

//mod calculator;
struct Pessoa{
    nome:String,
    idade: u32,

}
fn main() {
    /* 
    //importando Classe
    calculator::minha_funcao();
*/
let pessoa1 =Pessoa {

    nome: String::from("Zé"),
    idade:30,

};

print!("Nome= {}",pessoa1.nome);
print!("Idade= {}",pessoa1.idade);
}

/*
//exemplo funcao
fn teste(a: i32) {
    println!("Função: {}", a);
}
 */
