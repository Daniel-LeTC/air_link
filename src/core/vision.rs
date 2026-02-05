use ort::session::Session;
use ort::value::Tensor;
use image::{DynamicImage, imageops::FilterType};
use crate::Result;

#[derive(Clone)]
pub struct HandResult {
    pub index_tip: (f32, f32),
    pub thumb_tip: (f32, f32),
    pub all_landmarks: Vec<(f32, f32, f32)>,
    pub score: f32, // Add score for debugging
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
        
        let mut score = 0.0;
        if outputs.len() > 1 {
            let (_, score_data) = outputs[1].try_extract_tensor::<f32>()?;
            score = score_data[0];
            
            // Lowered threshold to 0.5 for better detection in low light
            if score < 0.5 {
                return Ok(None);
            }
        }

        let (_shape, data) = outputs[0].try_extract_tensor::<f32>()
            .map_err(|e| crate::AirLinkError::CoreError(format!("Output extraction failed: {}", e)))?;

        if data.len() < 63 { return Ok(None); }

        let mut all_landmarks = Vec::with_capacity(21);
        for i in 0..21 {
            let idx = i * 3;
            let x = data[idx];
            let y = data[idx + 1];
            let z = data[idx + 2];
            
            let (nx, ny) = if x > 1.1 || y > 1.1 { (x / 224.0, y / 224.0) } else { (x, y) };
            all_landmarks.push((nx, ny, z));
        }

        let index_tip = (all_landmarks[8].0, all_landmarks[8].1);
        let thumb_tip = (all_landmarks[4].0, all_landmarks[4].1);

        Ok(Some(HandResult {
            index_tip,
            thumb_tip,
            all_landmarks,
            score,
        }))
    }
}
