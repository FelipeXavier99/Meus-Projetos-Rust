// Em codewars o desafio era pegar um código morse e voltar uma palavra (detalhe q são 3 espaços entre as palavras)

//codewars.com

mod preloaded;
use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".



fn decode_morse(encoded: &str) -> String {
   encoded
        .trim() // Remove espaços extras no início e fim
        .split("   ") // Divide as palavras (três espaços)
        .map(|palavra| {
            palavra
                .split_whitespace() // Divide a palavra em letras (um espaço)
                .filter_map(|codigo| MORSE_CODE.get(codigo)) // Converte cada letra
                .cloned() // Obtém os valores do HashMap
                .collect::<String>() // Junta as letras para formar palavras
        })
        .collect::<Vec<String>>() // Junta todas as palavras num vetor
        .join(" ") // Junta as palavras com um espaço
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    }
}
