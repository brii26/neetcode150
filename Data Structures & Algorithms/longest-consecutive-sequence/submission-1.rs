impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // create a hashset from an input nums
        // from that hashset get keys
        // from list of keys iterate
        // for each keys lookup on hashset1 while increment counter
        // if found insert to hashset2, else continue to the next hashset1 key
        // if the hashset1 key found on hashset2 which had beed called previously within the iteration
        // continue
        // return max len

        let mut set1 = HashSet::new();
        let mut set2: HashSet<i32> = HashSet::new();

        for el in nums {
            set1.insert(el);
        }

        let mut max = 0;
        for el in set1.iter() {
            let mut ctr = 1;
            if set2.contains(el) {
                continue;
            } else {
                set2.insert(*el);
            }
            let mut val = *el+1;
            while set1.contains(&val) {
                set2.insert(val);
                ctr += 1;
                val += 1;
            }
            max = if max < ctr {ctr} else {max};
        }

        max
    }
}
