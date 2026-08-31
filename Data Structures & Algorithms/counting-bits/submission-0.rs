impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut count = 0;
        let mut res = Vec::new();
        for i in 0..n+1 {
            let mut j = i;
            while j > 0 {
                if j&1 == 1 {
                    count+=1;
                }
                j >>= 1;
            }
            res.push(count);
            count = 0;
        }
        res
    }
}
