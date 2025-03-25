//Pra saber sem começa com letra ou umero numa STRING

fn main() {
    let minha_string = "123abc";

    // Verifica se a string começa com um número
    if minha_string.starts_with(|c: char| c.is_numeric()) {
        println!("A string começa com um número");
    } else {
        println!("A string não começa com um número");
    }

    // Verifica se a string começa com uma letra
    if minha_string.starts_with(|c: char| c.is_alphabetic()) {
        println!("A string começa com uma letra");
    } else {
        println!("A string não começa com uma letra");
    }
}


// e se termina com letra ou número:

fn main() {
    let s = "foo";
    let compactar = format!("{}1",s);
   
    if s.ends_with(|c: char| c.is_numeric()) {
    
        println!("A string termina com um número!")
    } else {
    
        println!("A string NÃO termina com um número. {}",compactar)
    }
}
   
