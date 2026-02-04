use crate::core::{CameraManager, vision::HandDetector, MouseManager};
use crate::core::logic::SmoothFilter;
use crate::Result;
use tracing::{info, error};
use std::time::Instant;

pub struct ScreenConfig {
    pub width: i32,
    pub height: i32,
    pub x_offset: i32,
    pub y_offset: i32,
}

pub struct AppContext {
    pub camera: CameraManager,
    pub detector: HandDetector,
    pub mouse: MouseManager,
    pub filter: SmoothFilter,
    pub sensitivity: f32,
    pub screen_config: ScreenConfig,
    pub is_clicking: bool,
}

impl AppContext {
    pub fn new(camera_id: u32, sensitivity: f32, screen_config: ScreenConfig) -> Result<Self> {
        info!("Initializing Air-Link Core...");
        let mut camera = CameraManager::new(camera_id)?;
        camera.start()?;
        
        let detector = HandDetector::new("assets/hand_landmark.onnx")?;
        let mouse = MouseManager::new()?;
        let filter = SmoothFilter::new(0.2); // Smooth but responsive
        
        Ok(Self { 
            camera, 
            detector, 
            mouse, 
            filter,
            sensitivity, 
            screen_config,
            is_clicking: false,
        })
    }

    pub fn run_loop(&mut self) -> Result<()> {
        println!("🚀 AIR-LINK TURBO MODE ACTIVE!");
        
        let mut last_time = Instant::now();
        let mut frame_count = 0;

        loop {
            frame_count += 1;
            if last_time.elapsed().as_secs() >= 1 {
                println!("Heartbeat - FPS: {}", frame_count);
                frame_count = 0;
                last_time = Instant::now();
            }

            match self.camera.capture_frame() {
                Ok(frame) => {
                    match self.detector.detect(&frame) {
                        Ok(Some(hand)) => {
                            let (x, y) = hand.index_tip;
                            let (tx, ty) = hand.thumb_tip;

                            // 1. Smooth the raw AI coordinates
                            let (smooth_x, smooth_y) = self.filter.filter(x, y);

                            // 2. ACTIVE ZONE LOGIC (Crop & Zoom)
                            // Use the central 50% of the camera view as the full screen area
                            let min_z = 0.2; 
                            let max_z = 0.8;
                            let z_size = max_z - min_z;

                            let nx = ((smooth_x - min_z) / z_size).clamp(0.0, 1.0);
                            let ny = ((smooth_y - min_z) / z_size).clamp(0.0, 1.0);

                            // 3. Mirror & Scale to Screen
                            let mirrored_x = 1.0 - nx; 
                            
                            let screen_x = self.screen_config.x_offset + 
                                           (mirrored_x * self.screen_config.width as f32) as i32;
                            
                            let screen_y = self.screen_config.y_offset + 
                                           (ny * self.screen_config.height as f32) as i32;

                            // 4. Move Mouse with Sensitivity Boost
                            // We multiply the final coordinates to amplify the delta in MouseManager
                            let boost = self.sensitivity * 1.5;
                            let adj_x = (screen_x as f32 * boost) as i32;
                            let adj_y = (screen_y as f32 * boost) as i32;

                            if let Err(e) = self.mouse.move_to(adj_x, adj_y) {
                                println!("❌ MOUSE ERROR: {}", e);
                            }

                            // 5. Gesture: Pinch to Click
                            let dist_sq = (x - tx).powi(2) + (y - ty).powi(2);
                            let threshold_sq = 0.003;

                            if dist_sq < threshold_sq {
                                if !self.is_clicking {
                                    println!("🎯 GESTURE: CLICK!");
                                    let _ = self.mouse.click_left();
                                    self.is_clicking = true;
                                }
                            } else {
                                self.is_clicking = false;
                            }
                        },
                        Ok(None) => {},
                        Err(e) => println!("⚠️ AI Error: {}", e),
                    }
                }
                Err(e) => {
                    println!("🎥 Camera Error: {}", e);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    }
}