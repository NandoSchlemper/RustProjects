mod card;

use card::card_compose::{ CriarCarta, Tipagens };

fn main() {
    let card_one = CriarCarta {
        name: String::from("Carta numero 1"),
        tipagem: Tipagens::Permanent,
        description: String::from("Uma carta muito foda"),
        card_color: String::from("Azul"),
    };

    card_one.mostrar_carta();
}
