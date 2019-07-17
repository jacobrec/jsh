pub fn consume(input: String, expect: &str) -> Result<String, String> {
    if input.starts_with(expect) {
        Ok(String::from(input.get(expect.len()..).expect("Failed to remove head of string")))
    } else {
        Err(input)
    }
}

pub fn clip_and_clean(input: String, clip: usize) -> String {
    let output = input.get(clip..).expect("checked earlier");
    clean(String::from(output))
}

pub fn clean(input: String) -> String {
    String::from(input.trim_start())
}
