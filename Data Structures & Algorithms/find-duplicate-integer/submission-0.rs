impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for el in nums {
            if !set.insert(el) {
                return el;
            }
        }
        -1
    }
}
