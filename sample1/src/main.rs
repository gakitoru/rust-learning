fn main() {
    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number < 0 {
        println!("number < 0");
    } else {
        println!("0 == number");
    }

    let number = 1;
    let result = if 0 <= number {
        number
    } else {
        -number
    };

    println!("{:?}", result);

    let mut count = 0;
    let result = loop {
        println!("count: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("{:?}", result);

    let mut count = 0;

    while count < 10 {
        println!("count: {}", count);
        count += 1;
    }

    let count: i32;

    for count in 0..10 {
        println!("count: {}", count);
    }

    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for element in &array {
        println!("element: {}", element);
    }

    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");

            break 'main;
            println!("sub loop end");
        }
        println!("main loop end");
    }


    let i: i32 = 10;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("misc"),
    }

    enum Color {
        Red,
        Blue,
        Green,
    }

    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }

    
    let result: Result<i32, String> = Err("error".to_string());
    let result_number = match result {
        Ok(number) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        },
    };
    println!("{}", result_number);

    for number in 1..=5 {
        println!("{}", number);
    }

    // let v = vec![1, 2, 3, 4, 5];
    // println!("{}", v[0]);

    // for element in &v {
    //     println!{"{}", element};
    // }

    // let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    // print(Box::new(byte_array));
    // let byte_array = [b'w', b'o', b'r', b'l', b'd'];
    // print(Box::new(byte_array));

    // let immut_val = 10;
    // let mut mut_val = 20;

    // mut_val += immut_val;
    // println!("{}", mut_val);

    // let v1: u64 = 10;
    // let v2 = 10u64;

}

// fn print(s: Box<[u8]>) {
//     println!("{:?}", s)
// }
