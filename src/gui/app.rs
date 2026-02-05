use eframe::egui;
use crate::core::app::AppContext;

pub struct AirLinkApp {
    context: Option<AppContext>,
    camera_texture: Option<egui::TextureHandle>,
}

impl AirLinkApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            context: None,
            camera_texture: None,
        }
    }

    pub fn with_context(mut self, ctx: AppContext) -> Self {
        self.context = Some(ctx);
        self
    }
}

impl eframe::App for AirLinkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Process Frame & AI
        if let Some(app_ctx) = &mut self.context {
            if let Ok(frame) = app_ctx.camera.capture_frame() {
                // Run AI Detection (Pinch logic is inside detector/app)
                let _ = app_ctx.detector.detect(&frame);

                // Convert image::DynamicImage to egui::ColorImage
                let rgb_img = frame.to_rgb8();
                let size = [rgb_img.width() as usize, rgb_img.height() as usize];
                let pixels = rgb_img.as_flat_samples();
                let color_image = egui::ColorImage::from_rgb(size, pixels.as_slice());

                // Load/Update texture
                self.camera_texture = Some(ctx.load_texture(
                    "camera_stream",
                    color_image,
                    egui::TextureOptions::LINEAR,
                ));
            }
        }

        // 2. Draw UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("📡 Air-Link Control Center");
                ui.add_space(8.0);

                if let Some(texture) = &self.camera_texture {
                    // Display the video feed
                    ui.image((texture.id(), ui.available_size()));
                } else {
                    ui.add_space(100.0);
                    ui.label("Searching for camera stream...");
                    ui.spinner();
                }

                ui.add_space(8.0);
                ui.separator();
                ui.label("Status: Running on Wayland/Hyprland");
                ui.add_space(4.0);
                ui.weak("Gesture: Pinch thumb + index to Left Click");
            });
        });

        // Request continuous repaint for video stream
        ctx.request_repaint();
    }
}