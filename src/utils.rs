use opencv::core::{Mat, MatTraitConst};
use opencv::imgcodecs;

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
            return Err(opencv::Error::new(
                opencv::core::StsError,
                format!("Failed to load image from path: {}", img_path),
            ));
        }
        Ok(Image { img })
    }
    pub fn convert_to_gray(&self) -> opencv::Result<Mat> {
        let mut gray_image = Mat::default();
        opencv::imgproc::cvt_color(&self.img, &mut gray_image, opencv::imgproc::COLOR_BGR2GRAY, 0)?;
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
