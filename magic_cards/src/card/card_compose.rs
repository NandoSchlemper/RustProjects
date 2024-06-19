use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Tipagens {
    Permanent,
    NonPermanent,
}

#[warn(unused_assignments)]
impl Tipagens {
    pub fn iterate_over_types(&self) -> () {
        for i in Tipagens::iter() {
            println!("{:?}", i);
        }
    }
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

    // Boas praticas em rust
    pub fn card_compose(
        &self,
        name: String,
        tipagem: Tipagens,
        description: String,
        card_color: String,
    ) -> CriarCarta {
        CriarCarta {
            name,
            tipagem,
            description,
            card_color,
        }
    }
}
