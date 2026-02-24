mod vectors;
fn main() {
let x = [1,4,6,7,3,9,5];
  let mut sum = 0 ;
  for r in &x {
sum +=r;
      
  }
  for r in &x {
println!("{}",r)
      
  }
  println!("{}",sum);
  vectors::vectors();
}
