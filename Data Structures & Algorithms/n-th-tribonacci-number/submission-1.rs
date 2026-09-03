impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0,0);
        map.insert(1,1);
        map.insert(2,1);
        Self::tri_helper(&mut map, n)
    }

    pub fn tri_helper(map: &mut HashMap<i32,i32>, n: i32) -> i32 {
        if map.contains_key(&n) {
            *map.get(&n).unwrap()
        } else {
            let a = Self::tri_helper(map, n-1); 
            let b = Self::tri_helper(map, n-2);
            let c = Self::tri_helper(map, n-3);
            map.insert(n-1, a);
            map.insert(n-2, b);
            map.insert(n-3, c);
            a + b + c
        }
    }
}
