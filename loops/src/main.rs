fn main() {
    let mut counter =3;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {}", result);
}
