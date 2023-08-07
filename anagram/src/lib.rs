use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    // Convert the characters of the word into a sorted Vec<char>
    let mut sorted_word: Vec<char> = word.to_lowercase().chars().collect();
    sorted_word.sort();

    // Iterate through the possible anagrams
    for &possible_anagram in possible_anagrams {
        // Skip the word itself
        if possible_anagram.to_lowercase() == word.to_lowercase() {
            continue;
        }

        // Convert the characters of the possible anagram into a sorted Vec<char>
        let mut sorted_possible_anagram: Vec<char> = possible_anagram.to_lowercase().chars().collect();
        sorted_possible_anagram.sort();

        // Check if the sorted anagram matches the sorted word
        if sorted_possible_anagram == sorted_word {
            // If they match, add the anagram to the HashSet
            anagrams.insert(possible_anagram);
        }
    }

    anagrams
}

