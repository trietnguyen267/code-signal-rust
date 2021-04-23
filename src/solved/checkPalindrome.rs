// fn main() {
//     assert_eq!(checkPalindrome(String::from("aabaa")), true);
//     assert_eq!(checkPalindrome(String::from("a")), true);
//     assert_eq!(checkPalindrome(String::from("abab")), false);
// }

#[allow(non_snake_case)]
fn checkPalindrome(inputString: String) -> bool {
    if inputString.chars().rev().collect::<String>() == inputString {
        return true;
    }
    false
}
