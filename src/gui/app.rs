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
        let mut hand_detected = false;
        let mut current_score = 0.0;

        // 1. Logic Processing
        if let Some(app_ctx) = &mut self.context {
            if let Ok(frame) = app_ctx.camera.capture_frame() {
                if let Ok(Some(hand)) = app_ctx.process_current_frame(&frame) {
                    hand_detected = true;
                    current_score = hand.score;
                }

                let rgb_img = frame.to_rgb8();
                let size = [rgb_img.width() as usize, rgb_img.height() as usize];
                let pixels = rgb_img.as_flat_samples();
                let color_image = egui::ColorImage::from_rgb(size, pixels.as_slice());

                self.camera_texture = Some(ctx.load_texture(
                    "camera_stream",
                    color_image,
                    egui::TextureOptions::LINEAR,
                ));
            }
        }

        // 2. Rendering
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("📡 Air-Link Control Center");
                
                // Status Overlay
                ui.horizontal(|ui| {
                    let status_color = if hand_detected { egui::Color32::GREEN } else { egui::Color32::RED };
                    ui.label("Hand: ");
                    ui.colored_label(status_color, if hand_detected { "DETECTED" } else { "NOT FOUND" });
                    if hand_detected {
                        ui.label(format!("({:.2}%)", current_score * 100.0));
                    }
                });

                ui.add_space(4.0);

                if let Some(texture) = &self.camera_texture {
                    let img_size = ui.available_size();
                    let (rect, _response) = ui.allocate_exact_size(img_size, egui::Sense::hover());
                    
                    // Draw Camera
                    ui.painter().image(texture.id(), rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);

                    // Draw Landmarks Overlay
                    if let Some(app_ctx) = &self.context {
                        if let Some(hand) = &app_ctx.last_hand {
                            let painter = ui.painter_at(rect);
                            let dot_color = if app_ctx.is_clicking { egui::Color32::GREEN } else { egui::Color32::YELLOW };

                            for landmark in &hand.all_landmarks {
                                let x = rect.min.x + landmark.0 * rect.width();
                                let y = rect.min.y + landmark.1 * rect.height();
                                painter.circle_filled(egui::pos2(x, y), 4.0, dot_color);
                            }
                        }
                    }
                } else {
                    ui.add_space(100.0);
                    ui.label("Waiting for camera...");
                    ui.spinner();
                }
            });
        });

        ctx.request_repaint();
    }
}
