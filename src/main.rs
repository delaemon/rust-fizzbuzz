fn main() {
    for i in 0..30 {
        let fizz = if i % 3 == 0 { "fizz" } else { "" };
        let buzz = if i % 5 == 0 { "buzz" } else { "" };
        println!("{} {}{}", i, fizz, buzz);
    }
}
