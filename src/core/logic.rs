/// Logic for coordinate transformation and simulation tests.
pub struct CoordinateMapper {
    cam_width: f32,
    cam_height: f32,
    screen_width: f32,
    screen_height: f32,
}

impl CoordinateMapper {
    pub fn new(cam_w: f32, cam_h: f32, screen_w: f32, screen_h: f32) -> Self {
        Self {
            cam_width: cam_w,
            cam_height: cam_h,
            screen_width: screen_w,
            screen_height: screen_h,
        }
    }

    pub fn map_to_screen(&self, cam_x: f32, cam_y: f32) -> (i32, i32) {
        let norm_x = cam_x / self.cam_width;
        let norm_y = cam_y / self.cam_height;
        let mirrored_x = 1.0 - norm_x;
        let screen_x = (mirrored_x * self.screen_width) as i32;
        let screen_y = (norm_y * self.screen_height) as i32;
        (screen_x, screen_y)
    }
}

/// Exponential Moving Average filter for smooth mouse movement.
pub struct SmoothFilter {
    alpha: f32,
    last_x: f32,
    last_y: f32,
    is_initialized: bool,
}

impl SmoothFilter {
    pub fn new(alpha: f32) -> Self {
        Self {
            alpha: alpha.clamp(0.0, 1.0),
            last_x: 0.0,
            last_y: 0.0,
            is_initialized: false,
        }
    }

    pub fn filter(&mut self, x: f32, y: f32) -> (f32, f32) {
        if !self.is_initialized {
            self.last_x = x;
            self.last_y = y;
            self.is_initialized = true;
            return (x, y);
        }

        let smoothed_x = x * self.alpha + self.last_x * (1.0 - self.alpha);
        let smoothed_y = y * self.alpha + self.last_y * (1.0 - self.alpha);

        self.last_x = smoothed_x;
        self.last_y = smoothed_y;

        (smoothed_x, smoothed_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_720p_to_1080p_mapping() {
        let mapper = CoordinateMapper::new(1280.0, 720.0, 1920.0, 1080.0);
        let (sx, sy) = mapper.map_to_screen(0.0, 0.0);
        assert_eq!(sx, 1920);
        assert_eq!(sy, 0);
    }

    #[test]
    fn test_smoothing_filter() {
        let mut filter = SmoothFilter::new(0.5);
        let (x1, y1) = filter.filter(100.0, 100.0);
        assert_eq!(x1, 100.0); // First input initialization

        let (x2, y2) = filter.filter(200.0, 200.0);
        assert_eq!(x2, 150.0); // (200 * 0.5) + (100 * 0.5) = 150
        assert_eq!(y2, 150.0);
    }
}
