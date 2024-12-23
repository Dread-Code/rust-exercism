use core::str;
use std::collections::HashSet;
//
// pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
//     let word_length = word.len();
//
//     let mut word_hash: HashMap<String, i32> = HashMap::new();
//     // Implement a map to count the letters
//     for letter in word.chars() {
//         match word_hash.entry(letter.to_string()) {
//             Entry::Vacant(entry) => {
//                 let num: i32 = 1;
//                 entry.insert(num);
//             }
//             Entry::Occupied(mut entry) => {
//                 let num: i32 = 1;
//                 let number = entry.get();
//                 *entry.get_mut() = number + num;
//             }
//         }
//     }
//
//     // Filter the words with the smae lenght
//     let words_with_same_length = possible_anagrams
//         .iter()
//         .filter(|item| item.len() == word_length);
//     println!("words_with_same_length => {:?}", words_with_same_length);
//
//     let mut anagrams = HashSet::new();
//     // check if the letters exist in the word_hash and Add the word
//     for ele_word in words_with_same_length {
//         let mut vector: Vec<char> = Vec::new();
//         for ele_letters in ele_word.chars() {
//             if let Some(_value) = word_hash.get(&ele_letters.to_string()) {
//                 println!("cosa => {}", ele_letters.to_string());
//                 vector.push(ele_letters);
//             } else {
//                 break;
//             }
//         }
//         if !vector.is_empty() {
//             anagrams.insert(*ele_word);
//         }
//     }
//
//     println!("set => {:?}",  anagrams);
//     anagrams
// }

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalize_word = |s: &str| -> Vec<char> {
        let mut chars: Vec<char> = s.to_lowercase().chars().collect();
        chars.sort();
        chars
    };

    let mut anagram_set = HashSet::new();

    let word_normalized = normalize_word(word);

    for anagram in possible_anagrams {
        let anagram_normalized = normalize_word(&anagram);

        println!("word =>{:?}, anagram => {:?}", word_normalized, anagram_normalized);
        if word_normalized == anagram_normalized && word.to_lowercase() != anagram.to_lowercase() {
            anagram_set.insert(*anagram);
        }

    }

    anagram_set

}
