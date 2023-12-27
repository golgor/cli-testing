fn main() {
    let loops = adder::funcs::testare(1);
    for i in 0..loops {
        println!("Hello, world! {}", i);
    }
}
