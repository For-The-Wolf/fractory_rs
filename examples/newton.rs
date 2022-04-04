extern crate fractory_rs;
extern crate image;

fn main() -> Result<(), image::ImageError> {
    // A Misiurewicz point
    let function_range = [-2.0, 2.0, -2.0, 2.0];
    let resolution = (2048, 2048);
    let iterations = 15;
    let function = fractory_rs::FractalFunction::Newton { iterations };
    let img = fractory_rs::draw_function(function_range, resolution, function);
    img.save("./outputs/newtwon.png")
}
