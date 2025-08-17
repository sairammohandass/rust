// fibo

fn main(){
    
    let mut n=6;
    println!("{}",fibo(n));
}
fn fibo(n :u32)->u32{
    
    if n<=1{
        return n;
    }
    fibo(n-1)+fibo(n-2)
}
