pub fn build_proverb(list: &[&str]) -> String {
    // todo!("build a proverb from this list of items: {list:?}")
    if list.is_empty() {
        return String::new(); // or return "".to_string();
    }
    let mut ans = Vec::new();
    for i in 0..list.len() - 1 {
        ans.push(format!("For want of a {} the {} was lost.",list[i], list[i + 1]));
    }
    ans.push(format!("And all for the want of a {}.",list[0]));
    ans.join("\n")
}
