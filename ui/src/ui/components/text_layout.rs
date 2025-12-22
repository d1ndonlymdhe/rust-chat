use std::{cell::RefCell, ffi::CString, rc::Rc};

use crate::raylib::color::Color;

use crate::components::{
    common::{Alignment, Base, Length, MouseEvent, tabbed_print},
    layout::{Layout, LayoutProps},
    raw_text::RawText,
};

use colored::Colorize;
#[derive(Clone)]
pub struct TextLayoutProps {
    pub layout: LayoutProps,
    pub font_size: i32,
    pub wrap: bool,
    pub content: String,
    pub text_color: Color,
}

impl TextLayoutProps {
    pub fn new() -> Self {
        let layout = Layout::get_col_builder()
            .dim((Length::FIT, Length::FIT))
            .bg_color(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            })
            .main_align(Alignment::Start)
            .cross_align(Alignment::Start)
            .padding((0, 0, 0, 0))
            .gap(0)
            .flex(1.0)
            .overflow_x(false)
            .overflow_y(false);
        return Self {
            layout: layout,
            font_size: 24,
            wrap: true,
            content: String::from(""),
            text_color: Color::BLACK,
        };
    }
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.into();
        self
    }
    pub fn font_size(mut self, size: i32) -> Self {
        self.font_size = size;
        self
    }
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn dim(mut self, dim: (Length, Length)) -> Self {
        let layout = self.layout.dim(dim);
        self.layout = layout;
        return self;
    }
    pub fn bg_color(mut self, color: Color) -> Self {
        let layout = self.layout.bg_color(color);
        self.layout = layout;
        self
    }
    pub fn main_align(mut self, align: Alignment) -> Self {
        let layout = self.layout.main_align(align);
        self.layout = layout;
        self
    }
    pub fn cross_align(mut self, align: Alignment) -> Self {
        let layout = self.layout.cross_align(align);
        self.layout = layout;
        self
    }
    pub fn padding(mut self, padding: (i32, i32, i32, i32)) -> Self {
        let layout = self.layout.padding(padding);
        self.layout = layout;
        self
    }
    pub fn gap(mut self, gap: i32) -> Self {
        let layout = self.layout.gap(gap);
        self.layout = layout;
        self
    }
    pub fn dbg_name(mut self, name: &str) -> Self {
        let layout = self.layout.dbg_name(name);
        self.layout = layout;
        self
    }
    pub fn flex(mut self, flex: f32) -> Self {
        let layout = self.layout.flex(flex);
        self.layout = layout;
        self
    }
    pub fn on_click(mut self, f: Box<dyn FnMut(MouseEvent) -> bool>) -> Self {
        let layout = self.layout.on_click(f);
        self.layout = layout;
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }
    pub fn overflow_x(mut self, overflow: bool) -> Self {
        let layout = self.layout.overflow_x(overflow);
        self.layout = layout;
        self
    }
    pub fn overflow_y(mut self, overflow: bool) -> Self {
        let layout = self.layout.overflow_y(overflow);
        self.layout = layout;
        self
    }
    pub fn border_width(mut self, border_width: i32) -> Self {
        let layout = self.layout.border_width(border_width);
        self.layout = layout;
        self
    }
    pub fn border_color(mut self, border_color: Color) -> Self {
        let layout = self.layout.border_color(border_color);
        self.layout = layout;
        self
    }

    pub fn build(self) -> Rc<RefCell<TextLayout>> {
        let layout = self.layout;
        return Rc::new(RefCell::new(TextLayout {
            layout: layout.get_layout(),
            font_size: self.font_size,
            wrap: self.wrap,
            content: self.content,
            text_color: self.text_color,
        }));
    }
}

pub struct TextLayout {
    layout: Layout,
    font_size: i32,
    wrap: bool,
    content: String,
    text_color: Color,
}

impl TextLayout {
    pub fn get_builder() -> TextLayoutProps {
        return TextLayoutProps::new();
    }
}

fn get_text_rows(content: &str, max_width: i32, font_size: i32) -> Vec<String> {
    let mut rows = vec![];
    let words: Vec<&str> = content.split_whitespace().collect();
    let mut current_row = String::new();
    for word in words {
        let test_row = if current_row.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_row, word)
        };
        let text_width;
        unsafe {
            let c_text = CString::new(test_row.as_str()).unwrap();
            text_width = raylib::ffi::MeasureText(c_text.as_ptr(), font_size);
        }
        if text_width <= max_width {
            current_row = test_row;
        } else {
            if !current_row.is_empty() {
                rows.push(current_row);
            }
            current_row = word.to_string();
        }
    }
    if !current_row.is_empty() {
        rows.push(current_row);
    }
    rows
}

impl Base for TextLayout {
    fn set_pos(&mut self, pos: (i32, i32)) {
        self.layout.set_pos(pos);
    }

    fn draw(
        &self,
        draw_handle: &mut raylib::prelude::RaylibDrawHandle,
    ) -> Vec<super::common::AbsoluteDraw> {
        self.layout.draw(draw_handle)
    }

    fn get_mouse_event_handlers(&self, mouse_event: super::common::MouseEvent) -> Vec<String> {
        let layout = &self.layout;
        let mouse_pos = mouse_event.pos;
        let max_x = layout.pos.0 + layout.draw_dim.0;
        let max_y = layout.pos.1 + layout.draw_dim.1;
        if mouse_event.left_button_down
            && mouse_pos.0 as i32 >= layout.pos.0
            && mouse_pos.0 as i32 <= max_x
            && mouse_pos.1 as i32 >= layout.pos.1
            && mouse_pos.1 as i32 <= max_y
        {
            return vec![self.get_id()];
        } else {
            return vec![];
        }
    }

    fn get_on_click(
        &self,
    ) -> std::rc::Rc<std::cell::RefCell<dyn FnMut(super::common::MouseEvent) -> bool>> {
        self.layout.on_click.clone()
    }

    fn get_key_event_handlers(&self, _key_event: super::common::KeyEvent) -> Vec<String> {
        vec![]
    }

    fn get_on_key(
        &self,
    ) -> std::rc::Rc<std::cell::RefCell<dyn FnMut(super::common::KeyEvent) -> bool>> {
        Rc::new(RefCell::new(|_key_event| true))
    }

    fn get_paddings(&self) -> (i32, i32, i32, i32) {
        self.layout.get_paddings()
    }

    fn set_raw_dim(&mut self, parent_draw_dim: (i32, i32)) {
        let layout_paddings = self.layout.padding;

        let layout = &mut self.layout;
        layout.children = vec![RawText::new(
            &self.content,
            self.font_size,
            (0, 0, 0, 0),
            // layout.padding,
            self.text_color,
        )];
        let content_width = unsafe {
            let c_text = CString::new(self.content.as_str()).unwrap();
            raylib::ffi::MeasureText(c_text.as_ptr(), self.font_size)
            // + layout.padding.0
            // + layout.padding.2
        };
        let (mut draw_width, mut draw_height) = crate::components::common::get_draw_dim(
            layout.dim,
            parent_draw_dim,
            &layout.children,
            layout.direction,
            layout.border_width,
        );

        if self.wrap {
            let max_width = draw_width - layout.padding.0 - layout.padding.2;
            if content_width > max_width {
                let text_rows = get_text_rows(&self.content, max_width, self.font_size);
                layout.children = text_rows
                    .iter()
                    .map(|row| {
                        RawText::new(
                            row,
                            self.font_size,
                            (0, 0, 0, 0), // layout.padding
                            self.text_color,
                        ) as Rc<RefCell<dyn Base>>
                    })
                    .collect();
            }
        }

        if layout.dim.0 == Length::FIT {
            draw_width = layout
                .children
                .iter()
                .map(|child| child.borrow().get_draw_dim().0)
                .max()
                .unwrap()
        }
        if let Length::FitPer(p) = layout.dim.0 {
            draw_width = layout
                .children
                .iter()
                .map(|child| child.borrow().get_draw_dim().0)
                .max()
                .unwrap();
            draw_width = (draw_width * p) / 100
        }
        if layout.dim.1 == Length::FIT {
            draw_height = layout
                .children
                .iter()
                .map(|child| child.borrow().get_draw_dim().1)
                .sum::<i32>()
                + layout.gap * (layout.children.len() as i32 - 1);
        }
        if let Length::FitPer(p) = layout.dim.1 {
            draw_height = layout
                .children
                .iter()
                .map(|child| child.borrow().get_draw_dim().1)
                .max()
                .unwrap();
            draw_height = (draw_height * p) / 100
        }

        layout.draw_dim = (
            draw_width + layout_paddings.0 + layout_paddings.2,
            draw_height + layout_paddings.1 + layout_paddings.3,
        );
    }

    fn get_draw_dim(&self) -> (i32, i32) {
        self.layout.get_draw_dim()
    }

    fn get_draw_pos(&self) -> (i32, i32) {
        self.layout.get_draw_pos()
    }

    fn measure_dimensions(&mut self, parent_draw_dim: (i32, i32), id: usize) -> usize {
        let ret_id = self.layout.measure_dimensions(parent_draw_dim, id);
        self.set_raw_dim(parent_draw_dim);
        ret_id
    }

    fn measure_positions(&mut self, parent_pos: (i32, i32)) {
        self.layout.measure_positions(parent_pos);
    }

    fn measure_overflows(
        &mut self,
        parent_draw_dim: (i32, i32),
        parent_pos: (i32, i32),
        scroll_map: &mut std::collections::HashMap<String, i32>,
        y_offset: i32,
    ) {
        self.layout
            .measure_overflows(parent_draw_dim, parent_pos, scroll_map, y_offset);
    }

    fn get_overflow(&self) -> (bool, bool) {
        self.layout.get_overflow()
    }

    fn get_flex(&self) -> f32 {
        self.layout.get_flex()
    }

    fn debug_dims(&self, depth: usize) {
        let layout = &self.layout;
        tabbed_print(
            &format!(
                "<layouttext width={} height={} x={} y={} bg_color={} text_color={} padding=({},{},{},{}) gap={} dir={:?} main_align={:?} cross_align={:?} name='{}' flex={}>",
                layout.draw_dim.0,
                layout.draw_dim.1,
                layout.pos.0,
                layout.pos.1,
                "███████"
                    .truecolor(
                        self.layout.bg_color.r,
                        self.layout.bg_color.g,
                        self.layout.bg_color.b
                    )
                    .bold(),
                "███████"
                    .truecolor(self.text_color.r, self.text_color.g, self.text_color.b)
                    .bold(),
                layout.padding.0,
                layout.padding.1,
                layout.padding.2,
                layout.padding.3,
                layout.gap,
                layout.direction,
                layout.main_align,
                layout.cross_align,
                self.get_id(),
                layout.flex
            ),
            depth,
        );
        for (_i, child) in layout.children.iter().enumerate() {
            child.borrow().debug_dims(depth + 1);
        }
        tabbed_print("</layouttext>", depth);
    }

    fn get_id(&self) -> String {
        self.layout.get_id()
    }

    fn get_by_id(&self, id: &str) -> Option<std::rc::Rc<std::cell::RefCell<dyn Base>>> {
        self.layout.get_by_id(id)
    }

    fn get_position(&self) -> super::common::Position {
        self.layout.get_position()
    }
}
