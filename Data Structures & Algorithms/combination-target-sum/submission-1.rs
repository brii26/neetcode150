impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut path = Vec::new();
        let mut res = Vec::new();
        let mut nums = nums;
        nums.sort();
        Self::dfs(0,&nums, &mut path, &mut res, 0, &target);
        res
    }

    pub fn dfs (idx: usize, nums: &[i32], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, sum: i32, target: &i32) {
        let mut new_sum = sum;

        if new_sum == *target {
            res.push(path.clone());
            return;
        }

        for i in idx..nums.len() {
            new_sum += nums[i];
            if new_sum > *target {
                return;
            }
            path.push(nums[i]);
            Self::dfs(i,nums,path,res,new_sum,target);
            path.pop();
            new_sum -= nums[i];
        }
    }
}
