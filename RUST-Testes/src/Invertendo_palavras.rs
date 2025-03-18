// Inverte as palavras com mais de 5 letras na mesma frase/string



fn inverter_palavras_longas(frase: &str) -> String {
    frase
        .split_whitespace() // Divide a frase em palavras e ficaria assim: ["Olá", "mundo!]
        .map(|palavra| {
            if palavra.len() >= 5 {
                palavra.chars().rev().collect() // Inverte se tiver 5 ou mais letras
            } else {
                palavra.to_string() // Mantém a palavra original
            }
        })
        .collect::<Vec<String>>() // Junta as palavras transformadas em um vetor
        .join(" ") // Junta as palavras novamente em uma string separada por espaço
}

fn main() {
    let frase = "abc testes exemplo rust";
    let resultado = inverter_palavras_longas(frase);
    println!("{}", resultado);
}





OBS:
palavra.chars() → Converte a &str em um iterador de caracteres (char).

    Exemplo: Se palavra for "Rust", o iterador gerado será ['R', 'u', 's', 't'].

.rev() → Inverte a ordem dos caracteres.

    Exemplo: ['t', 's', 'u', 'R'].

.collect() → Coleta os caracteres invertidos e os junta em uma nova String.

    Exemplo: "tsuR".
