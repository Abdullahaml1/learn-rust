use add_one::add;
fn main() {
    println!("Adding 3, 4 = {}", add(3, 4));
    println!("Adding 3, 4 = {}", add_one::add(3, 4));
}
