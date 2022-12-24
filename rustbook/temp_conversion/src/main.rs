use std::io;
fn main() {
    println!("🌡️  Enter temp in F");
    let mut fahren = String::new();
    io::stdin().read_line(&mut fahren).expect("Error");
    let fahren: f64 = fahren.trim().parse().expect("Error parsin");
    let celsius = (fahren - 32.0) / 1.8;
    println!("🌡️  {} in celsius is {:.1}", fahren, celsius);
}
