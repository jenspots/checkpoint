fn main() {
    println!("Hello, world!");
}

fn greeting(name: &String) -> String {
    format!("Hello, {name}!")
}

#[test]
fn test_greeting() {
    assert!(greeting(&"World".to_string()).eq("Hello, World!"));
}
