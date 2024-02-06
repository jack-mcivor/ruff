use itertools::Itertools;

use crate::fix::edits::pad;
use ruff_diagnostics::{AlwaysFixableViolation, Diagnostic, Edit, Fix};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{self as ast, Arguments, Expr};
use ruff_text_size::{Ranged, TextRange};

use crate::checkers::ast::Checker;
use crate::fix::snippet::SourceCodeSnippet;

use crate::rules::flynt::helpers;

/// ## What it does
/// Checks for usages of the '+' operator with strings that can be replaced with f-strings.
///
/// ## Why is this bad?
/// f-strings are more readable and generally preferred over `str.join` calls.
///
/// ## Example
/// ```python
/// "foo " + "bar"
/// ```
///
/// Use instead:
/// ```python
/// f"{foo} {bar}"
/// ```
///
/// ## References
/// - [Python documentation: f-strings](https://docs.python.org/3/reference/lexical_analysis.html#f-strings)

#[violation]
pub struct StringConcatenationToFString {
    expression: SourceCodeSnippet,
}

/// FLY001
pub(crate) fn string_concatenation_to_fstring(
    expression: &expr,
) -> Option<StringConcatenationToFString> {
    if let ast::BinOpKind::Add = expression.op() {
        let left = expression.left();
        let right = expression.right();
        if let Some(left) = helpers::string_literal(left) {
            if let Some(right) = helpers::string_literal(right) {
                let expression = SourceCodeSnippet::new(expression);
                return Some(StringConcatenationToFString { expression });
            }
        }
    }
    None
}
