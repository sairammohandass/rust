//check if array is sorted



fn main(){
    
    
    let nums = vec![4,5,7,8,61,89,0];
    println!("{}", check(nums));
    
    
}
fn check(nums:Vec<i32> )-> bool{
    
    let mut i =0;
    for j in 1..nums.len(){
        
        if nums[j]<nums[i]{
            
            return false;
        }
        i=i+1;
        
        
        
        
    }
    return true;
    
    
    
}
