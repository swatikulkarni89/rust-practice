pub fn loop_N_conditions() {
    let number: i32 = 10;

    if number > 5 {
        println!("condition 1 true");
    } else if number < 20 {
        println!("condition 2 true");
    } else {
        println!("both conditions failed");
    }

    let codition: bool = true;
    let number_1: i32 = if codition { 5 } else { 6 };
    println!("condition 2 {}", number_1);

    loop {
        println!("again!");
        break;
    }
    let mut counter: i32 = 1;

    loop {
        if (counter >= 10) {
            break;
        }
        counter += 1;
        println!("{}", counter)
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //let mut element: i32=0;
    for element in a.iter() {
        println!("each element of loop {}", element);
    }

    for number1 in 1..4 {
        println!("number {}", number1);
    }
}
