use std::{cell::RefCell, collections::HashMap, rc::Rc, vec};

use colored::Colorize;
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::components::common::{
    AbsoluteDraw, Alignment, Base, Component, Direction, ID, KeyEvent, Length, MouseEvent,
    Position, generate_id, get_drawable_y_and_h, tabbed_print,
};

pub struct Layout {
    pub children: Vec<Component>,
    pub dim: (Length, Length),
    pub draw_dim: (i32, i32),
    pub pos: (i32, i32),
    pub bg_color: Color,
    pub direction: Direction,
    // Padding (top, right, bottom, left)
    pub padding: (i32, i32, i32, i32),
    pub border_width: i32,
    pub border_color: Color,
    pub main_align: Alignment,
    pub cross_align: Alignment,
    pub gap: i32,
    pub dbg_name: ID,
    pub flex: f32,
    pub on_click: Rc<RefCell<dyn FnMut(MouseEvent) -> bool>>,
    pub on_key: Rc<RefCell<dyn FnMut(KeyEvent) -> bool>>,
    pub children_func: Option<Rc<RefCell<dyn Fn() -> Vec<Component>>>>,
    pub overflow: (bool, bool),
    pub scroll_offset: i32,
    pub position: Position,
}

pub struct LayoutProps {
    layout: Layout,
}

impl Clone for LayoutProps {
    fn clone(&self) -> Self {
        Self {
            layout: Layout {
                children: self.layout.children.clone(),
                dim: self.layout.dim,
                draw_dim: self.layout.draw_dim,
                pos: self.layout.pos,
                bg_color: self.layout.bg_color,
                direction: self.layout.direction,
                padding: self.layout.padding,
                main_align: self.layout.main_align,
                cross_align: self.layout.cross_align,
                gap: self.layout.gap,
                dbg_name: self.layout.dbg_name.clone(),
                flex: self.layout.flex,
                on_click: self.layout.on_click.clone(),
                children_func: self.layout.children_func.clone(),
                overflow: self.layout.overflow,
                scroll_offset: self.layout.scroll_offset,
                position: self.layout.position,
                on_key: self.layout.on_key.clone(),
                border_width: self.layout.border_width,
                border_color: self.layout.border_color,
            },
        }
    }
}

impl LayoutProps {
    pub fn new() -> Self {
        Self {
            layout: Layout {
                children: vec![],
                dim: (Length::FILL, Length::FILL),
                draw_dim: (0, 0),
                pos: (0, 0),
                bg_color: Color {
                    r: 0,
                    b: 0,
                    g: 0,
                    a: 0,
                },
                direction: Direction::Row,
                main_align: Alignment::Start,
                cross_align: Alignment::Start,
                padding: (0, 0, 0, 0),
                gap: 0,
                dbg_name: ID::Auto(generate_id()),
                flex: 1.0,
                on_click: Rc::new(RefCell::new(|_mouse_event| true)),
                on_key: Rc::new(RefCell::new(|_key_event| true)),
                children_func: None,
                scroll_offset: 0,
                overflow: (false, true),
                position: Position::Auto,
                border_width: 0,
                border_color: Color::BLACK,
            },
        }
    }
    pub fn children(mut self, children: Vec<Component>) -> Self {
        self.layout.children = children;
        self
    }
    pub fn dim(mut self, dim: (Length, Length)) -> Self {
        self.layout.dim = dim;
        self
    }
    pub fn bg_color(mut self, color: Color) -> Self {
        self.layout.bg_color = color;
        self
    }
    pub fn direction(mut self, direction: Direction) -> Self {
        self.layout.direction = direction;
        self
    }
    pub fn main_align(mut self, align: Alignment) -> Self {
        self.layout.main_align = align;
        self
    }
    pub fn cross_align(mut self, align: Alignment) -> Self {
        self.layout.cross_align = align;
        self
    }
    pub fn padding(mut self, padding: (i32, i32, i32, i32)) -> Self {
        self.layout.padding = padding;
        self
    }
    pub fn gap(mut self, gap: i32) -> Self {
        self.layout.gap = gap;
        self
    }
    pub fn dbg_name(mut self, name: &str) -> Self {
        self.layout.dbg_name = ID::Manual(name.into());
        self
    }
    pub fn flex(mut self, flex: f32) -> Self {
        self.layout.flex = flex;
        self
    }
    pub fn on_click(mut self, f: Box<dyn FnMut(MouseEvent) -> bool>) -> Self {
        self.layout.on_click = Rc::new(RefCell::new(f));
        self
    }
    pub fn on_key(mut self, f: Box<dyn FnMut(KeyEvent) -> bool>) -> Self {
        self.layout.on_key = Rc::new(RefCell::new(f));
        self
    }
    pub fn children_func(mut self, f: Rc<RefCell<dyn Fn() -> Vec<Component>>>) -> Self {
        self.layout.children_func = Some(f);
        self
    }
    pub fn overflow_x(mut self, overflow: bool) -> Self {
        self.layout.overflow.0 = overflow;
        self
    }
    pub fn overflow_y(mut self, overflow: bool) -> Self {
        self.layout.overflow.1 = overflow;
        self
    }
    pub fn set_position(mut self, position: Position) -> Self {
        self.layout.position = position;
        self
    }
    pub fn border_width(mut self, border_width: i32) -> Self {
        self.layout.border_width = border_width;
        self
    }
    pub fn border_color(mut self, border_color: Color) -> Self {
        self.layout.border_color = border_color;
        self
    }
    pub fn build(self) -> Rc<RefCell<Layout>> {
        let layout = self.layout;
        Rc::new(RefCell::new(Layout {
            children: layout.children.clone(),
            dim: layout.dim,
            draw_dim: layout.draw_dim,
            pos: layout.pos,
            bg_color: layout.bg_color,
            direction: layout.direction,
            padding: layout.padding,
            main_align: layout.main_align,
            cross_align: layout.cross_align,
            gap: layout.gap,
            dbg_name: layout.dbg_name.clone(),
            flex: layout.flex,
            on_click: layout.on_click.clone(),
            children_func: layout.children_func.clone(),
            overflow: layout.overflow,
            scroll_offset: layout.scroll_offset,
            position: layout.position,
            on_key: layout.on_key.clone(),
            border_color: layout.border_color,
            border_width: layout.border_width,
        }))
    }
    pub fn get_layout(self) -> Layout {
        self.clone().layout
    }
}

impl Layout {
    pub fn get_row_builder() -> LayoutProps {
        LayoutProps::new()
    }
    pub fn get_col_builder() -> LayoutProps {
        LayoutProps::new().direction(Direction::Column)
    }
    pub fn get_scroll_height(&self) -> i32 {
        let (auto_children, abs_children, _sticky_children) = self.get_children_by_pos();

        let mut concerned_children = auto_children;
        concerned_children.extend(abs_children);

        concerned_children
            .iter()
            .map(|child| child.borrow().get_draw_pos().1 + child.borrow().get_draw_dim().1)
            .max()
            .unwrap_or(0)
    }
    /// (Auto,Abs,Sticky)
    pub fn get_children_by_pos(&self) -> (Vec<Component>, Vec<Component>, Vec<Component>) {
        let mut auto_children = vec![];
        let mut abs_children = vec![];
        let mut sticky_children = vec![];
        for child in self.get_children() {
            let child_ref = child.clone();
            let child_ref = child_ref.borrow();
            match child_ref.get_position() {
                Position::Auto => auto_children.push(child.clone()),
                Position::Abs(_, _) => abs_children.push(child.clone()),
                Position::Sticky(_, _) => sticky_children.push(child.clone()),
            }
        }
        (auto_children, abs_children, sticky_children)
    }
}

impl Base for Layout {
    fn get_paddings(&self) -> (i32, i32, i32, i32) {
        return self.padding;
    }
    fn set_pos(&mut self, pos: (i32, i32)) {
        self.pos = pos;
    }
    fn get_draw_pos(&self) -> (i32, i32) {
        self.pos
    }
    fn get_on_click(&self) -> Rc<RefCell<dyn FnMut(MouseEvent) -> bool>> {
        self.on_click.clone()
    }
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) -> Vec<AbsoluteDraw> {
        let visible_height = self.draw_dim.1;
        let start_y = self.pos.1;
        let border_color = self.border_color;
        let border_width = self.border_width;
        if visible_height > 0 {
            if border_width > 0 {
                draw_handle.draw_rectangle(
                    self.pos.0,
                    start_y,
                    self.draw_dim.0 + border_width * 2,
                    visible_height + border_width * 2,
                    border_color,
                );
                draw_handle.draw_rectangle(
                    self.pos.0 + border_width,
                    start_y + border_width,
                    self.draw_dim.0,
                    visible_height,
                    // self.draw_dim.0 - (border_width*2),
                    // visible_height - (border_width * 2),
                    self.bg_color,
                );
            } else {
                draw_handle.draw_rectangle(
                    self.pos.0,
                    start_y,
                    self.draw_dim.0,
                    visible_height,
                    self.bg_color,
                );
            }
        }

        let (auto_children, mut abs_children, sticky_children) = self.get_children_by_pos();
        abs_children.extend(sticky_children);
        let mut abs_draw = Vec::with_capacity(abs_children.len());

        for child in abs_children.iter() {
            abs_draw.push(AbsoluteDraw::new(&child.borrow().get_id()));
        }

        for child in auto_children.iter() {
            let child = child.clone();
            let abs_child_draws = child.borrow().draw(draw_handle);
            abs_draw.extend(abs_child_draws);
        }

        abs_draw
    }
    fn get_mouse_event_handlers(&self, mouse_event: MouseEvent) -> Vec<String> {
        let mut hit_children = Vec::new();
        for child in self.children.iter() {
            let child = child.clone();
            let hit = child.borrow().get_mouse_event_handlers(mouse_event);
            hit_children.extend(hit);
        }
        let mouse_pos = mouse_event.pos;
        let max_x = self.pos.0 + self.draw_dim.0;
        let max_y = self.pos.1 + self.draw_dim.1;
        if mouse_event.left_button_down
            && mouse_pos.0 as i32 >= self.pos.0
            && mouse_pos.0 as i32 <= max_x
            && mouse_pos.1 as i32 >= self.pos.1
            && mouse_pos.1 as i32 <= max_y
        {
            hit_children.push(self.get_id());
        }
        hit_children
    }
    fn get_children(&self) -> Vec<Component> {
        self.children.clone()
    }
    fn set_raw_dim(&mut self, parent_dim: (i32, i32)) {
        let (draw_width, draw_height) = crate::components::common::get_draw_dim(
            self.dim,
            parent_dim,
            &self.children,
            self.direction,
            self.border_width,
        );
        self.draw_dim = (draw_width, draw_height);
    }
    fn get_draw_dim(&self) -> (i32, i32) {
        self.draw_dim
    }
    fn measure_dimensions(&mut self, parent_draw_dim: (i32, i32), id: usize) -> usize {
        let (auto_children, mut abs_children, sticky_children) = self.get_children_by_pos();
        abs_children.extend(sticky_children);
        // let auto_children = self.get_children();
        let auto_children_len = auto_children.len() as i32;
        let total_flex = auto_children
            .iter()
            .map(|child| child.borrow().get_flex())
            .sum::<f32>();
        let mut ret_id = id;
        for child in auto_children.iter() {
            let mut child = child.borrow_mut();
            let flex: f32 = child.get_flex();

            match self.direction {
                Direction::Row => {
                    let allowed_width = self.draw_dim.0 - self.padding.0 - self.padding.2;
                    let allowed_width = allowed_width - (self.gap * (auto_children_len - 1));
                    let child_width = f32::floor(flex * (allowed_width as f32 / total_flex)) as i32;
                    let child_height = self.draw_dim.1 - self.padding.1 - self.padding.3;
                    child.set_raw_dim((child_width, child_height));
                    ret_id = child.measure_dimensions((child_width, child_height), ret_id + 1);
                }
                Direction::Column => {
                    let allowed_height = self.draw_dim.1 - self.padding.1 - self.padding.3;
                    let allowed_height = allowed_height - (self.gap * (auto_children_len - 1));
                    let child_height =
                        f32::floor(flex * (allowed_height as f32 / total_flex)) as i32;
                    let child_width = self.draw_dim.0 - self.padding.0 - self.padding.2;
                    child.set_raw_dim((child_width, child_height));
                    ret_id = child.measure_dimensions((child_width, child_height), ret_id + 1);
                }
            }
        }

        for child in abs_children.iter() {
            let mut child = child.borrow_mut();
            child.set_raw_dim(self.draw_dim);
            ret_id = child.measure_dimensions(self.draw_dim, ret_id + 1)
        }
        self.set_raw_dim(parent_draw_dim);
        ret_id = ret_id + 1;
        if let ID::Auto(_) = &self.dbg_name {
            self.dbg_name = ID::Auto(ret_id.to_string());
        }
        ret_id
    }
    fn measure_positions(&mut self, passed_pos: (i32, i32)) {
        let border_width = self.border_width;
        self.pos = (passed_pos.0, passed_pos.1);
        let mut padding_left = self.padding.0;
        let mut padding_top = self.padding.1;

        let (auto_children, mut abs_children, sticky_children) = self.get_children_by_pos();
        abs_children.extend(sticky_children);

        let auto_children_len = auto_children.len();

        //Special Handling of abs children
        for child in abs_children.iter() {
            let mut child = child.borrow_mut();
            let position = child.get_position();
            match position {
                Position::Auto => {
                    panic!("Auto positioned children should not reach here")
                }
                Position::Abs(x, y) => {
                    child.measure_positions((self.pos.0 + x, self.pos.1 + y));
                }
                Position::Sticky(x, y) => {
                    child.measure_positions((self.pos.0 + x, self.pos.1 + y));
                }
            }
        }

        if auto_children_len == 0 {
            return;
        }

        let mut cross_paddings = Vec::from(
            (0..auto_children_len)
                .map(|_| self.padding.1)
                .collect::<Vec<i32>>(),
        );

        let mut comparisons = [self.main_align, self.cross_align];

        if self.direction == Direction::Column {
            comparisons.swap(0, 1);
        }

        if comparisons[0] != Alignment::Start {
            let self_width = self.draw_dim.0 - self.padding.0 - self.padding.2;
            let children_width = auto_children
                .iter()
                .map(|child| child.borrow().get_draw_dim().0);
            let children_width = match self.direction {
                Direction::Row => children_width.sum(),
                Direction::Column => children_width.max().unwrap(),
            };
            let total_gap = self.gap * (auto_children_len as i32 - 1);
            let remaining_space = self_width - children_width - total_gap;

            if comparisons[0] == Alignment::Center {
                if self.direction == Direction::Column {
                    // If column and cross align center, each child is centered
                    for (idx, child) in auto_children.iter().enumerate() {
                        let child_width = child.borrow().get_draw_dim().0;
                        cross_paddings[idx] = self.padding.0 + (self_width - child_width) / 2;
                    }
                } else {
                    padding_left = self.padding.0 + remaining_space / 2;
                }
            }

            if comparisons[0] == Alignment::End {
                padding_left = self.padding.0 + remaining_space;
            }
        }

        if comparisons[1] != Alignment::Start {
            let self_height = self.draw_dim.1 - self.padding.1 - self.padding.3;
            let children_height = auto_children
                .iter()
                .map(|child| child.borrow().get_draw_dim().1);
            let children_height = match self.direction {
                Direction::Row => children_height.max().unwrap(),
                Direction::Column => children_height.sum(),
            };
            let total_gap = self.gap * (auto_children_len as i32 - 1);
            let remaining_space = self_height - children_height - total_gap;
            let remaining_space = remaining_space.max(0);
            if comparisons[1] == Alignment::Center {
                padding_top = self.padding.1 + remaining_space / 2;
            }
            if comparisons[1] == Alignment::End {
                padding_top = self.padding.1 + remaining_space;
            }
        }

        let mut next_pos = self.pos;
        if self.direction == Direction::Column && self.cross_align == Alignment::Center {
            next_pos.0 = self.pos.0 + cross_paddings[0] + border_width;
        } else {
            next_pos.0 += padding_left + border_width;
        }
        next_pos.1 += padding_top + border_width;
        for (idx, child) in auto_children.iter().enumerate() {
            let mut child = child.borrow_mut();
            child.measure_positions(next_pos);

            let (child_width, child_height) = child.get_draw_dim();
            if idx < auto_children_len - 1 {
                match self.direction {
                    Direction::Row => next_pos.0 += child_width + self.gap,
                    Direction::Column => {
                        if self.cross_align == Alignment::Center {
                            next_pos.0 = self.pos.0 + cross_paddings[idx + 1];
                        }
                        next_pos.1 += child_height + self.gap
                    }
                }
            }
        }
    }

    fn measure_overflows(
        &mut self,
        parent_draw_dim: (i32, i32),
        parent_pos: (i32, i32),
        scroll_map: &mut HashMap<String, i32>,
        y_offset: i32,
    ) {
        let draw_pos = self.get_draw_pos();
        let draw_dim = self.get_draw_dim();

        let content_y = draw_pos.1;
        let content_h = draw_dim.1;

        let self_id = self.get_id();

        let container_y = parent_pos.1;
        let container_h = parent_draw_dim.1;

        let scroll_height = self.get_scroll_height();
        let max_scroll = (scroll_height - content_h - content_y).max(0);

        let start_y = content_y - y_offset;
        let scroll_map_entry = scroll_map.entry(self_id.clone()).or_insert(0);
        *scroll_map_entry = (*scroll_map_entry).min(max_scroll).max(0);
        let scroll_top = *scroll_map_entry;
        let (start_y, visible_height) =
            get_drawable_y_and_h(container_y, container_h, start_y, content_h);

        self.draw_dim.1 = visible_height;
        self.pos.1 = start_y;

        for child in self.children.iter() {
            let mut child = child.borrow_mut();
            match child.get_position() {
                Position::Auto => {
                    child.measure_overflows(
                        self.get_draw_dim(),
                        self.get_draw_pos(),
                        scroll_map,
                        y_offset + scroll_top,
                    );
                }
                Position::Sticky(_, _) => {
                    child.measure_overflows(
                        self.get_draw_dim(),
                        self.get_draw_pos(),
                        scroll_map,
                        scroll_top,
                    );
                }
                Position::Abs(_, _) => {
                    child.measure_overflows(
                        self.get_draw_dim(),
                        self.get_draw_pos(),
                        scroll_map,
                        y_offset + scroll_top,
                    );
                }
            }
        }
    }

    fn get_flex(&self) -> f32 {
        self.flex
    }
    fn debug_dims(&self, depth: usize) {
        tabbed_print(
            &format!(
                "<layout width={} height={} x={} y={} bg_color={} padding=({},{},{},{}) gap={} dir={:?} main_align={:?} cross_align={:?} position={:?} name='{}' flex={}>",
                self.draw_dim.0,
                self.draw_dim.1,
                self.pos.0,
                self.pos.1,
                "███████"
                    .truecolor(self.bg_color.r, self.bg_color.g, self.bg_color.b)
                    .bold(),
                self.padding.0,
                self.padding.1,
                self.padding.2,
                self.padding.3,
                self.gap,
                self.direction,
                self.main_align,
                self.cross_align,
                self.get_position(),
                self.get_id(),
                self.flex
            ),
            depth,
        );
        for (_i, child) in self.children.iter().enumerate() {
            child.borrow().debug_dims(depth + 1);
        }
        tabbed_print("</layout>", depth);
    }

    fn get_id(&self) -> String {
        match &self.dbg_name {
            ID::Auto(name) => name.clone(),
            ID::Manual(name) => name.clone(),
        }
    }
    fn get_by_id(&self, id: &str) -> Option<Component> {
        for child in self.children.iter() {
            if child.borrow().get_id() == id {
                return Some(child.clone());
            }
            if let Some(found) = child.borrow().get_by_id(id) {
                return Some(found);
            }
        }
        None
    }

    fn get_key_event_handlers(&self, key_event: super::common::KeyEvent) -> Vec<String> {
        let mut hit_children = Vec::new();
        for child in self.children.iter() {
            let child = child.clone();
            let hit = child.borrow().get_key_event_handlers(key_event);
            hit_children.extend(hit);
        }
        return hit_children;
    }

    fn get_on_key(&self) -> Rc<RefCell<dyn FnMut(super::common::KeyEvent) -> bool>> {
        Rc::new(RefCell::new(|_key_event| true))
    }

    fn get_overflow(&self) -> (bool, bool) {
        self.overflow
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn set_children(&mut self, new_children :Vec<Component>) {
        self.children = new_children
    }
}
