pub fn run(){
    let num: Vec<i32> = vec![1,2,3,4,5];
    println!("print vector {:?}",num);
    println!("print single value {:?}",num[0]);
    println!("print vector lenght {:?}",num.len());
    
    //stack allocatedSS
    println!("print allocated memory {:?}",std::mem::size_of_val(&num));
    //slice of array
    

}