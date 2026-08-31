impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut l = 0;
        let mut r = s.len()-1;

        let str = s.as_bytes();
        while l < r {
            while l < r && !str[l].is_ascii_alphanumeric() {
                l+=1;
            }
            while l < r && !str[r].is_ascii_alphanumeric() {
                r-=1;
            }
            if l >= r {
                break;
            }
            if str[l].to_ascii_lowercase() != str[r].to_ascii_lowercase() {
                return false;
            }
            l+=1;
            r-=1;
        }

        true
    }
}
