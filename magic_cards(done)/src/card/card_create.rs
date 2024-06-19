use crate::card::card_compose::{CriarCarta, Tipagens};
use std::io;

pub fn get_value(mut x: String) -> String {
    io::stdin()
        .read_line(&mut x)
        .expect("Valor inserido NAO consta como tipo --> String !!!");
    x
}

pub fn binary_comp_types(x: String) -> (Tipagens) {
    let mod_x = x.trim().parse();
    if mod_x == 1 {
        let response = Tipagens::Permanent;
        response
    } else if mod_x == 2 {
        let response: Tipagens = Tipagens::NonPermanent;
        response
    } else {
        println!("Numero informado esta incorreto!");
        None
    };
}

pub fn full_card_creation() -> CriarCarta {
    println!("Bem vindo a nosso criador de cartas Magic!\n");
    println!("(1/4) Digite um nome para sua carta!");

    // Definindo nome da carta
    let mut card_name = String::new();
    let user_card_name: String = get_value(card_name);
    // -- > -- | -- < --

    println!("(2/4) Qual vai ser o tipo da sua carta?\n");
    let mut card_type: Tipagens;
    card_type.iterate_over_types();
}
