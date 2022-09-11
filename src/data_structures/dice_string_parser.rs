use core::num;

use super::factor::{Factor, Value};
use regex::Regex;

// pub fn from_string(input: &str) -> Box<Factor> {
//     /*
//     Example input: max(1w10, 1w3+3w2)+3

//     1. remove whitespace
//     2. convert into string of symbols

//     */
// }

#[derive(Debug, PartialEq, Eq)]
enum InputSymbol {
    Constant(Value),
    FairDie { min: Value, max: Value },
    Add,
    Multiply,
    Comma,
    Closing,
    Opening,
    MaxOpening,
    MinOpening,
}

impl InputSymbol {
    fn is_atomic(&self) -> bool {
        return match &self {
            Self::FairDie { min, max } => true,
            Self::Constant(_) => true,
            _ => false,
        };
    }
}

// either a factor or an inputSymbol
enum ChainElement<'a> {
    Factor(Factor, Option<&'a ChainElement<'a>>),
    Input(InputSymbol),
}

fn input_symbols_to_factor(symbols: Vec<InputSymbol>) -> Box<Factor> {
    // let mut chain: Vec<ChainElement> = symbols
    //     .iter()
    //     .map(|e| match e {
    //         InputSymbol::FairDie { min, max } => ChainElement::Factor(Factor::FairDie {
    //             min: *min,
    //             max: *max,
    //         }),
    //         InputSymbol::Constant(i) => ChainElement::Factor(Factor::Constant(*i)),
    //         i => ChainElement::Input(*i),
    //     })
    //     .collect();

    // for i in 0..chain.len() {}

    todo!()

    // let len = symbols.len();

    // return match len {
    //     0 => panic!("length of input symbol vector is zero!"),
    //     1 => {
    //         let s = &symbols[0];
    //         match s {
    //             InputSymbol::FairDie { min, max } => Box::new(Factor::FairDie {
    //                 min: *min,
    //                 max: *max,
    //             }),
    //             InputSymbol::Constant(i) => Box::new(Factor::Constant(*i)),
    //             _ => panic!("Cannot generate factor from single input symbol {:?}", s),
    //         }
    //     }
    //     n => {
    //         let levels: Vec<u32> = vec![];

    //         todo!()
    //     }
    // };
}

fn string_to_input_symbols(input: &str) -> Vec<InputSymbol> {
    let mut input = input.to_owned();
    string_utils::clean_string(&mut input);
    let mut symbols: Vec<InputSymbol> = vec![];

    let mut char_iterator = input.chars();
    let mut last_taken_not_processed: Option<char> = None;
    'outer: loop {
        let c = match last_taken_not_processed {
            Some(a) => {
                last_taken_not_processed = None;
                a
            }
            None => match char_iterator.next() {
                Some(e) => e,
                None => break 'outer,
            },
        };

        let flush = || {};
        match c {
            'M' => {
                symbols.push(InputSymbol::MaxOpening);
            }
            'm' => symbols.push(InputSymbol::MinOpening),
            '(' => symbols.push(InputSymbol::Opening),
            ')' => symbols.push(InputSymbol::Closing),
            ',' => symbols.push(InputSymbol::Comma),
            '*' => symbols.push(InputSymbol::Multiply),
            '+' => symbols.push(InputSymbol::Add),
            'd' => {
                let mut num_char_vec: Vec<char> = vec![];
                'inner: loop {
                    let c2 = match char_iterator.next() {
                        Some(e) => e,
                        None => break 'inner,
                    };
                    if c2.is_numeric() {
                        num_char_vec.push(c2)
                    } else {
                        last_taken_not_processed = Some(c2);
                        break;
                    }
                }
                let max: String = num_char_vec.into_iter().collect();
                let max: i64 = max.parse().unwrap();
                symbols.push(InputSymbol::FairDie { min: 1, max });
            }
            '-' => {
                symbols.push(InputSymbol::Add);
                symbols.push(InputSymbol::Constant(-1));
                symbols.push(InputSymbol::Multiply);
            }
            n => {
                let mut num_char_vec: Vec<char> = vec![n];
                'inner: loop {
                    let c2 = match char_iterator.next() {
                        Some(e) => e,
                        None => break 'inner,
                    };
                    if c2.is_numeric() {
                        num_char_vec.push(c2)
                    } else {
                        last_taken_not_processed = Some(c2);
                        break;
                    }
                }
                let n: String = num_char_vec.into_iter().collect();
                println!("n is {n}");
                let n: i64 = n.parse().unwrap();
                symbols.push(InputSymbol::Constant(n));
            }
        }
    }

    symbols
}

mod string_utils {
    use regex::Regex;
    const PERMITTED_CHARACTERS: &str = "minax(,)dw0123456789+-*";
    // pub fn remove_from_string(input: &str, remove: &str) -> String {
    //     let re = Regex::new(remove).unwrap();
    //     return re.replace_all(input, "").to_string();
    // }

    pub fn clean_string(s: &mut String) {
        *s = s.to_lowercase();
        s.retain(|c| PERMITTED_CHARACTERS.chars().into_iter().any(|c2| c == c2));
        *s = s.replace("max(", "M");
        *s = s.replace("min(", "m");
        *s = s.replace("w", "d");

        let re_dice_with_factor = Regex::new(r"(\d)d").unwrap();
        *s = re_dice_with_factor.replace(s, "$1*d").to_string();
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn removing_whitespace() {
        let mut input = r#" max(3w6)        "#.to_owned();
        string_utils::clean_string(&mut input);
        assert_eq!("M3*d6)", input);
    }
    #[test]
    fn string_to_input_symbols_1() {
        let real: Vec<InputSymbol> = string_to_input_symbols("max(13,2)");
        let expected: Vec<InputSymbol> = vec![
            InputSymbol::MaxOpening,
            InputSymbol::Constant(13),
            InputSymbol::Comma,
            InputSymbol::Constant(2),
            InputSymbol::Closing,
        ];
        assert_eq!(real, expected);
    }
    #[test]
    fn string_to_input_symbols_2() {
        let real: Vec<InputSymbol> = string_to_input_symbols("4 w32 - 3");
        let expected: Vec<InputSymbol> = vec![
            InputSymbol::Constant(4),
            InputSymbol::Multiply,
            InputSymbol::FairDie { min: 1, max: 32 },
            InputSymbol::Add,
            InputSymbol::Constant(-1),
            InputSymbol::Multiply,
            InputSymbol::Constant(3),
        ];
        assert_eq!(real, expected);
    }
}
