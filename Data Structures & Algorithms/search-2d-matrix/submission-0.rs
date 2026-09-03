impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut l = 0;
        let mut r = row*col-1;
        let mut m = l + (r-l)/2;
        // r_idx = i/col;
        // c_idx = i%col;

        while l!=m && r!=m {
            let mid_value = matrix[m/col][m%col];
            if target > mid_value {
                l = m;
            } else if target < mid_value {
                r = m;
            } else {
                return true;
            }
            m = l + (r-l)/2;
        }
        
        let r_val = matrix[r/col][r%col];
        let l_val = matrix[l/col][l%col];
        if target == r_val || target == l_val {
            true
        } else {
            false
        }
    }
}
