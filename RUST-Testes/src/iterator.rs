fn main() {
    let numeros = vec![1, 2, 3, 4, 5];

    // Criando um Iterator a partir do vetor
    let iter = numeros.iter();

    // Usando o método `map` para elevar ao quadrado cada elemento
    let quadrados: Vec<i32> = iter.map(|&x| x * x).collect();

    // Imprimindo os quadrados
    println!("{:?}", quadrados);

    // Usando o método `filter` para encontrar apenas números pares
    let pares: Vec<i32> = numeros.iter().filter(|&&x| x % 2 == 0).cloned().collect();

    // Imprimindo os números pares
    println!("{:?}", pares);
}
