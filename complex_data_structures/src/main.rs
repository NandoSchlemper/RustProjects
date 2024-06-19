use std::collections::HashMap;

fn main() {
    // Hashmaps
    let mut data: HashMap<_, _> = HashMap::new();
    data.insert(String::from("Batata"), 1);
    data.insert(String::from("Abacate"), 2);

    let fruta = String::from("Abacate");
    let search_data = data.get(&fruta).copied().unwrap_or(0);
    println!("{}", search_data);
}
