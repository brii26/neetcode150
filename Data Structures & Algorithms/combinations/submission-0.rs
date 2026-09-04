impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut path = Vec::new();
        Self::dfs(1,0,n,k,&mut path, &mut res);
        res
    }

    pub fn dfs(idx: usize, ctr: i32, n: i32, k: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        let mut new_ctr = ctr;
        if new_ctr == k {
            res.push(path.clone());
            return;
        }

        for i in idx..=n as usize{
            path.push(i as i32);
            new_ctr+=1;
            Self::dfs(i+1, new_ctr, n , k, path ,res);
            path.pop();
            new_ctr-=1;
        }
    }
}
