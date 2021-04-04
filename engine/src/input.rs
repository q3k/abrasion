use std::collections::{BTreeMap,BTreeSet};

#[derive(Clone, Debug)]
pub struct Input {
    pub devices: BTreeMap<u64, Device>,
    highest_no: u64,
}
impl ecs::Global for Input {}

impl Input {
    pub fn allocate_device(&mut self) -> u64 {
        self.highest_no += 1;
        self.highest_no
    }
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            highest_no: 0,
        }
    }
    pub fn mouse_cursor(&self) -> Option<&MouseCursor> {
        for dev in self.devices.values() {
            if let &Device::MouseCursor(cursor) = &dev {
                return Some(&cursor);
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub enum Device {
    MouseCursor(MouseCursor),
}

#[derive(Clone, Debug)]
pub struct MouseCursor {
    // x and y coordinates [0.0, 1.0), top left at (0, 0).
    pub x: f32,
    pub y: f32,

    pressed: BTreeSet<MouseButton>,
}

impl MouseCursor {
    pub fn new() -> Self {
        Self {
            x: 0., y: 0.,
            pressed: BTreeSet::new(),
        }
    }

    pub fn set_mouse_pressed(&mut self, button: MouseButton) {
        self.pressed.insert(button);
    }

    pub fn set_mouse_released(&mut self, button: MouseButton) {
        self.pressed.remove(&button);
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other,
}
