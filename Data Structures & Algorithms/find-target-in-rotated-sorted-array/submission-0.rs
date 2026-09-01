impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len()-1;
        let mut m = l + (r-l)/2;

        while l!=m && r!=m {
            if nums[r] > nums[m] {
                if target >= nums[m] && target <= nums[r] {
                    l = m;
                } else {
                    r = m;
                }
            } else {
                if target <= nums[m] && target >= nums[l] {
                    r = m;
                } else {
                    l = m;
                }
            }
            m = l+(r-l)/2;
        }
        if nums[r] == target {
            r as i32
        } else if nums[l] == target {
            l as i32
        } else {
            -1
        }
    }
}
