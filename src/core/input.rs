use uinput::event::controller::Controller::Mouse;
use uinput::event::controller::Mouse::Left;
use uinput::event::relative::Relative::Position;
use uinput::event::relative::Position::{X, Y};
use uinput::event::Event::{Controller, Relative};
use crate::Result;
use tracing::info;

pub struct MouseManager {
    device: uinput::Device,
    last_x: i32,
    last_y: i32,
    is_initialized: bool,
}

impl MouseManager {
    pub fn new() -> Result<Self> {
        info!("Initializing Native uinput mouse...");
        
        let device = uinput::default()
            .map_err(|e| crate::AirLinkError::CoreError(format!("uinput default failed: {}", e)))?
            .name("Air-Link Virtual Mouse")
            .map_err(|e| crate::AirLinkError::CoreError(format!("uinput name failed: {}", e)))?
            .event(Controller(Mouse(Left)))
            .map_err(|e| crate::AirLinkError::CoreError(format!("uinput event failed: {}", e)))?
            .event(Relative(Position(X)))
            .map_err(|e| crate::AirLinkError::CoreError(format!("uinput event failed: {}", e)))?
            .event(Relative(Position(Y)))
            .map_err(|e| crate::AirLinkError::CoreError(format!("uinput event failed: {}", e)))?
            .create()
            .map_err(|e| crate::AirLinkError::CoreError(format!("uinput create failed: {}", e)))?;

        Ok(Self { 
            device,
            last_x: 0,
            last_y: 0,
            is_initialized: false,
        })
    }

    pub fn move_to(&mut self, x: i32, y: i32) -> Result<()> {
        if !self.is_initialized {
            self.last_x = x;
            self.last_y = y;
            self.is_initialized = true;
            return Ok(());
        }

        // Calculate Delta (How much the hand moved since last frame)
        let dx = x - self.last_x;
        let dy = y - self.last_y;

        // Only move if there is a significant change to avoid jitter
        if dx.abs() > 0 || dy.abs() > 0 {
            // Send relative movement to OS
            self.device.send(Relative(Position(X)), dx)
                .map_err(|e| crate::AirLinkError::CoreError(format!("uinput send X failed: {}", e)))?;
            self.device.send(Relative(Position(Y)), dy)
                .map_err(|e| crate::AirLinkError::CoreError(format!("uinput send Y failed: {}", e)))?;
            
            self.device.synchronize()
                .map_err(|e| crate::AirLinkError::CoreError(format!("uinput sync failed: {}", e)))?;
        }

        self.last_x = x;
        self.last_y = y;
            
        Ok(())
    }

    pub fn click_left(&mut self) -> Result<()> {
        self.device.send(Controller(Mouse(Left)), 1)
            .map_err(|e| crate::AirLinkError::CoreError(format!("Click down failed: {}", e)))?;
        self.device.synchronize()
            .map_err(|e| crate::AirLinkError::CoreError(format!("Sync failed: {}", e)))?;
            
        self.device.send(Controller(Mouse(Left)), 0)
            .map_err(|e| crate::AirLinkError::CoreError(format!("Click up failed: {}", e)))?;
        self.device.synchronize()
            .map_err(|e| crate::AirLinkError::CoreError(format!("Sync failed: {}", e)))?;
            
        Ok(())
    }
}
