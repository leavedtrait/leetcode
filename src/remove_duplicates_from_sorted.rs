fn main() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let u = Solution::remove_duplicates(&mut nums);
    println!("{u}")
}
struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut left = 1;
        for right in 1..nums.len(){
            if nums[right] != nums[right-1]{
                nums[left]= nums[right];
                left +=1;
            } 
        }
        left as i32 
    }
}