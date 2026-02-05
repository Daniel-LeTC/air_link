use crate::cli::{Cli, Commands};
use crate::Result;
use crate::core::app::{AppContext, ScreenConfig};
use crate::gui::app::AirLinkApp;
use tracing::info;
use nokhwa::query;
use nokhwa::utils::ApiBackend;

pub fn handle_command(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Run { 
            camera_id, sensitivity, mode,
            screen_width, screen_height, screen_x_offset, screen_y_offset
        } => {
            info!("Starting Air Mouse in {:?} mode...", mode);
            
            let screen_config = ScreenConfig {
                width: screen_width,
                height: screen_height,
                x_offset: screen_x_offset,
                y_offset: screen_y_offset,
            };

            let mut app = AppContext::new(camera_id, sensitivity, screen_config)?;
            app.run_loop()?;
        }
        Commands::Gui { camera_id } => {
            info!("Launching GUI Mode...");
            
            let screen_config = ScreenConfig {
                width: 1920,
                height: 1080,
                x_offset: 0,
                y_offset: 0,
            };

            let app_ctx = AppContext::new(camera_id, 1.5, screen_config)?;

            let native_options = eframe::NativeOptions::default();
            eframe::run_native(
                "Air-Link AI Mouse",
                native_options,
                Box::new(|cc| Ok(Box::new(AirLinkApp::new(cc).with_context(app_ctx)))),
            ).map_err(|e| crate::AirLinkError::CoreError(format!("GUI failed: {}", e)))?;
        }
        Commands::ListCameras => {
            info!("Searching for available cameras...");
            
            let devices = query(ApiBackend::Auto)
                .map_err(|e| crate::AirLinkError::CoreError(format!("Query failed: {}", e)))?;

            if devices.is_empty() {
                println!("No cameras detected. Check your connection, bro!");
            } else {
                println!("\n📸 DETECTED CAMERAS:");
                println!("-------------------");
                for dev in devices {
                    println!("=> Name: {} | Index: {:?}", dev.human_name(), dev.index());
                }
                println!("-------------------\n");
            }
        }
    }
    Ok(())
}