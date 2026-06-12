
/// Used to label terminal output for tests into different colors.  Applied colors will tack onto
/// text until a different color is set.
///
/// ## EXAMPLE
/// ```ignore
/// println!("{}I am a green message!{} Here are some defaulted colored words!", TestColors::PASS, TestColors::RESET);
/// ```
/// The words "I am a message!" will be whatever color is associated with PASS, and the string "Here
/// are some words!" will be whatever color is associated with DEFAULT.
pub struct TestColors;
impl TestColors {
    pub const DEFAULT:  &'static str = "\x1b[0m";  // \x1b[0m  == WHATEVER THE DEFAULT COLOR IS FOR THE TERMINAL
    pub const FAIL:     &'static str = "\x1b[31m"; // \x1b[31m == RED TERMINAL TEXT
    pub const PASS:     &'static str = "\x1b[32m"; // \x1b[32m == GREEN TERMINAL TEXT
    pub const INFO:     &'static str = "\x1b[33m"; // \x1b[33m == YELLOW TERMINAL TEXT
}
