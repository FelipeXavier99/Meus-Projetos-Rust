//1)Macros Declarativas (Macros de Item):
//Macros de item geram código Rust inteiro. Isso pode incluir declarações de funções, estruturas, 
módulos, ou até mesmo outras macros.


macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
}

fn main() {
    say_hello!(); // Chama a macro para gerar código. **DETALHE Q TEM Q TER (!)
}


//-------------------------------------------------
/*
2)Macros de Procedimento (Macros de Expressão):
Macros de procedimento geram código Rust em uma expressão.
    Elas são definidas usando a palavra-chave macro. Em contraste com as macros de item, as macros de procedimento 
se parecem mais com funções regulares, com argumentos e corpo.
*/

macro my_function() {
    println!("This is a procedural macro function.");
}

fn main() {
    my_function!(); // Chama a macro para gerar código em uma expressão.
}
