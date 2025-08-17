//remove duplicate in sorted array!!


//[1,1,2,3,3]
//o/p => 3 for [1,2,3]
fn main(){
    
    let mut arr = [1,1,2,3,3];
    
    let mut i =0;
    for j in 1..arr.len(){
        
        if arr[i]!=arr[j]{
            
            arr[i+1]=arr[j];
            i+=1; //
            
        }
        
        
        
    }
    println!("{}", i+1);//3
    println!("{:?}", arr);//[1, 2, 3, 3, 3]
    
    
    
    
    
    
}
