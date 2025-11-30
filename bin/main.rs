use opencv::highgui;
use opencv::imgcodecs;

use vslam_rs::utils::Image;

const SHOW_IMAGE: bool = false;


fn main() -> opencv::Result<()> {
    let image_path = "image/automobile_test.jpg";
    let image = Image::load_from_file(image_path, imgcodecs::IMREAD_COLOR)?;
    let gray_image = image.convert_to_gray()?;
    let resized_image = image.resize_image(image.get_width(), image.get_height())?;

    if SHOW_IMAGE {
        // Display the image
        highgui::named_window("Gray image with reduced size", highgui::WINDOW_AUTOSIZE)
            .expect("Failed to create window");
        highgui::imshow("Display Image", &resized_image).expect("Failed to display image");
        highgui::wait_key(0).expect("Failed to wait for key press");
    }



    // Save the resized image
    let output_path = "image/automobile_test_resized.jpg";
    let gray_output_path = "image/automobile_test_gray.jpg";
    let _ = image.write_image(output_path, &resized_image);
    let _ = image.write_image(gray_output_path, &gray_image);
    Ok(())
}
