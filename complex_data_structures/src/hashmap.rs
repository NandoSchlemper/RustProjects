pub mod hash_data {
    use std::collections::HashMap;
    pub fn hash_map_explain() -> () {
        /*
        Hashmaps
        Instanciando o HashMap sem valores predefinidos
        */
        let mut data: HashMap<_, _> = HashMap::new();

        // Inserindo dados no hashmap
        data.insert(String::from("Batata"), 1);
        data.insert(String::from("Abacate"), 2);
        println!("Capitulo 1: Hashmaps");
        println!("Criando o hashmap: {:?}", data);

        // Definindo uma fruta para ser procurada como KEY dentro do hashmap
        let fruta = String::from("Abacate");

        /*
        Pega o VALOR da KEY que procuramos!
        Utilizamos .copied() para aplicar a trait Copy
        que permite copiarmos o valor de get sem inutilizar o hashmap (Ownership)
        Enquanto o unwrap é utilizado para extrair os valores de option32
        */
        let search_fruta = data.get(&fruta).copied().unwrap();
        println!(
            "Pegando o VALOR da KEY 'Abacate', sendo ele: {}",
            search_fruta
        );

        println!("Iterando sobre o HashMap:");
        for (k, v) in &data {
            println!("Valor-Chave: {k},  Valor-Atribuído: {v}");
        }

        // Atualizando o valor de Abacate para 10
        // Quando criamos uma variavel e adicionamos ela no HashMap, ele pega seu Ownership
        // Exemplo:
        let x = String::from("Laranja");
        let y: i32 = 3;

        data.insert(x, y);
        // A partir daqui, o HashMap Data, chumba o ownership de X e Y, ou seja, nao podemos mais
        // Utilizar as variavels x e y!
    }
}
