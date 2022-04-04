extern crate fractory_rs;
extern crate image;

fn main() -> Result<(), image::ImageError> {
    // A Misiurewicz point
    let centre_coordinate = (-0.77568377, 0.13646737);

    let zoom = 1200;
    let resolution = (2048, 2048);
    let iterations = 500;
    let threshold = 4.0;

    let img = fractory_rs::mandlebrot_frame(
        centre_coordinate,
        zoom as f64,
        threshold,
        iterations,
        resolution,
    );
    img.save("./outputs/mandlebrot_zoomed.png")
}
