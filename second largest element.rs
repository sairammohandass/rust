// second largest element.


//[45,1,2,3,56,89,45,54]

fn main(){

let  arr=vec![45,1,2,3,556,89,45,54];

   println!("{}", s_largest(arr));
}

fn s_largest(arr: Vec<i32>) -> i32{
  
  
   let mut max1;
   let mut max2;
    if arr[0]>arr[1]{
         max1 =arr[0];
         max2 = arr[1];
    }
    else {
    max2= arr[0];
    max1= arr[1];
    }
    for i in 2..arr.len(){
        
        if arr[i]> max1{
            
            max2=max1;
            max1=arr[i];
        }
        else if arr[i]>max2{
            
            max2= arr[i];
        }
        
    }
  
    return max2;
}
