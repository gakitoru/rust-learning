struct Color {
    r: i32,
    g: i32,
    b: i32,
}


fn calc_data(data: &String) {
    println!("{}", data);
}

fn main() {
    let important_data = "Hello, World!".to_string();

    calc_data(&important_data);

    println!("{}", &important_data);

    let y;

    {
        let x = 5; 
        y = &x;
        dbg!(x);
    }
    dbg!(y);
}
