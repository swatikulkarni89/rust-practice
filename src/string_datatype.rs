pub fn run(){

let  mut str1= String::from("string1 ");
println!{"print str1 {}", str1};

str1.push('p');
str1.push('r');
str1.push_str("int");

println!{"print str1 {}", str1};
//str1.capacity();
println!{"print str1 capacity {}", str1.capacity()};

assert_eq!(2,2);


}
