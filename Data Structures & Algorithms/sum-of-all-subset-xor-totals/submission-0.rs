impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut xor = 0;
        let mut sum = 0;
        Self::dfs(0, &nums, &mut xor, &mut sum);
        sum
    }


    pub fn dfs (idx: usize, nums: &[i32], xor: &mut i32, sum: &mut i32) {
        *sum += *xor;
        let n = nums.len();
        for i in idx..n {
            *xor ^= nums[i];
            Self::dfs(i+1,nums,xor,sum);
            *xor ^= nums[i];
        }
    }
}