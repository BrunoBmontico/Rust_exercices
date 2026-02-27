pub fn palindrome() {
    let word:&str = "word";
    let rev_word:Rev<Chars> = word.chars().rev();

    if  rev_word.chars().rev() == word {
        println!("palindrome");
    } else {
        println!("not palindrome");
    }
}