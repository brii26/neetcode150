impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut path = Vec::new();
        let mut res = Vec::new();
        let mut candidates = candidates;
        candidates.sort();
        Self::dfs(0, &candidates, &mut path, &mut res, target, 0);
        res
    }

    pub fn dfs (idx: usize, candidates: &[i32], path: &mut Vec<i32>, 
                res: &mut Vec<Vec<i32>>, target: i32, sum: i32) {
        let mut new_sum = sum;
        if new_sum == target {
            res.push(path.clone());
            return;
        }
        let mut first_iter = true;
        for i in idx..candidates.len() {
            if first_iter {
               first_iter = false; 
            } else {
                if candidates[i] == candidates[i-1] { continue; }
            }
            new_sum += candidates[i];
            if new_sum > target {
                return;
            }
            path.push(candidates[i]);
            Self::dfs(i+1, candidates, path, res, target, new_sum);
            path.pop();
            new_sum -= candidates[i];
        }
    }
}
