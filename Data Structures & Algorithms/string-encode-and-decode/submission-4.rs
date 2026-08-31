impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut result = String::new();
        for word in strs {
            result += &format!{"{word}😊"};                
        }
        result
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut entry = String::new();
        let mut result = Vec::new();
        let mut chars = s.chars();
        while let Some(ch) = chars.next() {
            if ch == '😊' {
                result.push(entry);
                entry = String::new();
            } else {
                entry.push(ch);
            }
        }
        result
    }
}
