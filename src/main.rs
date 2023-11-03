use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{recognize, verify},
    multi::many1,
    IResult,
};
use std::sync::{Arc, Mutex};

fn main() {
    // Should succeed, but seems to get caught
    let input = "ABC";
    let result_1 = parse(input);
    assert_eq!(result_1, Ok(("", input)));

    // Should fail since `A` exists more than once
    let input = "AABC";
    let result_1 = parse(input);
    assert_eq!(result_1, Ok(("", input)));
}

/// Takes a string input and checks that it only contains one instance of each character contained in a vector
pub fn parse(input: &str) -> IResult<&str, &str> {
    let allowed_once = Arc::new(Mutex::new(vec!["A", "B", "C"]));

    let result: IResult<&str, &str> = recognize(many1(verify(
        alt((tag("A"), tag("B"), tag("C"))),
        |s: &str| {
            println!("Checking character {}", s);
            if let Some(index) = allowed_once.lock().unwrap().iter().position(|x| *x == s) {
                // FIXME: Is it getting stuck here waiting to get a lock?
                println!("Found result at index {}, attempting to remove", index);
                allowed_once.lock().unwrap().swap_remove(index);
                println!("Removed!");
                return true;
            } else {
                println!("Did not find result, failing.");
                return false;
            }
        },
    )))(input);
    return result;
}
