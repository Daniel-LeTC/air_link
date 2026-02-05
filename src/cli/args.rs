use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "air-link")]
#[command(about = "High-performance Air Mouse for Legacy Hardware", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the tracking loop (CLI mode)
    Run {
        /// Camera device index
        #[arg(short, long, default_value_t = 0)]
        camera_id: u32,

        /// Cursor sensitivity multiplier
        #[arg(short, long, default_value_t = 1.5)]
        sensitivity: f32,

        /// Tracking algorithm to use
        #[arg(short, long, value_enum, default_value_t = TrackingMode::Ai)]
        mode: TrackingMode,

        // --- Screen Configuration ---
        #[arg(long, default_value_t = 1920)]
        screen_width: i32,
        #[arg(long, default_value_t = 1080)]
        screen_height: i32,
        #[arg(long, default_value_t = 0)]
        screen_x_offset: i32,
        #[arg(long, default_value_t = 0)]
        screen_y_offset: i32,
    },
    /// Start with Graphical User Interface
    Gui {
        /// Camera device index
        #[arg(short, long, default_value_t = 0)]
        camera_id: u32,
    },
    /// Helper to find your camera ID
    ListCameras,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum TrackingMode {
    /// Fastest, tracks a specific color
    Color,
    /// Slower, uses AI model
    Ai,
}
