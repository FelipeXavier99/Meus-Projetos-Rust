//Exemlos pra buscar bandeira de cartao de credito 

// 1- com condiçao
extern crate regex;
use regex::Regex;

fn main() {
    let card_number = "4532012345678901"; // Substitua isso pelo número do cartão que você deseja verificar.

    // Crie uma expressão regular para corresponder aos dois primeiros dígitos desejados.
    let first_two_digits_regex = Regex::new(r"^(34|37)").unwrap();

    if first_two_digits_regex.is_match(&card_number[..2]) {
        println!("Os dois primeiros dígitos correspondem a uma bandeira específica (por exemplo, Amex).");
    } else {
        println!("Os dois primeiros dígitos não correspondem à bandeira desejada.");
    }
}

//2- com Match
fn main() {
    let card_number = "3532012345678901"; // Substitua isso pelo número do cartão que você deseja verificar.

    match &card_number[..2] {
        "34" | "37" => println!("Os dois primeiros dígitos correspondem a Amex"),
        "51" | "52" | "53" | "54" | "55" => println!("Os dois primeiros dígitos correspondem a Mastercard"),
        "4" => println!("Os dois primeiros dígitos correspondem a Visa"),
        _ => println!("Os dois primeiros dígitos não correspondem a nenhuma bandeira conhecida."),
    }
}

//3- buscando 2 caracter
fn obter_dois_primeiros_chars(texto: &str) -> &str {
    &texto[0..2]
}

fn main() {
    let texto = "Exemplo";
    let dois_primeiros = obter_dois_primeiros_chars(texto);
    println!("Os dois primeiros caracteres são: {}", dois_primeiros);
}
