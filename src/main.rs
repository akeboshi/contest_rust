pub mod io;

fn main() {
    let s: String = io::read();
    println!("{}", s);
}
