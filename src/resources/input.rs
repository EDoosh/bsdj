use crate::utils;
use bevy::input::{
    keyboard::KeyboardInput,
    mouse::{MouseButtonInput, MouseWheel},
};
use bevy::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

/// The number of frames between a left click being released and repressed
/// for it to be considered a double-click. By default, this program runs
/// at 60fps.
pub const DOUBLE_CLICK_FRAME_COUNT: u64 = 30;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputRes::new());
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new().with_system(update_inputs),
        );
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum InputType {
    Key(KeyCode),
    Mouse(MouseButton),
}

impl InputType {
    /// Returns the 4 directional keycodes.
    pub fn directional_keycodes() -> [InputType; 4] {
        [
            InputType::Key(KeyCode::Up),
            InputType::Key(KeyCode::Down),
            InputType::Key(KeyCode::Left),
            InputType::Key(KeyCode::Right),
        ]
    }

    /// Returns an Option where the value is the x and y to change by.
    pub fn directional_change(&self) -> Option<(isize, isize)> {
        match self {
            InputType::Key(KeyCode::Up) => Some((0, 1)),
            InputType::Key(KeyCode::Down) => Some((0, -1)),
            InputType::Key(KeyCode::Left) => Some((-1, 0)),
            InputType::Key(KeyCode::Right) => Some((1, 0)),
            _ => None,
        }
    }

    /// Returns the 0-F keycodes.
    pub fn hex_keycodes() -> [InputType; 16] {
        [
            InputType::Key(KeyCode::Key0),
            InputType::Key(KeyCode::Key1),
            InputType::Key(KeyCode::Key2),
            InputType::Key(KeyCode::Key3),
            InputType::Key(KeyCode::Key4),
            InputType::Key(KeyCode::Key5),
            InputType::Key(KeyCode::Key6),
            InputType::Key(KeyCode::Key7),
            InputType::Key(KeyCode::Key8),
            InputType::Key(KeyCode::Key9),
            InputType::Key(KeyCode::A),
            InputType::Key(KeyCode::B),
            InputType::Key(KeyCode::C),
            InputType::Key(KeyCode::D),
            InputType::Key(KeyCode::E),
            InputType::Key(KeyCode::F),
        ]
    }

    pub fn input_to_num(&self) -> Option<usize> {
        InputType::hex_keycodes().iter().position(|x| x == self)
    }
}

/// A struct containing user inputs.
pub struct InputRes {
    /// The currently pressed keys, along with when they started being held.
    pressed_inputs: HashMap<InputType, u64>,
    /// The current time in frames since the start.
    current_time: u64,
    /// Stores the previous 16 key inputs.
    key_history: utils::SizedHeadedArray<InputType, 16>,
    /// The current cursor position.
    cursor_position: (i32, i32),
    /// Scroll delta
    scroll_delta: f32,

    /// The time the last click was released.
    last_click_release: Option<u64>,
    /// The last time the cursor was moved.
    last_cursor_move: Option<u64>,

    /// The delay between a key initially being pressed and it repeating.
    /// A value of None means no repeat.
    key_delay: Option<u64>,
    /// The time between repeating key presses.
    key_repeat: u64,
}

impl InputRes {
    pub fn new() -> InputRes {
        InputRes {
            pressed_inputs: HashMap::new(),
            current_time: 0,
            key_history: utils::SizedHeadedArray::new(),
            cursor_position: (0, 0),
            scroll_delta: 0.,
            last_click_release: None,
            last_cursor_move: None,
            key_delay: Some(7),
            key_repeat: 3,
        }
    }

    /// Next frame reset.
    pub fn next_frame(&mut self) {
        self.current_time += 1;
    }

    /// Gets the current time
    pub fn get_time(&self) -> u64 {
        self.current_time
    }

    /// Returns the current cursor position on screen.
    ///
    /// Returns None in the event the cursor is off-screen.
    pub fn get_cursor_position(&self) -> Option<(i32, i32)> {
        // Check if within bounds of the screen.
        if self.cursor_position.0 < 0
            || self.cursor_position.0 >= 160
            || self.cursor_position.1 < 0
            || self.cursor_position.1 >= 144
        {
            Some(self.cursor_position)
        } else {
            None
        }
    }

    /// Returns the current tile the cursor is hovering over.
    ///
    /// Returns None in the event the cursor is off-screen.
    pub fn get_cursor_tile_position(&self) -> Option<(i32, i32)> {
        let cursor_pos = self.get_cursor_position()?;
        Some((cursor_pos.0 / 8, cursor_pos.1 / 8))
    }

    /// Returns the current scroll delta.
    pub fn get_scroll_delta(&self) -> i32 {
        self.scroll_delta as i32
    }

    /// Sets an input to be pressed
    fn press_key(&mut self, key: InputType) {
        // Check it's not from a repeating keypress
        if self.pressed_inputs.contains_key(&key) {
            return;
        }
        self.pressed_inputs.insert(key, self.current_time);
    }

    /// Sets an input to no longer be pressed
    fn release_key(&mut self, key: InputType) {
        self.pressed_inputs.remove(&key);
    }

    /// Returns the currently pressed inputs.
    pub fn get_pressed_inputs(&self) -> HashSet<InputType> {
        self.pressed_inputs.keys().cloned().collect()
    }

    /// Returns true if all the items in the array are being pressed.
    /// NOTE: Other inputs may be pressed. If you do not wish for other inputs
    /// to be pressed, check `InputRes::exclusively_pressed`
    pub fn is_pressed(&self, inputs: &[InputType]) -> bool {
        HashSet::from_iter(inputs.iter().cloned()).is_subset(&self.get_pressed_inputs())
    }

    /// Returns true if only the items in the array are being pressed.
    /// NOTE: Other inputs may not be pressed. If you wish for other inputs
    /// to be allowed to be pressed, check `InputRes::is_pressed`
    pub fn exclusively_pressed(&self, inputs: &[InputType]) -> bool {
        self.get_pressed_inputs() == HashSet::from_iter(inputs.iter().cloned())
    }

    /// Check if an input was pressed this frame.
    pub fn just_pressed(&self, input: &InputType) -> bool {
        if let Some(pressed_at) = self.pressed_inputs.get(input) {
            return *pressed_at == self.get_time();
        }
        false
    }

    /// Check if an input with key-delay/repeat has been triggered
    pub fn dr_pressed(&self, input: &InputType) -> bool {
        if let Some(pressed_at) = self.pressed_inputs.get(input) {
            let time = self.get_time();
            let since_pressed = time - *pressed_at;
            // Key started being pressed this frame
            if since_pressed == 0 {
                true
            } else if let Some(delay) = self.key_delay {
                if since_pressed < delay {
                    false
                } else {
                    (since_pressed - delay) % self.key_repeat == 0
                }
            } else {
                false
            }
        } else {
            // Key isn't even down.
            false
        }
    }

    /// Check if a double click has occured this frame.
    pub fn double_click(&self) -> bool {
        // Clicked within 30 frames of releasing and DID NOT MOVE THE CURSOR.
        let current_click_time = self
            .pressed_inputs
            .get(&InputType::Mouse(MouseButton::Left));

        if let Some(current_click_time) = current_click_time {
            if let Some(last_click_release_time) = self.last_click_release {
                return *current_click_time <= last_click_release_time + DOUBLE_CLICK_FRAME_COUNT;
            }
        }

        false
    }
}

fn update_inputs(
    mut input: ResMut<InputRes>,
    wnds: Res<Windows>,
    mut keyboard: EventReader<KeyboardInput>,
    mut mouse: EventReader<MouseButtonInput>,
    mut wheel: EventReader<MouseWheel>,
) {
    input.next_frame();

    // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(cursor_pos) = wnd.cursor_position() {
        // NOTE: This works for when the user can manually resize the window,
        // where the game screen runs in letterboxed mode

        // // The window's width and height as a ratio of the
        // // size of the game to the width/height.
        // let win_width_ratio = wnd.width() / 160.;
        // let win_height_ratio = wnd.height() / 144.;

        // // The size of the game relative to its default size (144 x 160)
        // let game_ratio = f32::min(win_width_ratio, win_height_ratio);

        // // The distance from the edge of the screen that the game is.
        // let border_x = (wnd.width() - 160. * game_ratio) / 2.;
        // let border_y = (wnd.height() - 144. * game_ratio) / 2.;

        // let cursor_x = ((cursor_pos.x - border_x) / game_ratio) as i32;
        // let cursor_y = ((cursor_pos.y - border_y) / game_ratio) as i32;

        let cursor_x = cursor_pos.x as i32;
        let cursor_y = cursor_pos.y as i32;

        // The Y-value does not conform to standard computer-based positioning.
        // Adjust it so it is.
        // 143 is the correct value so the top is 0 and bottom is 143. Not 144.
        let cursor_y = -cursor_y + 143;

        let new_cursor_pos = (cursor_x, cursor_y);

        if new_cursor_pos != input.cursor_position {
            input.last_cursor_move = Some(input.current_time)
        }

        input.cursor_position = new_cursor_pos;
    }

    // For each keyboard event (releases or presses)
    for key in keyboard.iter() {
        // For some reason Keycode can be None?
        if let Some(keycode) = key.key_code {
            if key.state.is_pressed() {
                input.press_key(InputType::Key(keycode))
            } else {
                input.release_key(InputType::Key(keycode))
            }
        }
    }

    // For each mouse button event (releases or presses)
    for btn in mouse.iter() {
        if btn.state.is_pressed() {
            input.press_key(InputType::Mouse(btn.button));
        } else {
            input.release_key(InputType::Mouse(btn.button));

            if btn.button == MouseButton::Left {
                input.last_click_release = Some(input.current_time);
            }
        }
    }

    // Get the remainder from the previous frame
    // and set it to the current frame
    input.scroll_delta = input.scroll_delta.rem_euclid(1.);
    // For each mouse wheel event (there should only be one)
    // Note: Wheel is empty if there is no change in scroll.
    for whl in wheel.iter() {
        input.scroll_delta += whl.y;
    }
}
