// convertendo nuemros inteiros para formato de telefone
//Exercicio codewars.com


fn telefone(numbers: &[u8]) -> String {
    format!(
        "({}{}{}) {}{}{}-{}{}{}{}",
        numbers[0], numbers[1], numbers[2], // Código de área
        numbers[3], numbers[4], numbers[5], // Primeiros 3 dígitos
        numbers[6], numbers[7], numbers[8], numbers[9] // Últimos 4 dígitos
    )
}

fn main() {
    let numeros = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let resultado = telefone(numeros);
    println!("{}", resultado); // Saída esperada: "(123) 456-7890"
}
