use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType, CameraFormat, Resolution, FrameFormat};
use nokhwa::Camera;
use crate::Result;
use image::DynamicImage;

pub struct CameraManager {
    camera: Camera,
}

impl CameraManager {
    pub fn new(index: u32) -> Result<Self> {
        let index = CameraIndex::Index(index);
        
        // Correct way to request 640x480 for performance on legacy CPUs
        let format = CameraFormat::new(
            Resolution::new(640, 480),
            FrameFormat::MJPEG, 
            30
        );
        
        let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Closest(format));

        let camera = Camera::new(index, requested)
            .map_err(|e| crate::AirLinkError::CoreError(format!("Camera init failed: {}", e)))?;

        Ok(Self { camera })
    }

    pub fn start(&mut self) -> Result<()> {
        self.camera.open_stream()
            .map_err(|e| crate::AirLinkError::CoreError(format!("Stream open failed: {}", e)))?;
        Ok(())
    }

    pub fn capture_frame(&mut self) -> Result<DynamicImage> {
        let frame = self.camera.frame()
            .map_err(|e| crate::AirLinkError::CoreError(format!("Capture failed: {}", e)))?;
        
        let decoded = frame.decode_image::<RgbFormat>()
            .map_err(|e| crate::AirLinkError::CoreError(format!("Decode failed: {}", e)))?;

        Ok(DynamicImage::ImageRgb8(decoded))
    }
}