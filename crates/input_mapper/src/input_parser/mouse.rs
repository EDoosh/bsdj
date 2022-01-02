use crate::inputs::CustomInput;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use strum_macros::EnumString;

/// For all variants, their name is what will be returned with
/// `to_string`, and the names will be correctly translated when
/// passed to `from_str`.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, strum_macros::Display)]
#[strum(ascii_case_insensitive)]
pub enum MouseClickInput {
    /// Aliases: MouseLeft, MouseClick, Click
    #[strum(
        serialize = "LeftClick",
        serialize = "MouseLeft",
        serialize = "MouseClick",
        serialize = "Click"
    )]
    LeftClick,
    /// Aliases: MouseMiddle
    #[strum(serialize = "MiddleClick", serialize = "MouseMiddle")]
    MiddleClick,
    /// Aliases: MouseRight
    #[strum(serialize = "RightClick", serialize = "MouseRight")]
    RightClick,
    /// Aliases: MouseDouble
    #[strum(serialize = "DoubleClick", serialize = "MouseDouble")]
    DoubleClick,
}

impl MouseClickInput {
    /// Converts a MouseButton to a MouseClickInput.
    /// DoubleClick must be manually implemented, as it's a meta-input.
    pub fn from_mouse_input(btn: MouseButton) -> MouseClickInput {
        match btn {
            MouseButton::Left => MouseClickInput::LeftClick,
            MouseButton::Middle => MouseClickInput::MiddleClick,
            MouseButton::Right => MouseClickInput::RightClick,
            _ => todo!("MouseButton not yet implemented!"),
        }
    }

    /// Converts to a CustomInput
    pub fn to_custom_input(&self) -> CustomInput {
        CustomInput::MouseClick(*self)
    }
}

/// An input for when the mouse moves a certain direction.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, strum_macros::Display)]
#[strum(ascii_case_insensitive)]
pub enum MouseMovementInput {
    /// Aliases: MouseMoveUp, MouseMovementUp
    #[strum(
        serialize = "MouseUp",
        serialize = "MouseMoveUp",
        serialize = "MouseMovementUp"
    )]
    MouseUp,
    /// Aliases: MouseMoveDown, MouseMovementDown
    #[strum(
        serialize = "MouseDown",
        serialize = "MouseMoveDown",
        serialize = "MouseMovementDown"
    )]
    MouseDown,
    /// Aliases: MouseMoveLeft, MouseMovementLeft
    #[strum(
        serialize = "MouseLeft",
        serialize = "MouseMoveLeft",
        serialize = "MouseMovementLeft"
    )]
    MouseLeft,
    /// Aliases: MouseMoveRight, MouseMovementRight
    #[strum(
        serialize = "MouseRight",
        serialize = "MouseMoveRight",
        serialize = "MouseMovementRight"
    )]
    MouseRight,
    /// Aliases: MouseMoveVertical, MouseMovementVertical
    #[strum(
        serialize = "MouseVertical",
        serialize = "MouseMoveVertical",
        serialize = "MouseMovementVertical"
    )]
    MouseVertical,
    /// Aliases: MouseMoveHorizontal, MouseMovementHorizontal
    #[strum(
        serialize = "MouseHorizontal",
        serialize = "MouseMoveHorizontal",
        serialize = "MouseMovementHorizontal"
    )]
    MouseHorizontal,
}

impl MouseMovementInput {
    /// Converts a mouse movement delta into a MouseMovementInput.
    /// The first returned value is the X movement, while the second is the Y movement.
    pub fn from_movement_delta(
        delta: Vec2,
    ) -> (Option<MouseMovementInput>, Option<MouseMovementInput>) {
        let mut mouse_movement = (None, None);

        if delta.x < 0. {
            mouse_movement.0 = Some(MouseMovementInput::MouseLeft)
        } else if delta.x > 0. {
            mouse_movement.0 = Some(MouseMovementInput::MouseRight)
        }

        if delta.y < 0. {
            mouse_movement.1 = Some(MouseMovementInput::MouseUp)
        } else if delta.y > 0. {
            mouse_movement.1 = Some(MouseMovementInput::MouseDown)
        }

        mouse_movement
    }

    /// Converts to a CustomInput
    pub fn to_custom_input(&self) -> CustomInput {
        match self {
            MouseMovementInput::MouseVertical => CustomInput::or(
                CustomInput::MouseMovement(MouseMovementInput::MouseUp),
                CustomInput::MouseMovement(MouseMovementInput::MouseDown),
            ),
            MouseMovementInput::MouseHorizontal => CustomInput::or(
                CustomInput::MouseMovement(MouseMovementInput::MouseLeft),
                CustomInput::MouseMovement(MouseMovementInput::MouseRight),
            ),
            _ => CustomInput::MouseMovement(*self),
        }
    }
}

/// An input for when the mouse wheel is scrolled.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, strum_macros::Display)]
#[strum(ascii_case_insensitive)]
pub enum MouseWheelInput {
    /// Aliases: MouseWheelUp, ScrollUp, MouseScrollUp
    #[strum(
        serialize = "WheelUp",
        serialize = "MouseWheelUp",
        serialize = "ScrollUp",
        serialize = "MouseScrollUp"
    )]
    WheelUp,
    /// Aliases: MouseWheelDown, ScrollDown, MouseScrollDown
    #[strum(
        serialize = "WheelDown",
        serialize = "MouseWheelDown",
        serialize = "ScrollDown",
        serialize = "MouseScrollDown"
    )]
    WheelDown,
    /// Aliases: MouseWheelLeft, ScrollLeft, MouseScrollLeft
    #[strum(
        serialize = "WheelLeft",
        serialize = "MouseWheelLeft",
        serialize = "ScrollLeft",
        serialize = "MouseScrollLeft"
    )]
    WheelLeft,
    /// Aliases: MouseWheelRight, ScrollRight, MouseScrollRight
    #[strum(
        serialize = "WheelRight",
        serialize = "MouseWheelRight",
        serialize = "ScrollRight",
        serialize = "MouseScrollRight"
    )]
    WheelRight,
    /// Aliases: MouseWheelVertical, ScrollVertical, MouseScrollVertical
    #[strum(
        serialize = "WheelVertical",
        serialize = "MouseWheelVertical",
        serialize = "ScrollVertical",
        serialize = "MouseScrollVertical"
    )]
    WheelVertical,
    /// Aliases: MouseWheelHorizontal, ScrollHorizontal, MouseScrollHorizontal
    #[strum(
        serialize = "WheelHorizontal",
        serialize = "MouseWheelHorizontal",
        serialize = "ScrollHorizontal",
        serialize = "MouseScrollHorizontal"
    )]
    WheelHorizontal,
}

impl MouseWheelInput {
    /// Converts a mouse scroll delta into a MouseWheelInput.
    /// The first returned value is the X movement, while the second is the Y movement.
    pub fn from_scroll_delta(delta: Vec2) -> (Option<MouseWheelInput>, Option<MouseWheelInput>) {
        let mut mouse_wheel = (None, None);

        if delta.x < 0. {
            mouse_wheel.0 = Some(MouseWheelInput::WheelLeft)
        } else if delta.x > 0. {
            mouse_wheel.0 = Some(MouseWheelInput::WheelRight)
        }

        if delta.y < 0. {
            mouse_wheel.1 = Some(MouseWheelInput::WheelUp)
        } else if delta.y > 0. {
            mouse_wheel.1 = Some(MouseWheelInput::WheelDown)
        }

        mouse_wheel
    }

    /// Converts to a CustomInput
    pub fn to_custom_input(&self) -> CustomInput {
        match self {
            MouseWheelInput::WheelVertical => CustomInput::or(
                CustomInput::MouseWheel(MouseWheelInput::WheelUp),
                CustomInput::MouseWheel(MouseWheelInput::WheelDown),
            ),
            MouseWheelInput::WheelHorizontal => CustomInput::or(
                CustomInput::MouseWheel(MouseWheelInput::WheelLeft),
                CustomInput::MouseWheel(MouseWheelInput::WheelRight),
            ),
            _ => CustomInput::MouseWheel(*self),
        }
    }
}

/// An input for when the left mouse-button is held in and the mouse is dragged.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, strum_macros::Display)]
#[strum(ascii_case_insensitive)]
pub enum MouseDragInput {
    MouseDrag,
}

impl MouseDragInput {
    /// Converts to a CustomInput
    pub fn to_custom_input(&self) -> CustomInput {
        CustomInput::MouseDrag(*self)
    }
}
