// Write a function that takes two strings and returns whether they are anagrams of each other (i.e., they contain the same characters, but in a different order).

pub fn anagram(str1: &str, str2: &str) -> bool {
    let mut temp1 = str1.as_bytes().to_owned();
    let mut temp2 = str2.as_bytes().to_owned();
    temp1.sort();
    temp2.sort();
    if temp1 == temp2 {
        true
    } else {
        false
    }
}
