#[derive(Debug)]// so that debug trait works
  struct User{
        
        
        number :i32,
        phone_type: String,
        address: String,
        
        
    }

fn main() {

    let user=User{
        
        number:8778,
        phone_type:String::from("realme `10 pro plus"),
        address:String::from("pondy"),
        
        
    };
  
    println!("{:?}", user); //! macro doesnt know to print the entire struct so we use debug trait :? 
}
