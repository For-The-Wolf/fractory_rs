extern crate fractory_rs;
extern crate image;

fn main() -> Result<(), image::ImageError> {
    let bounds = [-2.0, 0.47, -1.12, 1.12];
    let resolution = (2048, 2048);
    let iterations = 200;
    let threshold = 4.0;
    let colourmap = fractory_rs::ColourMaps::GrayScale;

    let img = fractory_rs::draw_function(
        bounds,
        resolution,
        fractory_rs::FractalFunction::Mandlebrot {
            iterations,
            threshold,
            colourmap,
        },
    );
    img.save("./outputs/mandlebrot.png")
}
