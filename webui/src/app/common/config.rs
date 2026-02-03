use std::collections::HashMap;
use egui::Window;
use serde::{Deserialize, Serialize};

#[derive(Default,Debug,Clone,Deserialize, Serialize)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
}
#[derive(Default,Debug,Clone,Deserialize, Serialize)]
pub struct UiConfig {
    pub window: HashMap<String,WindowConfig>,
}

impl UiConfig {
    pub fn update_window_config<Out>(&mut self,name:&str,handle:impl FnOnce(&mut WindowConfig)->Out)->Out {
        if !self.window.contains_key(name) {
            self.window.insert(name.to_string(), WindowConfig::default());
        }
        if let Some(e) = self.window.get_mut(name) {
            handle(e)
        }else{
            handle(&mut WindowConfig::default())
        }
    }
    pub fn set_window_width(&mut self, name:&str, width:f32) {
        self.update_window_config(name,|x|{
            x.width = width;
        })
    }
    pub fn get_window_width(&mut self, name:&str,default:f32) -> f32 {
        if let Some(e) = self.window.get(name) {
            e.width
        }else{
            self.set_window_width(name,default);
            default
        }
    }
}

#[derive(Default,Debug,Copy,Clone,Deserialize,Serialize)]
pub enum Mode{
    #[default]
    Chat,
    Notify,
}
#[derive(Default,Debug,Clone,Deserialize, Serialize)]
pub struct Config {
    pub window: UiConfig,
    pub mode: Mode
}

