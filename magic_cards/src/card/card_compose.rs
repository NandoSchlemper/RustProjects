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

    pub fn build_card(&self) -> Struct {
        let carta_usuario = CriarCarta {
            name: nome,
            tipagem,
            description,
            card_color,
        };
        carta_usuario
    }
}
