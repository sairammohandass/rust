use std::io;

fn main()
{
 
 println!("{}",mn());// fun call

}

fn mn() -> bool{

let mut io = String::new(); //ownership of io

    
    io::stdin().read_line(&mut io).expect("Failed to read line"); //refernce. only once
    let num:i32=io.trim().parse().expect("");
    // println!("num : {}", num);
    if num %2==0
    {
    return true;
    //   println!("even ");
    }
    return false;
}

