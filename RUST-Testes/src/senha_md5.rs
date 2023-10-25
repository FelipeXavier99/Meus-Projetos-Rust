// lembrando q tem q borar nas dependecias

//[dependencies]
//md-5 = "2.1"

extern crate md_5;

use md_5::Md5;
use std::io::Write;

fn main() {
    let input_string = "MinhaSenha123";
    
    // Crie um novo objeto Md5
    let mut md5 = Md5::new();
    
    // Escreva os bytes da string no objeto Md5
    md5.write_all(input_string.as_bytes()).unwrap();
    
    // Calcule o hash MD5
    let result = md5.checksum();
    
    // Converta o hash em uma representação hexadecimal
    let hash_hex = format!("{:x}", result);
    
    println!("Senha original: {}", input_string);
    println!("Hash MD5: {}", hash_hex);
}

