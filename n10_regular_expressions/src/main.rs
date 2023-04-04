use std::str::Chars;

#[derive(Debug)]
enum PatternChar {
    Letter(char),
    AnyLetter,
}

#[derive(Debug)]
enum PatternToken {
    SingleChar(PatternChar),
    ZeroOrMoreChar(PatternChar),
}

struct Solution;

impl Solution {
    /// Split pattern string to pattern tokens
    ///
    /// We have constraints:
    /// 1. p contains only lowercase English letters, '.', and '*'.
    /// 2. It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
    fn split_pattern_to_tokens(p: String) -> Vec<PatternToken> {
        let mut tokens: Vec<PatternToken> = Vec::with_capacity(p.len());
        let mut prev_char_iterator = p.chars();
        let mut curr_char_iterator = p.chars().skip(1);

        while let Some(curr) = curr_char_iterator.next() {
            let prev = prev_char_iterator
                .next()
                .expect("Previous runs out faster than current");
            match (prev, curr) {
                ('.', '*') => tokens.push(PatternToken::ZeroOrMoreChar(PatternChar::AnyLetter)),
                (l, '*') => tokens.push(PatternToken::ZeroOrMoreChar(PatternChar::Letter(l))),
                ('*', _) => (),
                ('.', _) => tokens.push(PatternToken::SingleChar(PatternChar::AnyLetter)),
                (l, _) => tokens.push(PatternToken::SingleChar(PatternChar::Letter(l))),
            }
        }
        match prev_char_iterator.next() {
            Some('*') => (),
            Some('.') => tokens.push(PatternToken::SingleChar(PatternChar::AnyLetter)),
            Some(l) => tokens.push(PatternToken::SingleChar(PatternChar::Letter(l))),
            None => (),
        }
        tokens
    }

    /// Recursive check for pattern matching token by token
    fn recursive_pattern_check(
        mut str_iter: Chars,
        tokens: &[PatternToken],
        chars_remain: usize,
    ) -> bool {
        match (chars_remain, tokens.len()) {
            (0, 0) => return true,
            (_, 0) => return false,
            (0, _) => {
                let mut all_may_be_zero = true;
                for token in tokens {
                    if let PatternToken::SingleChar(_) = token {
                        all_may_be_zero = false;
                        break;
                    }
                }
                return  all_may_be_zero;
            }
            (_, _) => (),
        }
        let curr_token = &tokens[0];
        match curr_token {
            PatternToken::SingleChar(ch) => {
                let str_char = str_iter.next().expect("Took from empty string");
                match ch {
                    PatternChar::Letter(l) => {
                        if str_char == *l {
                            return Self::recursive_pattern_check(
                                str_iter.clone(),
                                &tokens[1..],
                                chars_remain - 1,
                            );
                        } else {
                            return false;
                        }
                    }
                    PatternChar::AnyLetter => {
                        return Self::recursive_pattern_check(
                            str_iter.clone(),
                            &tokens[1..],
                            chars_remain - 1,
                        )
                    }
                }
            }
            PatternToken::ZeroOrMoreChar(ch) => {
                let mut flag: bool = false;
                let mut str_char: char;
                for i in 0..chars_remain + 1 {
                    if i > 0 {
                        str_char = str_iter.next().expect("Took from empty string");
                        match ch {
                            PatternChar::Letter(l) => {
                                if str_char == *l {
                                    flag = Self::recursive_pattern_check(
                                        str_iter.clone(),
                                        &tokens[1..],
                                        chars_remain - i,
                                    );
                                } else {
                                    break;
                                }
                            }
                            PatternChar::AnyLetter => {
                                flag = Self::recursive_pattern_check(
                                    str_iter.clone(),
                                    &tokens[1..],
                                    chars_remain - i,
                                );
                            }
                        }
                    } else {
                        flag = Self::recursive_pattern_check(
                            str_iter.clone(),
                            &tokens[1..],
                            chars_remain - i,
                        );
                    }
                    if flag {
                        return flag;
                    }
                }

                return flag;
            }
        }
    }

    pub fn is_match(s: String, p: String) -> bool {
        // expected signature from leetcode
        if p.len() == 0 {
            return s.len() == 0;
        }
        let tokens = Self::split_pattern_to_tokens(p);
        Self::recursive_pattern_check(s.chars(), &tokens[..], s.len())
    }
}

fn main() {
    let example_1_s = String::from("a");
    let example_1_p = String::from("ab*");
    let result = Solution::is_match(example_1_s, example_1_p);
    println!("{result}");
}
