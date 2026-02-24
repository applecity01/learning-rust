pub fn code(){
let mut x = 1;
let final_result=loop {
x = x* 2;
if x > 100 {
    break x;
}
};
println!("{}",final_result)
}