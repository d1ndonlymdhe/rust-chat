use crate::components::common::*;
use raylib::{
    color::Color,
    ffi::{KeyboardKey, MouseButton},
    prelude::{RaylibDraw, RaylibDrawHandle},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc, vec};

pub struct UIRoot {}
impl UIRoot {
    pub fn start(builder: Box<dyn Fn() -> Component>, dim: (i32, i32), title: &str) {
        let (mut rl, thread) = raylib::init()
            .height(dim.1)
            .width(dim.0)
            .title(title)
            .build();

        rl.set_target_fps(60);

        let mut should_rebuild_ui = true;
        let mut scroll_map: HashMap<String, i32> = HashMap::new();
        let mut main_child = builder();
        let mut focused_id = None;

        while !rl.window_should_close() {
            let mouse_pos = rl.get_mouse_position();
            let left_mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

            let key = rl.get_key_pressed();
            let shift_down = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
                || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);

            let ctrl_down = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
                || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);

            let mouse_event = MouseEvent {
                pos: (mouse_pos.x as i32, mouse_pos.y as i32),
                left_button_down: left_mouse_pressed,
            };

            let mut d = rl.begin_drawing(&thread);
            let wheel_move = d.get_mouse_wheel_move_v();
            let scroll_y = wheel_move.y;

            let scroll_event = ScrollEvent {
                pos: (mouse_pos.x as i32, mouse_pos.y as i32),
                delta: scroll_y as i32,
            };

            let key_event = KeyEvent {
                key,
                shift_down,
                ctrl_down,
            };

            {
                let a = UIRoot::get_mouse_event_handlers(
                    main_child.clone(),
                    &mut focused_id,
                    mouse_event,
                );

                let b = if key_event.ctrl_down || key_event.shift_down || key_event.key.is_some() {
                    UIRoot::handle_key_event(main_child.clone(), focused_id.clone(), key_event)
                } else {
                    false
                };

                let c = UIRoot::get_scroll_event_handler(
                    main_child.clone(),
                    &mut scroll_map,
                    scroll_event,
                );
                if a || b || c {
                    should_rebuild_ui = true;
                }
            }
            if should_rebuild_ui {
                main_child = builder();
                UIRoot::measure_dimensions(main_child.clone(), dim);
                UIRoot::measure_positions(main_child.clone());
                UIRoot::measure_overflows(main_child.clone(), dim, &mut scroll_map);
                UIRoot::draw(&mut d, main_child.clone());
                should_rebuild_ui = false;
            }
        }
    }

    fn draw(draw_handle: &mut RaylibDrawHandle, root_child: Component) {
        draw_handle.clear_background(Color::BLACK);

        let child = root_child.borrow();
        let mut abs_draw = { child.draw(draw_handle) };

        loop {
            let mut new_abs_draws = vec![];
            for draw_instruction in abs_draw.iter() {
                let AbsoluteDraw { component_id } = draw_instruction;
                // let child = root.get_by_id(component_id);
                let child = UIRoot::get_by_id(root_child.clone(), component_id);
                if let Some(child) = child {
                    let child = child.borrow();
                    let child_pos = child.get_position();
                    match child_pos {
                        Position::Auto => {
                            panic!("No auto children should exist here")
                        }
                        Position::Sticky(_, _) => {
                            let more_abs_draw = child.draw(draw_handle);
                            new_abs_draws.extend(more_abs_draw);
                        }
                        Position::Abs(_, _) => {
                            let more_abs_draw = child.draw(draw_handle);
                            new_abs_draws.extend(more_abs_draw);
                        }
                    }
                }
            }
            if new_abs_draws.is_empty() {
                break;
            } else {
                abs_draw = new_abs_draws;
            }
        }
    }

    fn handle_key_event(
        root_child: Component,
        root_focused_id: Option<String>,
        key_event: KeyEvent,
    ) -> bool {
        if key_event.ctrl_down && key_event.key.is_some_and(|v| v == KeyboardKey::KEY_D) {
            UIRoot::debug_dims(root_child.clone());
        }
        if let Some(focused_id) = root_focused_id {
            if let Some(focused_child) = UIRoot::get_by_id(root_child.clone(), &focused_id) {
                let focused_child = focused_child.borrow();
                focused_child.execute_on_key(key_event);
                return true;
            }
        }
        false
    }
    fn get_mouse_event_handlers(
        root_child: Component,
        root_focused_id: &mut Option<String>,
        mouse_event: MouseEvent,
    ) -> bool {
        let child = root_child.clone();
        let hit_children = child.borrow().get_mouse_event_handlers(mouse_event);

        let mut focused_id = None;

        for child_id in hit_children.iter() {
            let child = UIRoot::get_by_id(root_child.clone(), &child_id);
            if let Some(child) = child {
                let child = child.borrow();
                let propagate = child.execute_on_click(mouse_event);
                if child.is_focusable() && focused_id.is_none() {
                    focused_id = Some(child_id.clone());
                }
                if !propagate {
                    break;
                }
            }
        }

        if mouse_event.left_button_down {
            *root_focused_id = focused_id;
            return true;
        }
        return false;
    }

    fn get_scroll_event_handler(
        root_child: Component,
        scroll_map: &mut HashMap<String, i32>,
        scroll_event: ScrollEvent,
    ) -> bool {
        if scroll_event.delta == 0 {
            return false;
        }
        let child = root_child.clone();
        if let Some(handler_id) = child.borrow().get_scroll_event_handler(scroll_event) {
            let entry = scroll_map.entry(handler_id);
            let scroll_offset = entry.or_insert(0);
            *scroll_offset -= scroll_event.delta * 15;
            return true;
        }
        false
    }

    fn measure_dimensions(root_child: Component, dim: (i32, i32)) {
        let mut mut_child = root_child.borrow_mut();
        mut_child.set_raw_dim(dim);
        mut_child.measure_dimensions(dim, 0);
    }
    fn measure_positions(root_child: Component) {
        root_child.borrow_mut().measure_positions((0, 0));
    }
    fn measure_overflows(
        root_child: Component,
        dim: (i32, i32),
        scroll_map: &mut HashMap<String, i32>,
    ) {
        root_child
            .borrow_mut()
            .measure_overflows(dim, (0, 0), scroll_map, 0);
    }
    fn debug_dims(root_child: Component) {
        tabbed_print(&format!("<root>",), 0);
        root_child.borrow().debug_dims(1);
        tabbed_print("</root>", 0);
    }
    fn get_by_id(root_child: Component, id: &str) -> Option<Rc<RefCell<dyn Base>>> {
        let child = root_child.clone();

        let is_target = {
            let borrowed_child = child.borrow();
            borrowed_child.get_id() == id
        };
        if is_target {
            match child.try_borrow_mut() {
                Ok(_) => Some(child.clone()),
                Err(_) => None,
            }
        } else {
            child.borrow().get_by_id(id)
        }
    }
}
