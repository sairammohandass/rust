// find the target
// [3,2,0,1,5]
//target 5
//op=> 4
//binary search
fn main (){
    
    
    let mut arr= [0,3,4,21,30,38,44,71];
    let target =328;
    let mut start =0;
    let mut end = arr.len()-1;
   
    // println!("start:{} mid:{} end:{}", start,mid,end);
    
   while start<=end{
    let mut mid = (start+end)/2;
        if target ==arr[mid]{
        
        println!("{}",mid);
        return;
        }
        else if target<arr[mid]{
            end=mid-1;
        }
        else  {
            
            start= mid+1;
        }
      
        
    }
  println!("not found");  
    
    }
