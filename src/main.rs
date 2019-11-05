mod animal;
use animal::Animal;

fn main() {
    let x = 42;

    let thor = Animal {
        name: String::from("Thor"),
        age: 0,
    };


    println!("His name is: {}", thor.name);
    println!("And he is {} years old", thor.name);
}
