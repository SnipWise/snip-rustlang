struct Human {
    name: String,
    age: u32,
    hair_color: String,
}

fn main() {
    let human = Human {
        name: "Alice".to_string(),
        age: 30,
        hair_color: "Brown".to_string(),
    };

    println!("Hello, my name is {} and I am {} years old. My hair color is {}.", human.name, human.age, human.hair_color);
}
