impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut path = Vec::new();
        let mut res = Vec::new();
        Self::sub_helper(0,&nums, &mut path, &mut res);
        res
    }

    pub fn sub_helper(start: usize, nums: &[i32], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(path.clone());
        let n = nums.len();
        for i in start..n {
            path.push(nums[i]);
            Self::sub_helper(i+1,nums,path, res);
            path.pop();
        }
    }
}
