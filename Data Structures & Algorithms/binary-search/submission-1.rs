impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {

        if nums.is_empty() {
            return -1;
        }
        let mut low: i32 = 0;
        let mut high = nums.len() as i32 - 1;

        while low <= high {
            let mid = low + (high - low) / 2;

            if nums[mid as usize] == target {
                return mid as i32;
            }

            if nums[mid as usize] < target { 
                low = mid+1;
            } 

            if nums[mid as usize] > target {
                high = mid-1;
            }
        }
        -1
    }
}


