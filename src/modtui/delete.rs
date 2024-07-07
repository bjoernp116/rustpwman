/* Copyright 2021 Martin Grap

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License. */


use std::sync::{Arc, Mutex};
use cursive::Cursive;
use cursive::views::{Dialog, LinearLayout, TextView};

use super::AppState;
use super::show_message;
use super::get_selected_entry_name;
use super::redraw_tui;
use super::get_special_styles;
use super::visualize_if_modified;


pub fn entry(s: &mut Cursive, state_temp_del: Arc<Mutex<AppState>>) { 
    let (danger_style, reverse_style) = get_special_styles();    

    match get_selected_entry_name(s) {
        Some(name) => {
            let res = Dialog::new()
            .title("Rustpwman delete entry")
            .padding_lrtb(2, 2, 1, 1)
            .content(
                LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                    .child(TextView::new("The Entry "))
                    .child(TextView::new(name.as_str())
                        .style(reverse_style))
                        .child(TextView::new(" will be "))
                        .child(TextView::new("DELETED")
                            .style(danger_style))    
                        .child(TextView::new(". Do you want to proceed?"))
                )
            )
            .button("Cancel", |s| { s.pop_layer(); })            
            .button("OK", move |s| {
                state_temp_del.lock().unwrap().store.delete(&name);
                visualize_if_modified(s, state_temp_del.clone());
                redraw_tui(s, state_temp_del.clone());
                s.pop_layer();
                show_message(s, "Entry deleted successfully. The first remaning element has been selected\nYou may need to scroll to it manually."); 
            });
            
            s.add_layer(res); 
        },
        None => {
            show_message(s, "Unable to determine selected entry"); 
            return; 
        }
    } 
}
