use image;
use num;
type Point = (f64, f64);

// The main abstraction of the library, a struct is Drawable if it can map a 2D coordinate to a pixel value
// i.e Point -> RGB
pub trait Drawable {
    fn draw_point(self, coordinate: Point) -> image::Rgb<u8>;
}

// A trait for defining colourmaps, useful for rendering escape-time fractals
// The range of value should be bounded 0-1.
pub trait ColourMap {
    fn colour_map(self, value: f64) -> image::Rgb<u8>;
}

#[derive(Copy, Clone, Debug)]
pub enum FractalFunction {
    Mandlebrot {
        iterations: u32,
        threshold: f64,
        colourmap: ColourMaps,
    },
    BurningShip {
        iterations: u32,
        threshold: f64,
        colourmap: ColourMaps,
    },
    Newton {
        iterations: u32,
    },
}

#[derive(Copy, Clone, Debug)]
pub enum ColourMaps {
    // Toms Rainbow ColourMap - taken from https://tomkwok.com/posts/color-maps/
    Trcm,
    GrayScale,
}

// Implementing Drawable for some example FractalFunctions
impl Drawable for FractalFunction {
    fn draw_point(self, coordinate: Point) -> image::Rgb<u8> {
        let (x, y) = coordinate;
        match self {
            FractalFunction::Mandlebrot {
                iterations,
                threshold,
                colourmap,
            } => {
                let c = num::complex::Complex::new(x, y);
                let mut z = num::complex::Complex::new(0.0, 0.0);
                let mut iteration: u32 = 0;
                while iteration < iterations && z.norm() < threshold {
                    z = (z * z) + c;
                    iteration += 1;
                }
                let iterations = iterations as f64;
                let iteration = iteration as f64;
                let value = (iterations - iteration) / iterations;
                colourmap.colour_map(value)
            }
            FractalFunction::BurningShip {
                iterations,
                threshold,
                colourmap,
            } => {
                let c = num::complex::Complex::new(x, y);
                let mut z = num::complex::Complex::new(0.0, 0.0);
                let mut iteration: u32 = 0;
                while iteration < iterations && z.norm() < threshold {
                    z = num::complex::Complex::new((z.re as f64).abs(), (z.im as f64).abs());
                    z = z * z + c;
                    iteration += 1;
                }
                let iterations = iterations as f64;
                let iteration = iteration as f64;
                let value = (iterations - iteration) / iterations;
                colourmap.colour_map(value)
            }
            FractalFunction::Newton { iterations } => {
                let mut z = num::complex::Complex::new(x, y);
                let root_1 = num::complex::Complex::new(1.0, 0.0);
                let root_2 = num::complex::Complex::new(-0.5, 0.5 * (3.0_f64.sqrt()));
                let root_3 = num::complex::Complex::new(-0.5, -0.5 * (3.0_f64.sqrt()));
                let roots = [root_1, root_2, root_3];
                let tolerance: f64 = 0.25;
                for _ in 0..iterations {
                    z = (z.powf(3.0) - root_1) / (3.0 * z.powf(2.0));
                    for n in 0..2 {
                        if (z - roots[n]).norm() < tolerance {
                            match n {
                                0 => return image::Rgb::<u8>::from([173, 133, 186]),
                                1 => return image::Rgb::<u8>::from([116, 161, 142]),
                                _ => return image::Rgb::<u8>::from([114, 76, 52]),
                            }
                        }
                    }
                }
                image::Rgb::<u8>::from([10, 10, 15])
            }
        }
    }
}

impl ColourMap for ColourMaps {
    fn colour_map(self, value: f64) -> image::Rgb<u8> {
        assert!(value <= 1.0 && value >= 0.0);
        match self {
            ColourMaps::Trcm => {
                let r = (255.0 * (-0.5 * (value - 0.75).powf(2.0) / 0.0625).exp()) as u8;
                let g = (255.0 * (-0.5 * (value - 0.5).powf(2.0) / 0.0625).exp()) as u8;
                let b = (255.0 * (-0.5 * (value - 0.2).powf(2.0) / 0.0625).exp()) as u8;
                image::Rgb::<u8>::from([r, g, b])
            }
            ColourMaps::GrayScale => {
                let v = (255.0 * value) as u8;
                image::Rgb::<u8>::from([v, v, v])
            }
        }
    }
}

// Takes a Drawable F and draws it over a given range at a given resolution
pub fn draw_function<F>(
    function_range: [f64; 4],
    image_size: (u32, u32),
    function: F,
) -> image::DynamicImage
where
    F: Drawable + Copy,
{
    let [x1, x2, y1, y2] = function_range;
    let (width, height) = image_size;
    let rgb = image::ImageBuffer::from_fn(width, height, |x, y| {
        let x = ((x as f64) / width as f64) * (x2 - x1) + x1;
        let y = ((y as f64) / height as f64) * (y2 - y1) + y1;
        let pixel = function.draw_point((x, y));
        pixel
    });

    image::DynamicImage::ImageRgb8(rgb)
}

// Helper functions for rendering Mandlebrot zooms
pub fn mandlebrot_frame(
    centre: Point,
    zoom: f64,
    threshold: f64,
    iterations: u32,
    resolution: (u32, u32),
) -> image::DynamicImage {
    let bounds: [f64; 4] = [-2.0, 0.47, -1.12, 1.12];
    let radius: f64 = 1.235 * (1.0 / zoom);
    let (x, y) = centre;
    assert!(zoom >= 1.0);
    assert!(x >= bounds[0] && x <= bounds[1] && y >= bounds[2] && y <= bounds[3]);
    let mut function_range = [x - radius, x + radius, y - radius, y + radius];
    if function_range[0] < bounds[0] {
        let dist = bounds[0] - function_range[0];
        function_range[0] += dist;
        function_range[1] += dist;
    } else if function_range[1] > bounds[1] {
        let dist = bounds[1] - function_range[1];
        function_range[0] += dist;
        function_range[1] += dist;
    }
    if function_range[2] < bounds[2] {
        let dist = bounds[2] - function_range[2];
        function_range[2] += dist;
        function_range[3] += dist;
    } else if function_range[3] > bounds[3] {
        let dist = bounds[3] - function_range[3];
        function_range[2] += dist;
        function_range[3] += dist;
    }
    let frame = draw_function(
        function_range,
        resolution,
        FractalFunction::Mandlebrot {
            iterations,
            threshold,
            colourmap: ColourMaps::Trcm,
        },
    );
    frame
}

pub fn make_zoom_frames(
    centre: (f64, f64),
    max_iterations: u32,
    max_zoom: f64,
    max_frames: u32,
    resolution: (u32, u32),
) {
    for iteration in 1..max_iterations {
        let img = mandlebrot_frame(centre, 1.0, 4.0, iteration, resolution);
        img.save(format!("./mandlebrot_frames/frame_{}.png", iteration))
            .unwrap();
        println!("frame {}/{}", iteration, max_iterations + max_frames);
    }
    for frame in 0..max_frames {
        let frame_n = frame + max_iterations;
        let zoom = ((frame as f64) / (max_frames as f64) * max_zoom.ln()).exp();
        let img = mandlebrot_frame(
            centre,
            zoom,
            4.0,
            max_iterations + (zoom as u32),
            resolution,
        );
        img.save(format!("./mandlebrot_frames/frame_{}.png", frame_n))
            .unwrap();
        println!("frame {}/{}", frame_n, max_iterations + max_frames);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn render_mandlebrot() -> Result<(), image::ImageError> {
        let img = draw_function(
            [-2.0, 0.47, -1.12, 1.12],
            (2048, 2048),
            FractalFunction::Mandlebrot {
                iterations: 100,
                threshold: 4.0,
                colourmap: ColourMaps::GrayScale,
            },
        );
        img.save("./outputs/mandlebrot_test.png")
    }
    #[test]
    fn render_burning_ship() -> Result<(), image::ImageError> {
        let img = draw_function(
            [-1.8, -1.7, -0.09, 0.01],
            (2048, 2048),
            FractalFunction::BurningShip {
                iterations: 200,
                threshold: 4.0,
                colourmap: ColourMaps::Trcm,
            },
        );
        img.save("./outputs/burning_ship_test.png")
    }
}
