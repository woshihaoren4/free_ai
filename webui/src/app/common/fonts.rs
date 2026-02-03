use wd_tools::PFArc;

pub fn set_fonts(ctx : &egui::Context){
    let mut fonts = egui::FontDefinitions::default();
    let font_data = include_bytes!("../../../assets/Alibaba-PuHuiTi-Medium.ttf");
    fonts.font_data.insert(
        "Alibaba-PuHuiTi-Medium".to_owned(),
        egui::FontData::from_static(font_data).arc(),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "Alibaba-PuHuiTi-Medium".to_owned());
    ctx.set_fonts(fonts);
}