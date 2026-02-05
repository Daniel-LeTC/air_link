use crate::core::{CameraManager, vision::HandDetector, MouseManager};
use crate::core::logic::SmoothFilter;
use crate::core::vision::HandResult;
use crate::Result;
use tracing::{info, error};
use std::time::Instant;
use image::DynamicImage;

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
    pub last_hand: Option<HandResult>,
}

impl AppContext {
    pub fn new(camera_id: u32, sensitivity: f32, screen_config: ScreenConfig) -> Result<Self> {
        info!("Initializing Air-Link Core...");
        let mut camera = CameraManager::new(camera_id)?;
        camera.start()?;
        
        let detector = HandDetector::new("assets/hand_landmark.onnx")?;
        let mouse = MouseManager::new()?;
        let filter = SmoothFilter::new(0.2);
        
        Ok(Self { 
            camera, 
            detector, 
            mouse, 
            filter,
            sensitivity, 
            screen_config,
            is_clicking: false,
            last_hand: None,
        })
    }

    pub fn process_current_frame(&mut self, frame: &DynamicImage) -> Result<Option<HandResult>> {
        match self.detector.detect(frame) {
            Ok(Some(hand)) => {
                let (x, y) = hand.index_tip;
                let (tx, ty) = hand.thumb_tip;
                
                // Get Thumb Base (Landmark 2) for Thumbs-Up logic
                let thumb_base_y = hand.all_landmarks[2].1;
                
                // New Gesture: Thumbs Up to Click
                // ty < thumb_base_y means the thumb tip is higher than its base
                let is_thumb_up = ty < (thumb_base_y - 0.04); 

                // 1. Mouse Movement (Only move if NOT clicking to lock position for precision)
                if !is_thumb_up {
                    let (smooth_x, smooth_y) = self.filter.filter(x, y);
                    
                    let min_z = 0.2; 
                    let max_z = 0.8;
                    let z_size = max_z - min_z;
                    let nx = ((smooth_x - min_z) / z_size).clamp(0.0, 1.0);
                    let ny = ((smooth_y - min_z) / z_size).clamp(0.0, 1.0);

                    let mirrored_x = 1.0 - nx; 
                    let screen_x = self.screen_config.x_offset + (mirrored_x * self.screen_config.width as f32) as i32;
                    let screen_y = self.screen_config.y_offset + (ny * self.screen_config.height as f32) as i32;

                    let boost = self.sensitivity * 1.5;
                    let adj_x = (screen_x as f32 * boost) as i32;
                    let adj_y = (screen_y as f32 * boost) as i32;

                    if let Err(_) = self.mouse.move_to(adj_x, adj_y) {}
                }

                // 2. Click Handling
                if is_thumb_up {
                    if !self.is_clicking {
                        println!("👍 THUMBS UP: LEFT CLICK!");
                        let _ = self.mouse.click_left();
                        self.is_clicking = true;
                    }
                } else {
                    self.is_clicking = false;
                }

                self.last_hand = Some(hand.clone());
                Ok(self.last_hand.clone())
            },
            Ok(None) => {
                self.last_hand = None;
                Ok(None)
            },
            Err(e) => Err(e),
        }
    }

    pub fn run_loop(&mut self) -> Result<()> {
        println!("🚀 AIR-LINK RUNNING (Gesture: Thumb-Up to Click)");
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
                    let _ = self.process_current_frame(&frame)?;
                }
                Err(e) => {
                    error!("Camera Error: {}", e);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    }
}