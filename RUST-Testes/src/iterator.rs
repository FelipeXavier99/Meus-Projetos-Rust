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



//Exemplo 2

struct Queue {
    items: Vec<i32>,
}

impl Queue {
    fn new(items: Vec<i32>) -> Self {
        Self { items }
    }
}

impl Iterator for Queue {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.items.len() > 0 {
            Some(self.items.remove(0))
        } else {
            None
        }
    }
}

fn main() {
    let mut queue = Queue::new(vec![3, 2, 1]);
    assert!(matches!(queue.next(), Some(3)));
    assert!(matches!(queue.next(), Some(2)));
    assert!(matches!(queue.next(), Some(1)));
    assert!(matches!(queue.next(), None));
}
