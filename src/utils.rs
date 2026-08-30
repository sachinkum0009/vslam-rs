use opencv::core::{Mat, MatTrait, MatTraitConst};
use opencv::{Error, imgcodecs};

pub struct Image {
    img: Mat,
}

impl Image {
    // pub fn new(name: String, type_of_data: i32) -> Self {
    //     Image { name, type_of_data }
    // }
    // opencv::opencv::hub::imgcodecs
    pub fn load_from_file(img_path: &str, flags: i32) -> opencv::Result<Self> {
        let img = imgcodecs::imread(&img_path, flags)?;
        if img.empty() {
            Err(opencv::Error::new(
                opencv::core::StsError,
                format!("Failed to load image from path: {}", img_path),
            ))?;
        }
        Ok(Image { img })
    }
    pub fn convert_to_gray(&self) -> opencv::Result<Mat> {
        let mut gray_image = Mat::default();
        opencv::imgproc::cvt_color(
            &self.img,
            &mut gray_image,
            opencv::imgproc::COLOR_BGR2GRAY,
            0,
        )?;
        Ok(gray_image)
    }

    pub fn resize_image(&self, width: i32, height: i32) -> opencv::Result<Mat> {
        let mut resized_image = Mat::default();
        opencv::imgproc::resize(
            &self.img,
            &mut resized_image,
            opencv::core::Size::new(width, height),
            0.0,
            0.0,
            opencv::imgproc::INTER_LINEAR,
        )?;
        Ok(resized_image)
    }

    pub fn get_width(&self) -> i32 {
        self.img.cols()
    }

    pub fn get_height(&self) -> i32 {
        self.img.rows()
    }

    pub fn write_image(&self, output_path: &str, img: &Mat) -> opencv::Result<()> {
        imgcodecs::imwrite(output_path, img, &opencv::core::Vector::<i32>::new())?;
        Ok(())
    }
}

// Camera Intrinsics
pub struct CameraIntrisic {
    pub fx: f32,
    pub fy: f32,
    pub cx: f32,
    pub cy: f32,
    pub s: f32,
}

impl Default for CameraIntrisic {
    fn default() -> Self {
        Self {
            fx: 0.0,
            fy: 0.0,
            cx: 0.0,
            cy: 0.0,
            s: 0.0,
        }
    }
}

impl CameraIntrisic {
    pub fn new(fx: f32, fy: f32, cx: f32, cy: f32, s: f32) -> Self {
        CameraIntrisic { fx, fy, cx, cy, s }
    }
    /// Reads the Camera Intrinsic Parameters
    /// from `yaml` file.
    ///
    /// # Arguments
    /// - file_path: String
    pub fn read_from_yaml_file(file_path: &str) -> Result<Self, Error> {
        Ok(CameraIntrisic::default())
    }
}
