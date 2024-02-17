class Solution(object):
    def two_sum(nums, target):
    num_indices = {}
    for i, num in enumerate(nums):
        complement = target - num
        if complement in num_indices:
            return [num_indices[complement], i]
        num_indices[num] = i
    return None

# Test case
nums = [2, 7, 11, 15]
target = 9
print(two_sum(nums, target))  # Output: [0, 1] (because nums[0] + nums[1] = 2 + 7 = 9)

nums = [2,7,11,15]
target = 9
Solution.twoSum(self= object,nums=nums,target=target)