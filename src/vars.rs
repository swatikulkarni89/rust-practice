pub fn run() {
    let name = "swati";
    println!("my name is {}", name);
    let mut age = 34;
    println!("my name is {} and I am {}", name, age);
    age = 35;
    println!("my name is {} and I am {}", name, age);
    //constant values
    const ID: i32 = 12;
    println!("id is {}", ID);

    // asigining multiple veriable
    let (myname, myage) = ("swati", 34);
    println!("{} is {}", myname, myage);
}
