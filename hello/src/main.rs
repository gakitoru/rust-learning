#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
enum Event {
    Quit,
    KeyDown(u8),
    MouseDown {x: i32, y: i32}
}

fn main() {
    let s1: String = String::from("Hello World!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("{0}, {1}, {2}", s1, s2, s3);

    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    println!("{:#?}", t);

    let mut a: [i32; 3] = [0, 1, 2];
    println!("{:#?}", a);

    let b: [i32; 3] = [0; 3];
    println!("{:#?}", b);

    a[1] = b[1];
    a[2] = b[2];

    println!("{:?}", &a[1..3]);


    let p = Person {
        name: String::from("John"),
        age: 8,
    };

    println!("{:#?}", p);

    let e1 = Event::Quit;
    let e2 = Event::MouseDown{x: 10, y: 10};
    
    println!("{:#?}", e1)
}