impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut count_zeros = 0;
        let mut product = 1;
        let mut zero_idx = 0;

        // calculate products except for zeros
        for i in 0..nums.len() {
            if nums[i] == 0 {
                count_zeros+=1;
                zero_idx = i;
            } else {
                product *= nums[i];
            }
        }
        
        let mut res = vec![0;nums.len()];
        if count_zeros == 1 {
            res[zero_idx] = product;
        } else if count_zeros == 0 {
            for i in 0..nums.len() {
                if nums[i] == 0 {
                    res[i] = product;
                } else {
                    res[i] = product/nums[i];
                }
            }
        }
        res
    }
}
