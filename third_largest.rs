
//[2,5,91,12,7,54,3]

fn main (){
    
    
    let arr=vec![2,5,91,12,7,54,54,3];
    println!("{}", s_mac(arr));
    
    
}
fn s_mac(arr:Vec<i32>)-> i32{
    
    let mut max1=0;
    let mut max2=0;
    let mut max3=0;

    
    for i in 0..arr.len(){
        
        if arr[i]>max1{
        max3=max2;
        max2=max1;
        max1=arr[i];
        }
        else if arr[i]>max2 && arr[i]!=max1{
            max3=max2;
            max2=arr[i];

        }
        else if arr[i]>max3 && arr[i]!=max1 &&arr[i]!=max2
        {
            
        max3=arr[i];
            
        }
        
    }
    max3
    
    
    
}
