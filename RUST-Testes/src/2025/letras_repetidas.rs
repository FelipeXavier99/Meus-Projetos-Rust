//codewars.com

use std::collections::HashSet;

fn tem_letras_repetidas(palavra: &str) -> bool {
    let mut letras_vistas = HashSet::new();
    for letra in palavra.chars() {
        if !letras_vistas.insert(letra) {
            return true; // Letra repetida encontrada
        }
    }
    false // Nenhuma letra repetida
}

fn main() {
    let palavra = "hello";
    if tem_letras_repetidas(palavra) {
        println!("A palavra '{}' tem letras repetidas.", palavra);
    } else {
        println!("A palavra '{}' não tem letras repetidas.", palavra);
    }
}
