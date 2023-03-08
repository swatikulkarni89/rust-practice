pub fn run(){

    let i=2;
    let j=4.5;
    let k:i64 =646464646464;
    println!("i:{}, j:{}, k:{} ", i, j, k);

println!("size of int32 is {}", std::i32::MAX);
println!("size of int64 is {}", std::i64::MAX);

println!("{:?} ", (i, j, k,std::i32::MAX));

}