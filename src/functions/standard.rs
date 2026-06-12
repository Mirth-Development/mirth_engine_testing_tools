
use crate::TestColors;

/// Used to determine if a test failed or passed based on the passed condition.
/// The supplied reason is a failure message, not for if the test passed.
pub fn check(
    condition_being_tested: bool,
    statement_to_show_whats_being_tested: &str,
    failure_statement: &str
) {
    if condition_being_tested {
        pass(statement_to_show_whats_being_tested);
    }
    else {
        fail(statement_to_show_whats_being_tested, failure_statement);
    }
}

/// Used for the [`check()`] function to print out a test message for the passed condition.
pub fn pass(statement_to_show_whats_being_tested: &str) {
    println!("{}[PASS]{} {}", TestColors::PASS, TestColors::DEFAULT, statement_to_show_whats_being_tested);
}

/// Used for the [`check()`] function to print out a failed message for the passed condition.
pub fn fail(statement_to_show_whats_being_tested: &str, failure_statement: &str) {
    println!("{}[FAIL]{} {} — {}", TestColors::FAIL, TestColors::DEFAULT, statement_to_show_whats_being_tested, failure_statement);
}
