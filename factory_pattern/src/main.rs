trait Pet {
    fn get_name(&self) -> String;
}

type PetFactory = fn (name: &str) -> Box<dyn Pet>;

struct Dog {
    name: String,
}

impl Dog {
    fn new(name: &str) -> Box<dyn Pet> {
        Box::new(Self {
            name: name.to_string(),
        })
    }
}

impl Pet for Dog {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("Dog {} dropping", self.name);
    }
}

struct Person {
    name: String,
    p: Box<dyn Pet>,
}

impl Person {
    fn new_with_pet(name: &str, f: PetFactory, pet_name: &str) -> Self {
        Self {
            name: name.to_string(),
            p: f(pet_name),
        }
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Person {} dropping", self.name);
    }
}

fn run() {
    println!("in run");

    let p = Person::new_with_pet("Warren", Dog::new, "Wilma");

    println!("{} has {}", p.name, p.p.get_name());
    println!("{} has {}", p.name, p.p.get_name());

    println!("out of run");
}

fn main() {
    println!("in main");
    run();
    println!("out of main");
}
