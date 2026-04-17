fn main() {
    println!("Hello, world!");
    another_function(33);
    let func_var = another_function;
    func_var(3);

    let (x, c) = another_functon_with_return(9, 'b');
}

fn another_function(x: i32) {
    println!("This is: {x}")
}
fn another_functon_with_return(x: i32, y: char) -> (i32, char) {
    // This function returns a tuple
    return (x * 3, 'a');
}
