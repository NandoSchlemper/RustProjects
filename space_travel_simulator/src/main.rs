struct Nheco {
    usuário: String,
}

fn compare_planets(val1: &str, val2: &str) -> bool {
    match val1.to_lowercase().as_str() {
        val2 => true,
        _ => false,
    }
}

fn main() {
    let user1: Nheco = Nheco {
        usuário: String::from("user1")
    };

    println!("Digite o usuário");
    let mut user_input = String::new();
    use std;
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Deu merda");

    let response = compare_planets(&user1.usuário, &user_input);

    match response {
        true => println!("São iguais"),
        false => println!("São diferentes"),
        _ => println("Deu merda")
    }
}
