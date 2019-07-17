pub fn clip_and_clean(input: String, clip: usize) -> String {
    let output = input.get(clip..).expect("checked earlier");
    clean(String::from(output))
}

pub fn clean(input: String) -> String {
    String::from(input.trim_start())
}

pub fn find_first(input: &String, chars_to_find: Vec<String>) -> Option<usize> {
    chars_to_find.iter()
        .map(|c| input.find(c))
        .filter(|oc| oc.is_some())
        .map(|oc| oc.unwrap())
        .min()
}
