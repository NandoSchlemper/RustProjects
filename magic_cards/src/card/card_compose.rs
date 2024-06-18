#[derive(Debug)]
pub enum Tipagens {
    Permanent,
    NonPermanent,
}

pub struct CriarCarta {
    pub name: String,
    pub tipagem: Tipagens,
    pub description: String,
    pub card_color: String,
}

impl CriarCarta {
    pub fn mostrar_carta(&self) -> () {
        println!(
            "Nome: {}\nTipagem: {:?}\nDescrição: {}\nCor: {}",
            self.name, self.tipagem, self.description, self.card_color
        );
    }

    pub fn build_card(nome: String, tipo: Tipagens, descrição: String, cor: String) -> CriarCarta {
        CriarCarta {
            name: nome,
            tipagem: tipo,
            description: descrição,
            card_color: cor,
        }
    }
}
