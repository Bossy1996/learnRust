//! # CratesIoAndStuff
//! 
//! `cratesioandstuff` is a collection of utilities to make performing certain
//! calculations more convinient.


///Adds one to the number given
/// 
/// # EXAMPLES
/// ```
/// let arg = 5;
/// let answer = cratesioandstuff::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}