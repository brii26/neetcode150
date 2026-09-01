impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0 ;
        let mut mid = nums.len()/2;
        let mut r = nums.len()-1;

        while l != mid && mid != r {
            if nums[r] < nums[mid] {
                l = mid;
                mid = mid + (r-mid)/2;
            } else {
                r = mid;
                mid = mid - (mid-l)/2;
            }
        }
        if nums[l] < nums[r] {nums[l]} else {nums[r]}
    }
}

// a conditino where nums[r] < nums[mid], should be in a state where
// it had been rotated to the right, which implies nums[r] should be < then
// nums [l] => smallest at the right segment

// nums[r] > nums[mid], smallest at the left segment

// how to handle 3 pointers shift && size constraint under the size 3
// while l != mid != r, where mid is ofset + distance
// termination resolved in returning min(l,(min(mid,r)))