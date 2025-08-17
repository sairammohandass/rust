fn main(){
    
    let nums = [5,6,1,3,2];
    let target:i32=4;
    
    for i in  0..nums.len(){
    
        for j in  i+1..nums.len(){
            
            if (nums[i]+nums[j]==target){
                
                println!("adding indexes of {} and {} gives {}", i,j, target)
            }
        }
        
        
        
    }
    
}
