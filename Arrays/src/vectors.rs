
pub fn vectors(){
    let mut vector: Vec<i32>  = Vec::new();
    let mut results: Vec<i32> = Vec::new() ;
    for t  in 1..20  {
vector.push(t);
    }
    for r in &vector  {
        if r%2 ==0 {
            results.push(*r*10);
        }
        
    }
    println!();
    for g in &results  {
        print!("{} ",g)
    }
}