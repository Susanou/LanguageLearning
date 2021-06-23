use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut ret = HashSet::new();

    let mut tmp:Vec<char> = word.to_lowercase().chars().collect(); //temporary variable while sorting
    tmp.sort_unstable();
    let origin:String = tmp.into_iter().collect();

    for w in possible_anagrams{
        let mut tmp:Vec<char> = w.to_lowercase().chars().collect(); //temporary variable while sorting
        tmp.sort_unstable();

        if tmp.into_iter().collect::<String>() == origin && w.to_lowercase() != word.to_lowercase(){
            ret.insert(*w);
        }
    }

    ret
}
