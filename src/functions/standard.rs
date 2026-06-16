
use crate::TestColors;

/// Used to determine if a test failed or passed based on the passed condition.
/// The supplied reason is a failure message, not for if the test passed.
pub fn check_condition(
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


/// Checks if a value falls within the provided minimum and maximum range (inclusive).
///
/// Accepts any type that implements [`PartialOrd`] and [`std::fmt::Display`], meaning
/// all numeric primitives, [`char`], [`String`], and [`&str`] are valid inputs.
///
/// # Panics
/// Panics if the value is outside the provided range.
///
/// # Example
/// ```
/// check_value(5, 1, 10);    // Passes
/// check_value(15, 1, 10);   // Panics
/// ```
pub fn check_if_value_is_within_range<T: PartialOrd + std::fmt::Display>(value: T, minimum: T, maximum: T) {
    assert!(
        value >= minimum && value <= maximum,
        "{}[FAIL]{} Value must be between {} and {} (inclusive). Got {}.",
        TestColors::FAIL, TestColors::DEFAULT, minimum, maximum, value
    );
}


/// Checks if a value is found within a slice of acceptable values.
///
/// Accepts any type that implements [`PartialEq`], [`std::fmt::Display`], and [`std::fmt::Debug`],
/// meaning all numeric primitives, [`char`], [`String`], and [`&str`] are valid inputs.
///
/// # Panics
/// Panics if the value is not found within the provided slice of acceptable values.
///
/// # Example
/// ```
/// check_if_value_is_acceptable(5, &[1, 3, 5, 7]);   // Passes
/// check_if_value_is_acceptable(4, &[1, 3, 5, 7]);   // Panics
///
/// check_if_value_is_acceptable("up", &["up", "down"]);   // Passes
/// check_if_value_is_acceptable("left", &["up", "down"]); // Panics
/// ```
pub fn check_if_value_is_acceptable<T: PartialEq + std::fmt::Debug + std::fmt::Display>(value: T, acceptable_values: &[T]) {
    assert!(
        acceptable_values.contains(&value),
        "{}[FAIL]{} \"{}\" is not a valid value. Acceptable values are: {:?}.",
        TestColors::FAIL, TestColors::DEFAULT, value, acceptable_values
    );
}
