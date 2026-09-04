impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut path = Vec::new();
        let mut used = vec![false;nums.len()];
        Self::dfs(&nums, &mut path, &mut res, &mut used);
        res
    }

    pub fn dfs (nums: &[i32], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, used: &mut Vec<bool>) {
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] {
                continue;
            } else {
                used[i] = true;
                path.push(nums[i]);
                Self::dfs(nums, path , res, used);
                path.pop();
                used[i] = false;
            }
        }
    }
}
