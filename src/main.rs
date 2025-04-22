use opencv::core::Mat;
use opencv::highgui;
use opencv::imgcodecs;
// use opencv::core::AlgorithmHint;
use opencv::prelude::*;

const SHOW_IMAGE: bool = false;

fn main() {
    let image_path = "image/automobile_test.jpg";
    let image =
        imgcodecs::imread(image_path, imgcodecs::IMREAD_COLOR).expect("Failed to read image");

    if image.empty() {
        println!("Image not found or unable to open");
        panic!("Image not found or unable to open");
    }

    let mut gray_image = Mat::default();
    opencv::imgproc::cvt_color(&image, &mut gray_image, opencv::imgproc::COLOR_BGR2GRAY, 0)
        .expect("Failed to convert image to grayscale");

    let mut resized_image = Mat::default();
    opencv::imgproc::resize(
        &image,
        &mut resized_image,
        opencv::core::Size::new((image.cols() / 2).into(), (image.rows() / 2).into()),
        0.0,
        0.0,
        opencv::imgproc::INTER_LINEAR,
    )
    .expect("Failed to resize image");

    if SHOW_IMAGE {
        // Display the image
        highgui::named_window("Gray image with reduced size", highgui::WINDOW_AUTOSIZE)
            .expect("Failed to create window");
        highgui::imshow("Display Image", &resized_image).expect("Failed to display image");
        highgui::wait_key(0).expect("Failed to wait for key press");
    }

    // Save the resized image
    let output_path = "image/automobile_test_resized.jpg";
    imgcodecs::imwrite(
        output_path,
        &resized_image,
        &opencv::core::Vector::<i32>::new(),
    )
    .expect("Failed to save image");
    println!("Resized image saved to: {}", output_path);
}
