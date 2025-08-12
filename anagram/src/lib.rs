use std::collections::HashMap;
use std::collections::HashSet;
fn counter(s: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for ch in s.chars().flat_map(|c| c.to_lowercase()) {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&'a str]) -> HashSet<&'a str> {
    println!(
        "For the '{word}' word find anagrams among the following words: {possible_anagrams:?}"
    );
    let word_freq = counter(word);
    let mut ans = HashSet::new();
    for &w in possible_anagrams {
        if w.to_lowercase() != word.to_lowercase() && counter(w) == word_freq{
                ans.insert(w);
            }
    }
    ans
}
