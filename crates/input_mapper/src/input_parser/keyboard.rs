use crate::inputs::CustomInput;
use bevy::input::keyboard::KeyCode;
use strum_macros::EnumString;

/// For all variants, their name is what will be returned with
/// `to_string`, and the names will be correctly translated when
/// passed to `from_str`.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, strum_macros::Display)]
#[strum(ascii_case_insensitive)]
pub enum KeyboardInputs {
    /// Aliases: Control
    #[strum(serialize = "ctrl", serialize = "control")]
    Ctrl,
    /// Aliases: LControl
    #[strum(serialize = "lctrl", serialize = "lcontrol")]
    LCtrl,
    /// Aliases: RControl
    #[strum(serialize = "rctrl", serialize = "rcontrol")]
    RCtrl,

    /// Aliases: Shft
    #[strum(serialize = "shift", serialize = "shft")]
    Shift,
    /// Aliases: LShft
    #[strum(serialize = "lshift", serialize = "lshft")]
    LShift,
    /// Aliases: RShft
    #[strum(serialize = "rshift", serialize = "rshft")]
    RShift,

    /// Aliases: Opt, Option
    #[strum(serialize = "alt", serialize = "opt", serialize = "option")]
    Alt,
    /// Aliases: LOpt, LOption
    #[strum(serialize = "lalt", serialize = "lopt", serialize = "loption")]
    LAlt,
    /// Aliases: ROpt, ROption
    #[strum(serialize = "ralt", serialize = "ropt", serialize = "roption")]
    RAlt,

    /// Aliases: Windows, Cmd, Command
    #[strum(
        serialize = "win",
        serialize = "windows",
        serialize = "cmd",
        serialize = "command"
    )]
    Win,
    /// Aliases: LWindows, LCmd, LCommand
    #[strum(
        serialize = "lwin",
        serialize = "lwindows",
        serialize = "lcmd",
        serialize = "lcommand"
    )]
    LWin,
    /// Aliases: RWindows, RCmd, RCommand
    #[strum(
        serialize = "rwin",
        serialize = "rwindows",
        serialize = "rcmd",
        serialize = "rcommand"
    )]
    RWin,

    Tab,
    /// Aliases: CapsLock
    #[strum(serialize = "caps", serialize = "capslock")]
    Caps,

    /// Aliases: 0
    #[strum(serialize = "key0", serialize = "0")]
    Key0,
    /// Aliases: 1
    #[strum(serialize = "key1", serialize = "1")]
    Key1,
    /// Aliases: 2
    #[strum(serialize = "key2", serialize = "2")]
    Key2,
    /// Aliases: 3
    #[strum(serialize = "key3", serialize = "3")]
    Key3,
    /// Aliases: 4
    #[strum(serialize = "key4", serialize = "4")]
    Key4,
    /// Aliases: 5
    #[strum(serialize = "key5", serialize = "5")]
    Key5,
    /// Aliases: 6
    #[strum(serialize = "key6", serialize = "6")]
    Key6,
    /// Aliases: 7
    #[strum(serialize = "key7", serialize = "7")]
    Key7,
    /// Aliases: 8
    #[strum(serialize = "key8", serialize = "8")]
    Key8,
    /// Aliases: 9
    #[strum(serialize = "key9", serialize = "9")]
    Key9,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// Aliases: Escape
    #[strum(serialize = "esc", serialize = "escape")]
    Esc,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    /// Aliases: PrtSc, SysRq, Screenshot
    #[strum(
        serialize = "prtscr",
        serialize = "prtsc",
        serialize = "sysreq",
        serialize = "screenshot"
    )]
    PrtScr,
    /// Aliases: Scroll
    #[strum(serialize = "scroll", serialize = "scrolllock")]
    ScrollLock,
    NumLock,

    /// Aliases: Break
    #[strum(serialize = "pause", serialize = "break")]
    Pause,
    /// Aliases: Ins
    #[strum(serialize = "insert", serialize = "ins")]
    Insert,
    Home,
    /// Aliases: Del
    #[strum(serialize = "delete", serialize = "del")]
    Delete,
    End,
    /// Aliases: PgDown, PageD
    #[strum(serialize = "pagedown", serialize = "pgdown", serialize = "paged")]
    PageDown,
    /// Aliases: PgUp, PageU
    #[strum(serialize = "pageup", serialize = "pgup", serialize = "pageu")]
    PageUp,

    /// Aliases: UpArrow
    #[strum(serialize = "uparrow", serialize = "up")]
    Up,
    /// Aliases: LeftArrow
    #[strum(serialize = "leftarrow", serialize = "left")]
    Left,
    /// Aliases: DownArrow
    #[strum(serialize = "downarrow", serialize = "down")]
    Down,
    /// Aliases: RightArrow
    #[strum(serialize = "rightarrow", serialize = "right")]
    Right,

    /// Aliases: Back
    #[strum(serialize = "backspace", serialize = "back")]
    Backspace,
    /// Aliases: Enter
    #[strum(serialize = "return", serialize = "enter")]
    Return,
    /// Aliases: Spacebar
    #[strum(serialize = "space", serialize = "spacebar")]
    Space,
    Compose,

    /// Aliases: Num0
    #[strum(serialize = "numpad0", serialize = "num0")]
    Numpad0,
    /// Aliases: Num1
    #[strum(serialize = "numpad1", serialize = "num1")]
    Numpad1,
    /// Aliases: Num2
    #[strum(serialize = "numpad2", serialize = "num2")]
    Numpad2,
    /// Aliases: Num3
    #[strum(serialize = "numpad3", serialize = "num3")]
    Numpad3,
    /// Aliases: Num4
    #[strum(serialize = "numpad4", serialize = "num4")]
    Numpad4,
    /// Aliases: Num5
    #[strum(serialize = "numpad5", serialize = "num5")]
    Numpad5,
    /// Aliases: Num6
    #[strum(serialize = "numpad6", serialize = "num6")]
    Numpad6,
    /// Aliases: Num7
    #[strum(serialize = "numpad7", serialize = "num7")]
    Numpad7,
    /// Aliases: Num8
    #[strum(serialize = "numpad8", serialize = "num8")]
    Numpad8,
    /// Aliases: Num9
    #[strum(serialize = "numpad9", serialize = "num9")]
    Numpad9,

    /// Aliases: Backtik, Backtick, `
    #[strum(
        serialize = "grave",
        serialize = "backtik",
        serialize = "backtick",
        serialize = "`"
    )]
    Grave,
    /// Aliases: -
    #[strum(serialize = "minus", serialize = "-")]
    Minus,
    /// Aliases: =
    #[strum(serialize = "equals", serialize = "eq", serialize = "=")]
    Equals,

    /// Aliases: \
    #[strum(serialize = "backslash", serialize = "\\")]
    Backslash,

    /// Aliases: ;
    #[strum(serialize = "semicolon", serialize = ";")]
    Semicolon,
    /// Aliases: '
    #[strum(serialize = "apostrophe", serialize = "'")]
    Apostrophe,

    /// Aliases: ,
    #[strum(serialize = "comma", serialize = ",")]
    Comma,
    /// Aliases: fullstop, .
    #[strum(serialize = "period", serialize = "fullstop", serialize = ".")]
    Period,
    /// Aliases: forwardslash, /
    #[strum(serialize = "slash", serialize = "forwardslash", serialize = "/")]
    Slash,
}

impl KeyboardInputs {
    /// Converts a KeyCode into a KeyboardInput.
    pub fn from_keycode(keycode: KeyCode) -> KeyboardInputs {
        match keycode {
            KeyCode::LControl => KeyboardInputs::LCtrl,
            KeyCode::RControl => KeyboardInputs::RCtrl,
            KeyCode::LShift => KeyboardInputs::LShift,
            KeyCode::RShift => KeyboardInputs::RShift,
            KeyCode::LAlt => KeyboardInputs::LAlt,
            KeyCode::RAlt => KeyboardInputs::RAlt,
            KeyCode::LWin => KeyboardInputs::LWin,
            KeyCode::RWin => KeyboardInputs::RWin,

            KeyCode::Tab => KeyboardInputs::Tab,
            KeyCode::Capital => KeyboardInputs::Caps,

            KeyCode::Key0 => KeyboardInputs::Key0,
            KeyCode::Key1 => KeyboardInputs::Key1,
            KeyCode::Key2 => KeyboardInputs::Key2,
            KeyCode::Key3 => KeyboardInputs::Key3,
            KeyCode::Key4 => KeyboardInputs::Key4,
            KeyCode::Key5 => KeyboardInputs::Key5,
            KeyCode::Key6 => KeyboardInputs::Key6,
            KeyCode::Key7 => KeyboardInputs::Key7,
            KeyCode::Key8 => KeyboardInputs::Key8,
            KeyCode::Key9 => KeyboardInputs::Key9,

            KeyCode::A => KeyboardInputs::A,
            KeyCode::B => KeyboardInputs::B,
            KeyCode::C => KeyboardInputs::C,
            KeyCode::D => KeyboardInputs::D,
            KeyCode::E => KeyboardInputs::E,
            KeyCode::F => KeyboardInputs::F,
            KeyCode::G => KeyboardInputs::G,
            KeyCode::H => KeyboardInputs::H,
            KeyCode::I => KeyboardInputs::I,
            KeyCode::J => KeyboardInputs::J,
            KeyCode::K => KeyboardInputs::K,
            KeyCode::L => KeyboardInputs::L,
            KeyCode::M => KeyboardInputs::M,
            KeyCode::N => KeyboardInputs::N,
            KeyCode::O => KeyboardInputs::O,
            KeyCode::P => KeyboardInputs::P,
            KeyCode::Q => KeyboardInputs::Q,
            KeyCode::R => KeyboardInputs::R,
            KeyCode::S => KeyboardInputs::S,
            KeyCode::T => KeyboardInputs::T,
            KeyCode::U => KeyboardInputs::U,
            KeyCode::V => KeyboardInputs::V,
            KeyCode::W => KeyboardInputs::W,
            KeyCode::X => KeyboardInputs::X,
            KeyCode::Y => KeyboardInputs::Y,
            KeyCode::Z => KeyboardInputs::Z,

            KeyCode::Escape => KeyboardInputs::Esc,
            KeyCode::F1 => KeyboardInputs::F1,
            KeyCode::F2 => KeyboardInputs::F2,
            KeyCode::F3 => KeyboardInputs::F3,
            KeyCode::F4 => KeyboardInputs::F4,
            KeyCode::F5 => KeyboardInputs::F5,
            KeyCode::F6 => KeyboardInputs::F6,
            KeyCode::F7 => KeyboardInputs::F7,
            KeyCode::F8 => KeyboardInputs::F8,
            KeyCode::F9 => KeyboardInputs::F9,
            KeyCode::F10 => KeyboardInputs::F10,
            KeyCode::F11 => KeyboardInputs::F11,
            KeyCode::F12 => KeyboardInputs::F12,
            KeyCode::F13 => KeyboardInputs::F13,
            KeyCode::F14 => KeyboardInputs::F14,
            KeyCode::F15 => KeyboardInputs::F15,
            KeyCode::F16 => KeyboardInputs::F16,
            KeyCode::F17 => KeyboardInputs::F17,
            KeyCode::F18 => KeyboardInputs::F18,
            KeyCode::F19 => KeyboardInputs::F19,
            KeyCode::F20 => KeyboardInputs::F20,
            KeyCode::F21 => KeyboardInputs::F21,
            KeyCode::F22 => KeyboardInputs::F22,
            KeyCode::F23 => KeyboardInputs::F23,
            KeyCode::F24 => KeyboardInputs::F24,

            KeyCode::Snapshot => KeyboardInputs::PrtScr,
            KeyCode::Scroll => KeyboardInputs::ScrollLock,
            KeyCode::Numlock => KeyboardInputs::NumLock,

            KeyCode::Pause => KeyboardInputs::Pause,
            KeyCode::Insert => KeyboardInputs::Insert,
            KeyCode::Home => KeyboardInputs::Home,
            KeyCode::Delete => KeyboardInputs::Delete,
            KeyCode::End => KeyboardInputs::End,
            KeyCode::PageDown => KeyboardInputs::PageDown,
            KeyCode::PageUp => KeyboardInputs::PageUp,

            KeyCode::Up => KeyboardInputs::Up,
            KeyCode::Down => KeyboardInputs::Down,
            KeyCode::Left => KeyboardInputs::Left,
            KeyCode::Right => KeyboardInputs::Right,

            KeyCode::Back => KeyboardInputs::Backspace,
            KeyCode::Return => KeyboardInputs::Return,
            KeyCode::Space => KeyboardInputs::Space,
            KeyCode::Compose => KeyboardInputs::Compose,

            KeyCode::Numpad0 => KeyboardInputs::Numpad0,
            KeyCode::Numpad1 => KeyboardInputs::Numpad1,
            KeyCode::Numpad2 => KeyboardInputs::Numpad2,
            KeyCode::Numpad3 => KeyboardInputs::Numpad3,
            KeyCode::Numpad4 => KeyboardInputs::Numpad4,
            KeyCode::Numpad5 => KeyboardInputs::Numpad5,
            KeyCode::Numpad6 => KeyboardInputs::Numpad6,
            KeyCode::Numpad7 => KeyboardInputs::Numpad7,
            KeyCode::Numpad8 => KeyboardInputs::Numpad8,
            KeyCode::Numpad9 => KeyboardInputs::Numpad9,

            KeyCode::Grave => KeyboardInputs::Grave,
            KeyCode::Minus => KeyboardInputs::Minus,
            KeyCode::Equals => KeyboardInputs::Equals,

            KeyCode::Backslash => KeyboardInputs::Backslash,

            KeyCode::Semicolon => KeyboardInputs::Semicolon,
            KeyCode::Apostrophe => KeyboardInputs::Apostrophe,
            KeyCode::Comma => KeyboardInputs::Comma,
            KeyCode::Period => KeyboardInputs::Period,
            KeyCode::Slash => KeyboardInputs::Slash,

            _ => todo!("Keycode not yet implemented!"),
        }
    }

    /// Converts this into a CustomInput type.
    pub fn to_custom_input(&self) -> CustomInput {
        match self {
            KeyboardInputs::Ctrl => CustomInput::or(
                CustomInput::Keyboard(KeyboardInputs::LCtrl),
                CustomInput::Keyboard(KeyboardInputs::RCtrl),
            ),

            KeyboardInputs::Shift => CustomInput::or(
                CustomInput::Keyboard(KeyboardInputs::LShift),
                CustomInput::Keyboard(KeyboardInputs::RShift),
            ),

            KeyboardInputs::Alt => CustomInput::or(
                CustomInput::Keyboard(KeyboardInputs::LAlt),
                CustomInput::Keyboard(KeyboardInputs::RAlt),
            ),

            KeyboardInputs::Win => CustomInput::or(
                CustomInput::Keyboard(KeyboardInputs::LWin),
                CustomInput::Keyboard(KeyboardInputs::RWin),
            ),

            _ => CustomInput::Keyboard(*self),
        }
    }
}
