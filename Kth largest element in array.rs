use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        
        let mut heap= BinaryHeap::new();
        // 3,2,1,5,6,4
        for i in nums{

            heap.push(Reverse(i));
            if heap.len()>k.try_into().unwrap()
            {
                heap.pop();
            }

           
            

        }  heap.peek().unwrap().0
       




    }
}
