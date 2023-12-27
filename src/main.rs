fn main() {
    let loops = cli_interface::funcs::testare(1);
    for i in 0..loops {
        println!("Hello, world! {}", i);
    }
}
