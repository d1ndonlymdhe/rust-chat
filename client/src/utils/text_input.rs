use ui::{
    components::{
        common::{Alignment, Component, Length, def_key_handler},
        text_input::TextInput,
    },
    raylib::ffi::KeyboardKey,
};

use crate::utils::state::State;

pub enum TextInputType {
    Password,
    Text,
}

pub fn text_input(
    value: String,
    set_val: State<dyn FnMut(&str) -> ()>,
    input_type: TextInputType,
    submit_func: Option<fn()->()>,
) -> Component {
    let content = match input_type {
        TextInputType::Password => {
            let hidden_str = "*".repeat(value.len());
            &(hidden_str.clone())
        }
        TextInputType::Text => &(value.clone()),
    };

    return TextInput::get_builder()
        .content(content)
        .main_align(Alignment::Center)
        .dim((Length::FILL, Length::FitPer(180)))
        .font_size(26)
        .padding((5, 0, 5, 0))
        .wrap(true)
        .on_key(Box::new(move |ev| {
            if let Some(submit_func) = &submit_func {
                if let Some(keyboard_key) = ev.key {
                    if keyboard_key == KeyboardKey::KEY_ENTER {
                        submit_func();
                        return false;
                    }
                }
            }
            let (_, new_email) = def_key_handler(ev, &value);
            set_val.clone().borrow_mut()(&new_email);
            false
        }))
        .build();
}
