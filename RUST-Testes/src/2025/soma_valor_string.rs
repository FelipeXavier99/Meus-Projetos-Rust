// Soma um valor quando tem numeros no final da String 
//Exemplo: "foobar001"; Esperado: "foobar002"

//codewars.com

fn substituir_ultimo_numero(s: &str) -> String {
    // Pega os últimos números da string incluindo zeros à esquerda
    let numeros = s.chars()
        .rev() // Inverte a string para percorrer de trás para frente
        .take_while(|c| c.is_numeric()) // Pega os últimos números
        .collect::<String>() // Coleta como string
        .chars()
        .rev()// Reverte novamente para manter a ordem correta
        .collect::<String>(); // Reverte para a ordem correta

    // Se encontramos números no final
    if !numeros.is_empty() {
        let prefixo = &s[..s.len() - numeros.len()]; // Pega o texto antes dos números

        // Clone da variável `numeros` para evitar o erro de "borrow of moved value"
        let numero_str = numeros.clone(); // Clonamos a string para usar mais tarde

        // Aumenta o número manualmente
        let numero = numero_str.parse::<i32>().unwrap_or(0) + 1; // Soma 1 ao número
        let novo_numero = format!("{:0width$}", numero, width = numeros.len()); // Formata o novo número com zeros à esquerda, preservando o comprimento

        // Concatena o prefixo com o número formatado
        format!("{}{}", prefixo, novo_numero)
    } else {
        // Caso não haja números no final, adiciona "1" no final
        let compactar = format!("{}1", s);
        println!("A string NÃO termina com um número. Novo valor: {}", compactar);
        compactar // Retorna a nova string com "1" no final
    }
}

fn main() {
    let exemplo = "foobar001";
    let resultado = substituir_ultimo_numero(exemplo);
    println!("Nova string: {}", resultado); // Esperado: "foobar002"
}
