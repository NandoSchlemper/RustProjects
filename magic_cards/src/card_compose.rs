pub mod create_magic_card {
    // Modifica a construção da carta
    enum Tipo {
        Artefato(String),
        Encantamento(String),
        Feitiço(String),
        MágicaInstantanea(String),
        Planeswalker(String),
        Terreno(String),
    }

    // Tipagem da carta, não modifica a construção dela
    enum Cor {
        Preta,
        Azul,
        Branca,
        Verde,
        Vermelho,
    }

    struct NewCard {
        name: String,
        description: String,
    }
}
