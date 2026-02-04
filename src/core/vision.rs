use ort::session::Session;
use ort::value::Tensor;
use image::{DynamicImage, imageops::FilterType};
use crate::Result;

pub struct HandResult {
    pub index_tip: (f32, f32),
    pub thumb_tip: (f32, f32),
}

pub struct HandDetector {
    session: Session,
}

impl HandDetector {
    pub fn new(model_path: &str) -> Result<Self> {
        let session = Session::builder()?
            .commit_from_file(model_path)
            .map_err(|e| crate::AirLinkError::CoreError(format!("ORT Init Error: {}", e)))?;
        Ok(Self { session })
    }

    pub fn detect(&mut self, img: &DynamicImage) -> Result<Option<HandResult>> {
        let resized = img.resize_exact(224, 224, FilterType::Triangle);
        let rgb_img = resized.to_rgb8();

        let mut pixels = Vec::with_capacity(3 * 224 * 224);
        for c in 0..3 {
            for y in 0..224 {
                for x in 0..224 {
                    let pix = rgb_img.get_pixel(x, y);
                    pixels.push(pix[c] as f32 / 255.0);
                }
            }
        }
        let shape = vec![1, 3, 224, 224];

        let outputs = self.session.run(ort::inputs![Tensor::from_array((shape, pixels))?])?;
        
        // 1. Check Confidence Score (Output 1)
        // If the model is from MediaPipe, the second output is usually the hand presence score.
        if outputs.len() > 1 {
            let (_, score_data) = outputs[1].try_extract_tensor::<f32>()?;
            let score = score_data[0];
            
            // If score is too low, it's just noise (False Positive)
            if score < 0.7 {
                return Ok(None);
            }
        }

        // 2. Extract Landmarks (Output 0)
        let (_shape, data) = outputs[0].try_extract_tensor::<f32>()
            .map_err(|e| crate::AirLinkError::CoreError(format!("Output extraction failed: {}", e)))?;

        if data.len() < 63 { return Ok(None); }

        let idx_8 = 8 * 3;
        let x8 = data[idx_8];
        let y8 = data[idx_8 + 1];
        
        let (nx, ny) = if x8 > 1.1 || y8 > 1.1 { (x8 / 224.0, y8 / 224.0) } else { (x8, y8) };

        if nx < 0.0 || nx > 1.0 || ny < 0.0 || ny > 1.0 { 
            return Ok(None); 
        }
        
        let idx_4 = 4 * 3;
        let tx = data[idx_4];
        let ty = data[idx_4 + 1];
        let (ntx, nty) = if tx > 1.1 { (tx / 224.0, ty / 224.0) } else { (tx, ty) };

        Ok(Some(HandResult {
            index_tip: (nx, ny),
            thumb_tip: (ntx, nty),
        }))
    }
}