fn main() {
    println!("Hello, world!");

    let x = 3;
    if x == 3 {
        println!("The x is: {x}");
    } else if x > 0 {
        println!("Positive integer");
    } else {
    }

    let cond: bool = false;
    let var: i16 = if cond { 3 } else { -3 };
    println!("This is var: {var}");

    // // infinie loop
    // loop {
    //     println!("Infinte Looop");
    // }
    //

    let mut counter: u32 = 0;
    let y: u32 = loop {
        let _flag: bool = false; // the underscore means: I know that this is unsed variable
        if counter == 20 {
            break counter * 3;
        }
        counter += 1;
    };
    println!("Y is: {y}");

    // Breaking using the loop `label`
    let mut outer_counter = 0;
    let sum;
    'upper_loop: loop {
        outer_counter += 1;
        if outer_counter == 20 {
            let mut i = 0;
            let mut outer_sum = 0;
            loop {
                i += 1;
                outer_sum += i;
                if i == outer_counter {
                    sum = outer_sum;
                    break 'upper_loop;
                }
            }
        }
    }
    println!("The sum is : {sum}");

    // While Loop
    let mut i = 0;
    let mut sum = 0;
    while i < 10 {
        i += 1;
        sum += 1;
    }
    println!("The sum is: {sum}");

    let arr = [1, 2, 3, 4];
    for x in arr {
        println!("The x is : {x}");
    }

    for i in 0..4 {
        println!("i: {i}");
    }

    // reversing the loop
    println!("\n\n\n");
    for i in (0..4).rev() {
        println!("i: {i}");
    }
}
