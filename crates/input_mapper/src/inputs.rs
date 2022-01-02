use crate::input_parser::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomInput {
    And(Box<CustomInput>, Box<CustomInput>),
    Or(Box<CustomInput>, Box<CustomInput>),
    Not(Box<CustomInput>),
    Keyboard(KeyboardInputs),
    MouseClick(MouseClickInput),
    MouseMovement(MouseMovementInput),
    MouseWheel(MouseWheelInput),
    MouseDrag(MouseDragInput),
    // GamepadButton(GamepadButton),
    // GamepadStick(GamepadStickDirection),
    // GamepadConnection(GamepadConnection),
}

impl CustomInput {
    /// Constructs an And of two Custom Inputs.
    pub fn and(a: CustomInput, b: CustomInput) -> CustomInput {
        CustomInput::And(Box::new(a), Box::new(b))
    }

    /// Constructs an Or of two Custom Inputs.
    pub fn or(a: CustomInput, b: CustomInput) -> CustomInput {
        CustomInput::Or(Box::new(a), Box::new(b))
    }

    /// Constructs a Not of a Custom Input.
    pub fn not_active(a: CustomInput) -> CustomInput {
        CustomInput::Not(Box::new(a))
    }
}

impl std::fmt::Display for CustomInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CustomInput::And(a, b) => format!("({} + {})", a, b),
                CustomInput::Or(a, b) => format!("({} | {})", a, b),
                CustomInput::Not(a) => format!("!{}", a),
                CustomInput::Keyboard(key) => key.to_string(),
                CustomInput::MouseClick(btn) => btn.to_string(),
                CustomInput::MouseMovement(mvmt) => mvmt.to_string(),
                CustomInput::MouseWheel(scroll) => scroll.to_string(),
                CustomInput::MouseDrag(drag) => drag.to_string(),
            }
        )
    }
}
