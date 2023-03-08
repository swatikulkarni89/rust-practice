#[derive(Debug)]
struct user {
    name:String,
    email: String,
    sign_in_count: u64,
    activ: bool,
}

pub fn run() {
    let mut user = user {
        name:String::from("lita"),
        email:String::from("lita@123.com"),
        sign_in_count: 1,
        activ: bool::from(true),
    };
    
        
    let rect:rectangle=rectangle { lenght: (32), width: (12) };
    println!("{:?}", user);
    println!("{}",area(&rect))

}
fn area(rectangle:&rectangle) ->i32{
    rectangle.lenght*rectangle.width
}
struct rectangle{
    lenght:i32,
    width:i32

}