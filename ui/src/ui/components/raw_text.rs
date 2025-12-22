use crate::components::common::*;
use raylib::prelude::*;
use std::{cell::RefCell, collections::HashMap, ffi::CString, rc::Rc};

pub struct RawText {
    pub content: String,
    pub font_size: i32,
    pub pos: (i32, i32),
    pub dbg_name: ID,
    pub padding: (i32, i32, i32, i32), // top, right, bottom, left
    pub color: Color,
    pub overflowed: bool,
}

impl Base for RawText {
    fn set_pos(&mut self, pos: (i32, i32)) {
        self.pos = pos;
    }
    fn get_draw_pos(&self) -> (i32, i32) {
        self.pos
    }
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) -> Vec<AbsoluteDraw> {
        if self.overflowed {
            return vec![];
        }
        draw_handle.draw_text(
            &self.content,
            self.pos.0,
            self.pos.1,
            self.font_size,
            self.color,
        );
        vec![]
    }
    fn get_paddings(&self) -> (i32, i32, i32, i32) {
        self.padding
    }
    fn measure_overflows(
        &mut self,
        parent_draw_dim: (i32, i32),
        parent_pos: (i32, i32),
        _scroll_map: &mut HashMap<String, i32>,
        y_offset: i32,
    ) {
        let (start_y, visible_height) = get_drawable_y_and_h(
            parent_pos.1,
            parent_draw_dim.1,
            self.pos.1 - y_offset,
            self.font_size,
        );
        self.set_raw_dim((self.get_draw_dim().0, visible_height));
        self.set_pos((self.get_draw_pos().0, start_y));
    }
    fn set_raw_dim(&mut self, parent_draw_dim: (i32, i32)) {
        if parent_draw_dim.1 < self.font_size {
            self.overflowed = true;
        } else {
            self.overflowed = false;
        }
    }
    fn get_draw_dim(&self) -> (i32, i32) {
        let width;
        unsafe {
            let c_text = CString::new(self.content.as_str()).unwrap();
            width = raylib::ffi::MeasureText(c_text.as_ptr(), self.font_size);
        }
        (
            width + self.padding.0 + self.padding.2,
            self.font_size + self.padding.1 + self.padding.3,
        )
    }
    fn measure_dimensions(&mut self, parent_draw_dim: (i32, i32), id: usize) -> usize {
        self.set_raw_dim(parent_draw_dim);
        let ret_id = id + 1;
        if let ID::Auto(_) = &self.dbg_name {
            self.dbg_name = ID::Auto(ret_id.to_string());
        }
        ret_id
    }
    fn measure_positions(&mut self, parent_pos: (i32, i32)) {
        self.pos = (parent_pos.0, parent_pos.1);
    }
    fn debug_dims(&self, depth: usize) {
        tabbed_print(
            &format!(
                "<text width={} height={} x={} y={}>",
                self.get_draw_dim().0,
                self.get_draw_dim().1,
                self.pos.0,
                self.pos.1
            ),
            depth,
        );
        tabbed_print(&self.content, depth + 1);
        tabbed_print("</text>", depth);
    }
    fn get_flex(&self) -> f32 {
        1.0
    }

    fn get_mouse_event_handlers(&self, _mouse_event: MouseEvent) -> Vec<String> {
        Vec::new()
    }

    fn get_id(&self) -> String {
        match &self.dbg_name {
            ID::Auto(name) => name.clone(),
            ID::Manual(name) => name.clone(),
        }
    }
    fn get_by_id(&self, _id: &str) -> Option<Rc<RefCell<dyn Base>>> {
        None
    }

    fn get_on_click(&self) -> Rc<RefCell<dyn FnMut(MouseEvent) -> bool>> {
        Rc::new(RefCell::new(|_mouse_event| true))
    }

    fn get_key_event_handlers(&self, _key_event: KeyEvent) -> Vec<String> {
        vec![]
    }

    fn get_on_key(&self) -> Rc<RefCell<dyn FnMut(KeyEvent) -> bool>> {
        Rc::new(RefCell::new(|_key_event| true))
    }
    fn get_overflow(&self) -> (bool, bool) {
        (false, false)
    }
    fn get_scroll_event_handler(&self, _scroll_event: ScrollEvent) -> Option<String> {
        None
    }
    fn get_position(&self) -> Position {
        Position::Auto
    }
}

impl RawText {
    pub fn new(
        content: &str,
        font_size: i32,
        padding: (i32, i32, i32, i32),
        color: Color,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            content: content.to_string(),
            font_size,
            pos: (0, 0),
            padding,
            dbg_name: ID::Auto(generate_id()),
            color,
            overflowed: false,
        }))
    }
}
