// find the missing number
// [3,2,0,1,5]

fn main (){
    
    
    let mut arr= [3,2,0,1,5];
    let mut n = arr.len();
    println!("{}", n);
    let mut sum_of=0;
    //sum of first n numbers is n(N+1)/2 5(6)/2=15
    for i in arr{
        sum_of+=i;
    }
    let sum =  n*(n+1)/2;
    let mut answer = sum-sum_of;
    println!("{}", answer);
}
