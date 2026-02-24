mod second_code;


fn main() {
for  number in 1..=10{
println!("{}",number*5)
}
println!("Таблица умножения \n ---------------");
second_code::code();
let  y = 1025 ;
let mut f = 0;
let  mut  t = y ;
if  y % 2 != 0 {
    println!("не степень двойки");
   return; 
}

while t > 1 {
    t = t/2;
  f= f + 1 ;
    print!("{} \n",t)
}
println!("это {} степень двойки",f);
}
