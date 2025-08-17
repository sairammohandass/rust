//find consecutive 1's
//[0,1,1,1,0,0,0,1,1,1,1,0,0,1]
//output =4 


fn main(){
    
    
    let mut arr= [0,1,1,1,1,1,0,0,0,1,1,1,1,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    let mut count =0;
    let mut max =0;
    for i in 0..arr.len(){
        
        if arr[i]==1{
            
             count+=1;
             while count>max{
         max=count;
     }
        }
         
        else{
            
     count=0;
    
     
     }
   
        
    }   println!("{}", max);
    
}
