

//exemplo com split
fn main() {
    let my_string = "Olá, Mundo! Isso é um exemplo.";
    
    // Dividindo a string em partes com base na vírgula e espaço como delimitadores
    let parts: Vec<&str> = my_string.split(",").collect();
    
    // Iterando pelas partes e imprimindo cada uma
    for part in parts {
        println!("{}", part);
    }
}


// debug
fn main() {
    let x = 42;
    let y = 17;

    // Usando dbg! para imprimir o valor de uma expressão
    dbg!(x + y);
    println!("{}", dbg!);

    // Resto do código...
}
