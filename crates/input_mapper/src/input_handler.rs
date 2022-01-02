use crate::inputs::CustomInput;
use std::collections::HashMap;

pub struct InputHandler<T> {
    actions: HashMap<T, CustomInput>,
}

impl<T> Default for InputHandler<T> {
    fn default() -> Self {
        InputHandler {
            actions: HashMap::new(),
        }
    }
}
