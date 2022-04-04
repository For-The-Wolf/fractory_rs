# fractory_rs
A rust library for rendering fractals

## Use
Create visualisations of any fractal function by implementing the `Drawable` trait and calling `fractory_rs::draw_function()` on the custom fractal definition. 

Or, use some of the included `FractalFunction`s and `ColourMap`s to quickly create visualisations of some popular fractals such as Mandlebrot and Burning Ship.

## Examples

Recreate the images below by calling the associated cargo example:

### Mandlebrot

Run `cargo run --example mandlebrot`

![mandlebrot example](/outputs/mandlebrot.png)

Run `cargo run --example mandlebrot_zoomed`

![mandlebrot zoomed example](/outputs/mandlebrot_zoomed.png)

### Burning ship

Run `cargo run --example burning_ship`

![burning ship example](/outputs/burning_ship.png)

### Newton's fractal

Run `cargo run --example newton`

![Newton fractal example](/outputs/newtwon.png)
