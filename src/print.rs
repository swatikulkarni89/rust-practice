pub fn run() {
    println!("print hello from print file");

    //formatting arguments
    println!("number is {}", 1);

    println!("{} number is {}", "even", 2);

    //positional argument
    println!("{0} number is {1} and {0} nmber is {2} too  ", "even", 2, 4);

    //named arguments
    println!(
        "{name} likes to do {activity} a lot",
        name = "piku",
        activity = "play"
    );
    //placeholder traits
    println!(
        "binary for number 10 is {:b}, hex is {:x} and octal for that is {:o}",
        10, 10, 10
    );

    //placeholder for debbug traits
    println!("{:?}", (2, "ad is ", "cary"));
}
