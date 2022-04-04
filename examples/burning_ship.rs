extern crate fractory_rs;
extern crate image;

fn main() -> Result<(), image::ImageError> {
    let bounds = [-1.8, -1.7, -0.09, 0.01];
    let resolution = (2048, 2048);
    let iterations = 200;
    let threshold = 4.0;
    let colourmap = fractory_rs::ColourMaps::Trcm;

    let img = fractory_rs::draw_function(
        bounds,
        resolution,
        fractory_rs::FractalFunction::BurningShip {
            iterations,
            threshold,
            colourmap,
        },
    );
    img.save("./outputs/burning_ship.png")
}
