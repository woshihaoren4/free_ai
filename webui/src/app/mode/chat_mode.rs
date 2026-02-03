use egui::Sense;
use crate::app::common::Config;

pub struct ChatModeMainWindow{}
impl ChatModeMainWindow {
    pub fn show_version(ui: &mut egui::Ui) {
        ui.label(
            egui::WidgetText::from(
                "This is only an alpha version and there may be major changes in the future！！！",
            )
                .color(egui::Color32::GRAY),
        );
    }
}


impl super::ModeHandleEvent for ChatModeMainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, cfg: &mut Config) {
        // 左侧：目录栏（可拖动中间分割线调整宽度）
        let panel_id = "chat_mode_left_panel";
        egui::SidePanel::left(panel_id)
            .resizable(true)
            .show_separator_line(true)     // 显示分割线
            .min_width(200.0)
            .max_width(500.0)
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.add_space(32.0); // 顶部空一行/留白
                ui.heading("目录");
                ui.separator();

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for item in ["简介", "第一章", "第二章", "附录"] {
                            ui.selectable_label(false, item);
                        }
                    });
            });
        // 右侧：文本区域（自动占据剩余空间）
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("文本");
            ui.separator();

            ui.label(
                egui::RichText::new("这里是右侧区域显示的文本内容。")
                    .size(16.0)
                    .color(egui::Color32::WHITE),
            );
        });
    }
}