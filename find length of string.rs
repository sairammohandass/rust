fn str_length(s: String) -> usize{
    
    s.chars().count()
}

fn main(){
    
   let  my = String::from("omalaaaaa aaa");
    let length=str_length(my);
    println!("the count is {}", length );
}
