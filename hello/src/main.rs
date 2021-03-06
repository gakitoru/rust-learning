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

fn func(code: i32) -> Result<i32, String> {
    println!("code: {}", code);
    Ok(100)
}

fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
    let code = result?;
    println!("code: {}", code);
    Ok(100)
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
    
    println!("{:#?}", e1);

    //let result: Result<i32, String> = Err("fuck!".to_string());
    //let result: Result<i32, String> = Ok(200);
    // match result {
    //     Ok(code) => println!("code: {}", code),
    //     Err(err) => println!("Err: {}", err),
    // };
    // if let Ok(code) = result {
    //     println!("code: {}", code);
    // }

    // let result: Result<i32, String> = Ok(200);
    // println!("code: {}", result.unwrap_or(-1));

    // let result: Result<i32, String> = Err("error".to_string());
    // println!("code: {}", result.unwrap_or(-1));
    
    // let result: Result<i32, String> = Ok(200);
    // let next_result = result.and_then(func);

    // let result: Result<i32, String> = Err("error".to_string());
    // let next_result = result.and_then(func);

    let result: Result<i32, String> = Ok(200);
    let next_result = error_handling(result);
}
