//[0,1,0,3,12]
//[1,3,12,0,0]

fn main(){
    
    
    let mut v = vec![2,1];
    let n = v.len();
    let mut count =0;
    // println!("{}",n);
    let mut i =0;
    
    for j in 0..v.len(){
        
        if v[j] ==0{
            count+=1;
        }
        else{
            
            v[i]=v[j];
            
            if i != j {   
                v[j] = 0;
            }
            i +=1;
            
        }
    }
     println!("{} {}",n,count);
      println!("{:?}",v);
    
    
    
    
    
    
    
}
