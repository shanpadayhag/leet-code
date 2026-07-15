use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(text: String) -> i32 {
        let mut last_seen_index: HashMap<char, usize> = HashMap::new();
        let mut window_start = 0;
        let mut longest = 0;

        for (current_index, current_char) in text.chars().enumerate() {
            if let Some(&previous_index) = last_seen_index.get(&current_char) {
                if previous_index >= window_start {
                    window_start = previous_index + 1;
                }
            }
            last_seen_index.insert(current_char, current_index);
            longest = longest.max(current_index - window_start + 1);
        }

        longest as i32
    }
}
