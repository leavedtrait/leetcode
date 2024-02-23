
pub struct Solution {}
// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

// Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:

//     Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
//     Return k.

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // nums.retain(|&x| x != val);
        // println!("{:?}", nums); // Print the contents of the filtered vector
        // println!("Size: {}", nums.len()); // Print the size of the filtered vector
        // nums.len() as i32 // Return the new length
        let mut index = 0;
        for x in 0..nums.len(){
            if nums[x] != val {
                nums[index] = nums[x];
                index += 1; 
            }
        }
        index as i32
    }
}
fn main() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4,2];
    let x = Solution::remove_element(&mut nums, 2);
    println!("{:?}", nums);
}
