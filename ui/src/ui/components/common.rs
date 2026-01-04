use crate::raylib::prelude::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    FILL,
    FIT,
    FIXED(i32),
    FillPer(i32),
    FitPer(i32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ID {
    Auto(String),
    Manual(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Auto,
    Sticky(i32, i32),
    Abs(i32, i32),
}

pub fn get_drawable_y_and_h(
    container_y: i32,
    container_height: i32,
    content_y: i32,
    content_height: i32,
) -> (i32, i32) {
    let y_min = container_y;
    let y_max = container_y + container_height;
    let bottom_y = content_y + content_height;

    let top_in = content_y >= y_min;
    let bottom_in = bottom_y <= y_max;

    let (draw_y, visible_height) = if top_in && bottom_in {
        (content_y, content_height)
    } else if !top_in && bottom_in {
        (y_min, bottom_y - y_min)
    } else if top_in && !bottom_in {
        (content_y, y_max - content_y)
    } else {
        (0, 0)
    };
    return (draw_y, visible_height);
}

#[derive(Debug, Clone)]
pub struct AbsoluteDraw {
    pub component_id: String,
}

impl AbsoluteDraw {
    pub fn new(component_id: &str) -> Self {
        return AbsoluteDraw {
            component_id: component_id.to_string(),
        };
    }
}

pub trait Base {
    fn set_pos(&mut self, pos: (i32, i32));
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) -> Vec<AbsoluteDraw>;
    fn get_mouse_event_handlers(&self, mouse_event: MouseEvent) -> Vec<String>;
    fn execute_on_click(&self, mouse_event: MouseEvent) -> bool {
        let f = self.get_on_click();
        let mut f = f.borrow_mut();
        f(mouse_event)
    }
    fn get_on_click(&self) -> Rc<RefCell<dyn FnMut(MouseEvent) -> bool>>;
    fn get_paddings(&self) -> (i32, i32, i32, i32);
    fn get_key_event_handlers(&self, key_event: KeyEvent) -> Vec<String>;
    fn get_scroll_event_handler(&self, scroll_event: ScrollEvent) -> Option<String> {
        let children = self.get_children();
        for child in children.iter() {
            let child = child.borrow();
            if let Some(handler_id) = child.get_scroll_event_handler(scroll_event) {
                return Some(handler_id);
            }
        }
        let overflow = self.get_overflow();
        let scroll_event_x = scroll_event.pos.0;
        let scroll_event_y = scroll_event.pos.1;
        if overflow.1 {
            let draw_pos = self.get_draw_pos();
            let draw_dim = self.get_draw_dim();
            let x = draw_pos.0;
            let y = draw_pos.1;
            let w = draw_dim.0;
            let h = draw_dim.1;
            let inside = scroll_event_x >= x
                && scroll_event_x <= x + w
                && scroll_event_y >= y
                && scroll_event_y <= y + h;
            if inside {
                return Some(self.get_id());
            }
        }
        None
    }
    fn execute_on_key(&self, key_event: KeyEvent) -> bool {
        let f = self.get_on_key();
        let mut f = f.borrow_mut();
        f(key_event)
    }
    fn get_on_key(&self) -> Rc<RefCell<dyn FnMut(KeyEvent) -> bool>>;
    fn set_raw_dim(&mut self, parent_draw_dim: (i32, i32));
    fn get_draw_dim(&self) -> (i32, i32);
    fn get_draw_pos(&self) -> (i32, i32);
    fn measure_dimensions(&mut self, parent_draw_dim: (i32, i32), id: usize) -> usize;
    fn measure_positions(&mut self, parent_pos: (i32, i32));
    fn measure_overflows(
        &mut self,
        parent_draw_dim: (i32, i32),
        parent_pos: (i32, i32),
        scroll_map: &mut HashMap<String, i32>,
        y_offset: i32,
    );
    fn get_overflow(&self) -> (bool, bool);
    fn get_flex(&self) -> f32;
    fn debug_dims(&self, depth: usize);
    fn get_children(&self) -> Vec<Rc<RefCell<dyn Base>>> {
        Vec::new()
    }
    fn set_children(&mut self, _new_children :Vec<Component>){
        // only override for Layout
    }
    fn get_id(&self) -> String;
    fn get_by_id(&self, id: &str) -> Option<Rc<RefCell<dyn Base>>>;
    fn is_focusable(&self) -> bool {
        false
    }
    fn get_position(&self) -> Position;
}

#[derive(Clone, Copy, Debug)]
pub struct MouseEvent {
    pub pos: (i32, i32),
    pub left_button_down: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct ScrollEvent {
    pub pos: (i32, i32),
    pub delta: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct KeyEvent {
    pub key: Option<KeyboardKey>,
    pub shift_down: bool,
    pub ctrl_down: bool,
}

pub fn get_draw_dim(
    dim: (Length, Length),
    parent_dim: (i32, i32),
    children: &Vec<Rc<RefCell<dyn Base>>>,
    direction: Direction,
    border_width: i32,
) -> (i32, i32) {
    let (width, height) = dim;
    let draw_width = match width {
        Length::FILL => parent_dim.0,
        Length::FIT => {
            let iter = children.iter().map(|child| child.borrow().get_draw_dim().0);
            match direction {
                Direction::Row => iter.sum(),
                Direction::Column => iter.max().unwrap_or(0),
            }
        }
        Length::FIXED(l) => l,
        Length::FillPer(p) => (parent_dim.0 * p) / 100,
        Length::FitPer(p) => {
            let iter = children.iter().map(|child| child.borrow().get_draw_dim().0);
            let fit_width = match direction {
                Direction::Row => iter.sum(),
                Direction::Column => iter.max().unwrap_or(0),
            };
            (fit_width * p) / 100
        }
    };

    let draw_height = match height {
        Length::FILL => parent_dim.1,
        Length::FIT => {
            let iter = children.iter().map(|child| {
                let child = child.borrow();
                let paddings = child.get_paddings();
                let dims = child.get_draw_dim();
                dims.1 + paddings.1 + paddings.3
            });
            match direction {
                Direction::Row => iter.max().unwrap_or(0),
                Direction::Column => iter.sum(),
            }
        }
        Length::FIXED(l) => l,
        Length::FillPer(p) => (parent_dim.1 * p) / 100,
        Length::FitPer(p) => {
            let iter = children.iter().map(|child| {
                let child = child.borrow();
                let paddings = child.get_paddings();
                let dims = child.get_draw_dim();
                dims.1 + paddings.1 + paddings.3
            });
            let fit_height = match direction {
                Direction::Row => iter.max().unwrap_or(0),
                Direction::Column => iter.sum(),
            };
            (fit_height * p) / 100
        }
    };
    // (draw_width,draw_height)
    (
        draw_width - border_width * 2,
        draw_height - border_width * 2,
    )
}

pub fn tabbed_print(text: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}{}", indent, text);
}

pub fn generate_id() -> String {
    // Uuid::new_v4().to_string()
    "".to_string()
}

pub fn keyboard_key_to_char(key: KeyboardKey) -> Option<char> {
    match key {
        KeyboardKey::KEY_A => Some('a'),
        KeyboardKey::KEY_B => Some('b'),
        KeyboardKey::KEY_C => Some('c'),
        KeyboardKey::KEY_D => Some('d'),
        KeyboardKey::KEY_E => Some('e'),
        KeyboardKey::KEY_F => Some('f'),
        KeyboardKey::KEY_G => Some('g'),
        KeyboardKey::KEY_H => Some('h'),
        KeyboardKey::KEY_I => Some('i'),
        KeyboardKey::KEY_J => Some('j'),
        KeyboardKey::KEY_K => Some('k'),
        KeyboardKey::KEY_L => Some('l'),
        KeyboardKey::KEY_M => Some('m'),
        KeyboardKey::KEY_N => Some('n'),
        KeyboardKey::KEY_O => Some('o'),
        KeyboardKey::KEY_P => Some('p'),
        KeyboardKey::KEY_Q => Some('q'),
        KeyboardKey::KEY_R => Some('r'),
        KeyboardKey::KEY_S => Some('s'),
        KeyboardKey::KEY_T => Some('t'),
        KeyboardKey::KEY_U => Some('u'),
        KeyboardKey::KEY_V => Some('v'),
        KeyboardKey::KEY_W => Some('w'),
        KeyboardKey::KEY_X => Some('x'),
        KeyboardKey::KEY_Y => Some('y'),
        KeyboardKey::KEY_Z => Some('z'),

        KeyboardKey::KEY_SPACE => Some(' '),
        KeyboardKey::KEY_ENTER => Some('\n'),

        KeyboardKey::KEY_COMMA => Some(','),
        KeyboardKey::KEY_PERIOD => Some('.'),
        KeyboardKey::KEY_APOSTROPHE => Some('\''),
        KeyboardKey::KEY_SEMICOLON => Some(';'),
        KeyboardKey::KEY_SLASH => Some('/'),
        KeyboardKey::KEY_BACKSLASH => Some('\\'),
        KeyboardKey::KEY_LEFT_BRACKET => Some('['),
        KeyboardKey::KEY_RIGHT_BRACKET => Some(']'),
        KeyboardKey::KEY_MINUS => Some('-'),
        KeyboardKey::KEY_EQUAL => Some('='),
        KeyboardKey::KEY_GRAVE => Some('`'),

        KeyboardKey::KEY_ZERO => Some('0'),
        KeyboardKey::KEY_ONE => Some('1'),
        KeyboardKey::KEY_TWO => Some('2'),
        KeyboardKey::KEY_THREE => Some('3'),
        KeyboardKey::KEY_FOUR => Some('4'),
        KeyboardKey::KEY_FIVE => Some('5'),
        KeyboardKey::KEY_SIX => Some('6'),
        KeyboardKey::KEY_SEVEN => Some('7'),
        KeyboardKey::KEY_EIGHT => Some('8'),
        KeyboardKey::KEY_NINE => Some('9'),
        _ => None,
    }
}

pub fn def_key_handler(key_event: KeyEvent, text: &str) -> (bool, String) {
    let mut text = text.to_string();
    if let Some(key) = key_event.key {
        if key == KeyboardKey::KEY_BACKSPACE {
            text.pop();
            return (true,text);
        }
        if let Some(mut c) = keyboard_key_to_char(key) {
            if key_event.shift_down {
                c = shift_character(c);
            }
            text.push(c);
            return (true,text);
        }
    }
    (false,text)
}

pub fn shift_character(c: char) -> char {
    match c {
        'a'..='z' => ((c as u8) - 32) as char,
        'A'..='Z' => c,
        '0' => ')',
        '1' => '!',
        '2' => '@',
        '3' => '#',
        '4' => '$',
        '5' => '%',
        '6' => '^',
        '7' => '&',
        '8' => '*',
        '9' => '(',
        '-' => '_',
        '=' => '+',
        '[' => '{',
        ']' => '}',
        '\\' => '|',
        ';' => ':',
        '\'' => '"',
        ',' => '<',
        '.' => '>',
        '/' => '?',
        '`' => '~',
        _ => c,
    }
}

pub type Component = Rc<RefCell<dyn Base>>;