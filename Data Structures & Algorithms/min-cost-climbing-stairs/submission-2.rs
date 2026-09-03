impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        let n = cost.len();
        if n > 2 {
            for i in (0..=(n - 3) as usize).rev() {
                cost[i] += cost[i + 1].min(cost[i + 2]);
            }
        }
        cost[0].min(cost[1])
    }
}