struct PixPayment {
    recipient_name: String,
    recipient_key: String,
    amount: f64,
    description: String,
}

impl PixPayment {
    fn new(recipient_name: String, recipient_key: String, amount: f64, description: String) -> Self {
        PixPayment {
            recipient_name,
            recipient_key,
            amount,
            description,
        }
    }

    fn process_payment(&self) {
        // Aqui você implementaria a lógica real de processamento de pagamento Pix,
        // que envolveria interação com uma API de pagamento e autenticação adequada.
        println!("Processando pagamento Pix:");
        println!("Beneficiário: {}", self.recipient_name);
        println!("Chave Pix: {}", self.recipient_key);
        println!("Valor: R$ {:.2}", self.amount);
        println!("Descrição: {}", self.description);
        println!("Pagamento processado com sucesso!");
    }
}

fn main() {
    let payment = PixPayment::new(
        String::from("Nome do Beneficiário"),
        String::from("chave-pix@exemplo.com.br"),
        100.0,
        String::from("Pagamento de exemplo"),
    );

    payment.process_payment();
}
