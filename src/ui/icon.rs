use eframe::egui::IconData;


pub fn get_icon() -> IconData {
    let icon_image = include_bytes!("../../icon.png");
    let img = match image::load_from_memory(icon_image) {
        Ok(img) => img,
        Err(_) => {
            return IconData::default();
        },
    };

    let img_buf = img.to_rgba8();
    let (w, h) = img_buf.dimensions();

    IconData {
        rgba: img_buf.into_raw(),
        width: w,
        height: h,
    }
}