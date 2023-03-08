pub fn run(){

let num: [i32;5]= [1,2,3,4,5];
println!("print array {:?}",num);
println!("print single value {:?}",num[0]);
println!("print array lenght {:?}",num.len());

//stack allocatedSS
println!("print allocated memory {:?}",std::mem::size_of_val(&num));
//slice of array


}