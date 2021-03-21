fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn abs(number: i32) -> i32 {
    if number < 0 {
        return -number;
    }
    number
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age: age,
        }
    }

    fn say_name(&self) -> &Self {
        println!("I am {}.", self.name);
        self
    }
    
    fn say_age(&self) -> &Self {
        println!("I am {} year(s) old.", self.age);
        self
    }
}

fn main() {
    let p = Person::new("Taro", 20);
    p.say_name().say_age();
}