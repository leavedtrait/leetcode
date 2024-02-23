use std::collections::HashMap;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);  // Output: [0, 1] (because nums[0] + nums[1] = 2 + 7 = 9)
}

struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<&i32, usize> =HashMap::new();
       for (i , num) in nums.iter().enumerate(){
           let required_no = target - num;
           if let Some(j)= map.get(&required_no){
               return vec![*j as i32, i as i32];
           }
           map.insert(num ,i);

       }
       vec![]
    }
}