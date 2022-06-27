use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_word = sort_string(&lowercase_word);
    let mut anagrams = HashSet::new();
    for &anagram in possible_anagrams {
        let lowercase_anagram = anagram.to_lowercase();
        if lowercase_word == lowercase_anagram { continue }
        if sort_string(&lowercase_anagram) == sorted_word {
            anagrams.insert(anagram);
        }
    }
    return anagrams;
}


fn sort_string(s: &String) -> Vec<char> {
    let mut sorted = s.chars().collect::<Vec<char>>();
    sorted.sort_unstable();
    return sorted;
}