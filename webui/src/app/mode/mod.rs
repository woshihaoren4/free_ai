pub mod chat_mode;

use crate::app::common::*;
use crate::app::mode::chat_mode::ChatModeMainWindow;

pub trait ModeHandleEvent{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame,cfg:&mut Config);
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct FreeAiApp {
    cfg : Config,
}

impl Default for FreeAiApp {
    fn default() -> Self {
        let cfg = Config::default();
        Self { cfg }
    }
}
impl FreeAiApp {
    pub fn save_key()->&'static str{
        "free_ai_app_store"
    }
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            let cfg = eframe::get_value::<Config>(storage, FreeAiApp::save_key()).unwrap_or_else(|| {
                let state = Config::default();
                state
            });
            return FreeAiApp { cfg };
        }
        let app = FreeAiApp::default();
        app
    }
    pub fn get_mode_main_window(mode:Mode) -> impl ModeHandleEvent{
        match mode {
            Mode::Chat => {
                ChatModeMainWindow{}
            }
            Mode::Notify => {
                ChatModeMainWindow{}
            }
        }
    }
    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut main_window = Self::get_mode_main_window(self.cfg.mode);
        main_window.update(ctx, frame, &mut self.cfg);
    }
}

impl eframe::App for FreeAiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update(ctx, frame);
    }
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, FreeAiApp::save_key(), &self.cfg);
    }
}



