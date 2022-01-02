use crate::inputs::CustomInput;
use regex::Regex;
use std::str::FromStr;

pub mod keyboard;
pub mod mouse;

pub use keyboard::*;
pub use mouse::*;

// BEFORE EDITING THESE DOUBLE CHECK THEY DON'T BREAK EXISTING
// KEYWORDS OR THE REGEX
const AND: &str = "+";
const OR: &str = "|";
const NOT: &str = "!";

/// Parses the string into a CustomInput. See the examples below for
/// how it can be used.
///
/// In summary, (a lot of) inputs have a name (and sometimes some aliases)
/// that are used to identify them. To require two inputs to be pressed at
/// the same time, seperate them with a `+`, e.g. `Ctrl + W`. To allow one
/// or the other or both to be pressed at the same time, seperate them with
/// a `|`, e.g. `Ctrl | Shift`. To require a key not to be pressed, prepend
/// a `!`, e.g. `!MouseClick`. The order of operations on this is parenthesis,
/// not, and, then or.
///
/// Read up on the documentation for all the input types to find aliases.
///
/// # Examples
///
/// ```
/// use input_mapper::input_parser::*;
/// use input_mapper::inputs::CustomInput;
///
/// // This is how big a moderately-basic input structure would normally be.
/// let expected = CustomInput::or(
///     CustomInput::and(
///         CustomInput::and(
///             CustomInput::or(
///                 CustomInput::Keyboard(KeyboardInputs::LCtrl),
///                 CustomInput::Keyboard(KeyboardInputs::RCtrl),
///             ),
///             CustomInput::or(
///                 CustomInput::Keyboard(KeyboardInputs::LShift),
///                 CustomInput::Keyboard(KeyboardInputs::RShift),
///             )
///         ),
///         CustomInput::or(
///             CustomInput::Keyboard(KeyboardInputs::W),
///             CustomInput::Keyboard(KeyboardInputs::Key1),
///         )
///     ),
///     CustomInput::and(
///         CustomInput::MouseClick(MouseClickInput::RightClick),
///         CustomInput::not_active(
///             CustomInput::MouseClick(MouseClickInput::LeftClick)
///         )
///     )
/// );
///
/// // It will be wrapped inside an Option.
/// let expected = Some(expected);
///
/// // Priority order is brackets, not, and, then or.
/// let parsed = parse_input("Ctrl + Shft + (w | 1) | MouseRight + !LeftClick").unwrap();
/// assert_eq!(parsed, expected);
///
/// // But brackets can be placed around anything if needed.
/// let parsed = parse_input("((Ctrl + (Shft)) + (w | 1)) | ((MouseRight) + (!LeftClick))").unwrap();
/// assert_eq!(parsed, expected);
///
/// // Spaces are optional and it's all case-insensitive.
/// // Many inputs also have aliases.
/// let parsed = parse_input("((Control+Shift)+(w|Key1))|(MouSeRigHt+!LEFTCLICK)").unwrap();
/// assert_eq!(parsed, expected);
///
/// ```
pub fn parse_input(input: &str) -> Result<Option<CustomInput>, String> {
    let input_regex = Regex::new(&format!(
        r"([(){and}{or}{not}]|[^(){and}{or}{not} \n\r]+)",
        and = AND,
        or = OR,
        not = NOT
    ))
    .unwrap();

    // This entire function is essentially an infix-to-postfix parser.

    // Store the operators in a stack to await being added to the postfixed array
    let mut operators = vec![];
    // The array containing the parsable order of input keywords/tokens
    let mut postfixed = vec![];

    // Find all the keywords/tokens
    for mat in input_regex.find_iter(input) {
        let input_keyword = mat.as_str();

        match input_keyword {
            // Will be matched later by a closing parenthesis
            "(" => operators.push("("),
            ")" => {
                // Remember to error if there is no opening parenthesis
                let mut op_token = operators
                    .pop()
                    .ok_or(format!("Unopened closing parenthesis in input `{}`", input))?;

                // Just keep removing from the tokens and adding to the ordered
                // postfix array until the closing parenthesis is found.
                while op_token != "(" {
                    postfixed.push(op_token);

                    op_token = operators
                        .pop()
                        .ok_or(format!("Unopened closing parenthesis in input `{}`", input))?;
                }
            }
            AND | OR | NOT => {
                // Is a functional operator.
                let current_token_priority = get_token_priority(input_keyword)?;

                // Iterate popping from the operators and adding to the postfix array
                // until an operator of higher precendence is found.
                let mut next_token = operators.pop();
                while next_token.is_some()
                    && get_token_priority(next_token.unwrap())? >= current_token_priority
                {
                    postfixed.push(next_token.unwrap());
                    next_token = operators.pop();
                }

                // If it quit because the token was a higher priority,
                // add it back in to the operators.
                if let Some(token) = next_token {
                    operators.push(token);
                }

                // Finally remember to add the current operator.
                operators.push(input_keyword);
            }
            // Is an input keyword. Just add directly to the postfixed array.
            _ => postfixed.push(input_keyword),
        }
    }

    // Spare tokens may still exist. Remove from operators in reverse order,
    // adding them into the end of the postfixed array.
    for token in operators.iter().rev() {
        // If the added token is an opening parenthesis it means
        // there was no closing.
        if token == &"(" {
            return Err(format!("Unclosed opening parenthesis in input `{}`", input));
        }

        postfixed.push(token)
    }

    // If postfixed is empty, it must mean that nothing was in the input string.
    if postfixed.is_empty() {
        return Ok(None);
    }

    eprintln!("{:?}", postfixed);

    // At this point we now have a postfixed array and can construct
    // a CustomInput from these values.

    let mut calc_stack = vec![];
    for token in postfixed {
        match token {
            AND => {
                // Get the top two values from the calc_stack to
                // AND together into a CustomInput that will be inserted
                // back into the top of the stack.
                let and_b = calc_stack.pop().ok_or(format!(
                    "Unexpected `{}` operator for input `{}`",
                    AND, input
                ))?;
                let and_a = calc_stack.pop().ok_or(format!(
                    "Unexpected `{}` operator for input `{}`",
                    AND, input
                ))?;
                calc_stack.push(CustomInput::and(and_a, and_b))
            }
            OR => {
                // Same as AND but for OR at the end
                let or_b = calc_stack.pop().ok_or(format!(
                    "Unexpected `{}` operator for input `{}`",
                    OR, input
                ))?;
                let or_a = calc_stack.pop().ok_or(format!(
                    "Unexpected `{}` operator for input `{}`",
                    OR, input
                ))?;
                calc_stack.push(CustomInput::or(or_a, or_b))
            }
            NOT => {
                // Get the top value from the calc_stack that
                // will be negated and reinserted at the top of the stack.
                let not = calc_stack.pop().ok_or(format!(
                    "Unexpected `{}` operator for input `{}`",
                    NOT, input
                ))?;
                calc_stack.push(CustomInput::not_active(not))
            }
            _ => calc_stack.push(
                parse_input_keyword(token)
                    .ok_or(format!("Invalid keyword `{}` in input `{}`", token, input))?,
            ),
        }
    }

    // Something has gone awry, because there are either no values or too many.
    if calc_stack.len() != 1 {
        Err(format!(
            "Something went wrong parsing the input `{}`. Is there a mismatched amount of input keywords?",
            input
        ))
    } else {
        Ok(Some(calc_stack[0].clone()))
    }
}

#[allow(clippy::manual_map)]
pub fn parse_input_keyword(keyword: &str) -> Option<CustomInput> {
    // Remove underscores and hyphens from the input string.
    let removable = &['_', '-'][..];
    let keyword = &keyword.replace(removable, "");

    if let Ok(custom_input) = KeyboardInputs::from_str(keyword) {
        Some(custom_input.to_custom_input())
    } else if let Ok(custom_input) = MouseClickInput::from_str(keyword) {
        Some(custom_input.to_custom_input())
    } else if let Ok(custom_input) = MouseMovementInput::from_str(keyword) {
        Some(custom_input.to_custom_input())
    } else if let Ok(custom_input) = MouseWheelInput::from_str(keyword) {
        Some(custom_input.to_custom_input())
    } else if let Ok(custom_input) = MouseDragInput::from_str(keyword) {
        Some(custom_input.to_custom_input())
    } else {
        None
    }
}

/// Returns the token's priority.
/// A higher number means more important.
///
/// # Errors
///
/// The function will return an Err if the token is not valid.
fn get_token_priority(token: &str) -> Result<u8, String> {
    Ok(match token {
        "(" | ")" => 1,
        NOT => 4,
        AND => 3,
        OR => 2,
        _ => return Err(
            format!("Token passed to `get_token_priority` not a valid token. Expected `(`, `)`, `{}`, `{}`, or `{}`, got `{}`",
                NOT,
                AND,
                OR,
                token
            ))
    })
}

#[cfg(test)]
mod tests {
    use crate::input_parser::*;
    use crate::inputs::CustomInput;

    #[test]
    fn check_token_priority() {
        let priority_not = get_token_priority(NOT);
        let priority_and = get_token_priority(AND);
        let priority_or = get_token_priority(OR);
        let priority_lparen = get_token_priority("(");
        let priority_rparen = get_token_priority(")");

        // Assert the parenthesis have the same priority
        assert_eq!(
            priority_lparen, priority_rparen,
            "`(` token and `)` token have differing priorities."
        );

        // Assert the orders are correct
        assert!(
            priority_not > priority_and,
            "`NOT` token does not have greater priority than `AND` token."
        );
        assert!(
            priority_and > priority_or,
            "`AND` token does not have greater priority than `OR` token."
        );
        assert!(
            priority_or > priority_lparen,
            "`OR` token does not have greater priority than the parenthesis tokens."
        );

        // Assert that invalid tokens will error BUT NOT CRASH.
        assert!(get_token_priority("X").is_err());
        assert!(get_token_priority("qwertyuiop").is_err());
        assert!(get_token_priority("").is_err());
    }

    #[test]
    fn basic_parse_input_keyword_checks() {
        // So the problem here is that there are hundreds of valid names for inputs.
        // Just test a few of them to make sure it all works.

        // A few mouse checks
        assert_eq!(
            parse_input_keyword("MouseClick"),
            Some(CustomInput::MouseClick(MouseClickInput::LeftClick))
        );
        assert_eq!(
            parse_input_keyword("DoubleClick"),
            Some(CustomInput::MouseClick(MouseClickInput::DoubleClick))
        );
        assert_eq!(
            parse_input_keyword("MouseMovementDown"),
            Some(CustomInput::MouseMovement(MouseMovementInput::MouseDown))
        );
        assert_eq!(
            parse_input_keyword("WheelVertical"),
            Some(CustomInput::or(
                CustomInput::MouseWheel(MouseWheelInput::WheelUp),
                CustomInput::MouseWheel(MouseWheelInput::WheelDown),
            ))
        );
        assert_eq!(
            parse_input_keyword("MouseDrag"),
            Some(CustomInput::MouseDrag(MouseDragInput::MouseDrag))
        );

        // A couple keyboard checks
        assert_eq!(
            parse_input_keyword("A"),
            Some(CustomInput::Keyboard(KeyboardInputs::A))
        );
        assert_eq!(
            parse_input_keyword("Command"),
            Some(CustomInput::or(
                CustomInput::Keyboard(KeyboardInputs::LWin),
                CustomInput::Keyboard(KeyboardInputs::RWin),
            ))
        );

        // If it's invalid it should return None
        assert!(parse_input_keyword("gobbledy-gook").is_none())
    }

    #[test]
    fn caps_check_parse_input_keyword() {
        // Any capitalisation should work.

        // A few mouse checks
        assert_eq!(
            parse_input_keyword("MouSeClICk"),
            Some(CustomInput::MouseClick(MouseClickInput::LeftClick))
        );
        assert_eq!(
            parse_input_keyword("doubleclick"),
            Some(CustomInput::MouseClick(MouseClickInput::DoubleClick))
        );
        assert_eq!(
            parse_input_keyword("mouseMovementDown"),
            Some(CustomInput::MouseMovement(MouseMovementInput::MouseDown))
        );
        assert_eq!(
            parse_input_keyword("Wheel_Vertical"),
            Some(CustomInput::or(
                CustomInput::MouseWheel(MouseWheelInput::WheelUp),
                CustomInput::MouseWheel(MouseWheelInput::WheelDown),
            ))
        );
        assert_eq!(
            parse_input_keyword("Mouse-Drag"),
            Some(CustomInput::MouseDrag(MouseDragInput::MouseDrag))
        );

        // A couple keyboard checks
        assert_eq!(
            parse_input_keyword("a"),
            Some(CustomInput::Keyboard(KeyboardInputs::A))
        );
        assert_eq!(
            parse_input_keyword("cOmMANd"),
            Some(CustomInput::or(
                CustomInput::Keyboard(KeyboardInputs::LWin),
                CustomInput::Keyboard(KeyboardInputs::RWin),
            ))
        );
    }

    #[test]
    fn alias_check_parse_input_keyword() {
        // Check that all aliases work.
        assert_eq!(
            parse_input_keyword("rwin"),
            Some(CustomInput::Keyboard(KeyboardInputs::RWin))
        );
        assert_eq!(
            parse_input_keyword("rwindows"),
            Some(CustomInput::Keyboard(KeyboardInputs::RWin))
        );
        assert_eq!(
            parse_input_keyword("rcmd"),
            Some(CustomInput::Keyboard(KeyboardInputs::RWin))
        );
        assert_eq!(
            parse_input_keyword("rcommand"),
            Some(CustomInput::Keyboard(KeyboardInputs::RWin))
        );
    }

    #[test]
    fn empty_parse_input() {
        // Parsing an empty string should return a None.
        assert_eq!(parse_input(""), Ok(None))
    }

    #[test]
    fn single_operator_parse_input() {
        // Parsing an operator with nothing else should return an error.
        assert_eq!(
            parse_input(" + "),
            Err("Unexpected `+` operator for input ` + `".to_string())
        )
    }

    #[test]
    fn basic_parse_input_checks() {
        // A single keyword.
        assert_eq!(
            parse_input("Click"),
            Ok(Some(CustomInput::MouseClick(MouseClickInput::LeftClick)))
        );

        // Two keyword with an OR between them.
        assert_eq!(
            parse_input("Click | Enter"),
            Ok(Some(CustomInput::or(
                CustomInput::MouseClick(MouseClickInput::LeftClick),
                CustomInput::Keyboard(KeyboardInputs::Return)
            )))
        );

        // Three keywords with no parenthesis and an or between all.
        // Parse order places imaginary parenthesis around the first two.
        assert_eq!(
            parse_input("Click | Enter | Z"),
            Ok(Some(CustomInput::or(
                CustomInput::or(
                    CustomInput::MouseClick(MouseClickInput::LeftClick),
                    CustomInput::Keyboard(KeyboardInputs::Return)
                ),
                CustomInput::Keyboard(KeyboardInputs::Z)
            )))
        );

        // Multiple keywords with all sorts of operators.
        assert_eq!(
            parse_input("(Click | Enter + (!Ctrl | !Alt | !Win) | ;) + Shift"),
            Ok(Some(CustomInput::and(
                CustomInput::or(
                    CustomInput::or(
                        CustomInput::MouseClick(MouseClickInput::LeftClick),
                        CustomInput::and(
                            CustomInput::Keyboard(KeyboardInputs::Return),
                            CustomInput::or(
                                CustomInput::or(
                                    CustomInput::not_active(CustomInput::or(
                                        CustomInput::Keyboard(KeyboardInputs::LCtrl),
                                        CustomInput::Keyboard(KeyboardInputs::RCtrl)
                                    )),
                                    CustomInput::not_active(CustomInput::or(
                                        CustomInput::Keyboard(KeyboardInputs::LAlt),
                                        CustomInput::Keyboard(KeyboardInputs::RAlt)
                                    ))
                                ),
                                CustomInput::not_active(CustomInput::or(
                                    CustomInput::Keyboard(KeyboardInputs::LWin),
                                    CustomInput::Keyboard(KeyboardInputs::RWin)
                                ))
                            )
                        )
                    ),
                    CustomInput::Keyboard(KeyboardInputs::Semicolon)
                ),
                CustomInput::or(
                    CustomInput::Keyboard(KeyboardInputs::LShift),
                    CustomInput::Keyboard(KeyboardInputs::RShift)
                )
            )))
        );
    }

    #[test]
    fn unbalanced_operators_parse_input() {
        // Unopened closing parenthesis
        assert_eq!(
            parse_input("Click + Shift) | (RightClick + Tab)"),
            Err(
                "Unopened closing parenthesis in input `Click + Shift) | (RightClick + Tab)`"
                    .to_string()
            )
        );
        // Unclosed opening parenthesis
        assert_eq!(
            parse_input("(Click + Shift) | (RightClick + Tab"),
            Err(
                "Unclosed opening parenthesis in input `(Click + Shift) | (RightClick + Tab`"
                    .to_string()
            )
        );

        // Not enough operands for operators
        assert_eq!(
            parse_input("(Click + Shift) | "),
            Err("Unexpected `|` operator for input `(Click + Shift) | `".to_string())
        );
        // Too many operands for operators
        assert_eq!(parse_input("(Click + Left Shift)"), Err("Something went wrong parsing the input `(Click + Left Shift)`. Is there a mismatched amount of input keywords?".to_string()));
    }
}
