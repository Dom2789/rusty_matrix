fn main() {
    println!("Hello, world!");
use rpi_led_matrix::{LedMatrix, LedColor,LedMatrixOptions};
let mut options = LedMatrixOptions::new();
options.set_hardware_mapping("adafruit-hat");
options.set_rows(32);
options.set_cols(64);
let matrix = LedMatrix::new(Some(options), None).unwrap();
let mut canvas = matrix.offscreen_canvas();
for red in (0..255).step_by(16) {
for green in (0..255).step_by(16) {
for blue in (0..255).step_by(16) {
    canvas.fill(&LedColor { red, green, blue });
    canvas = matrix.swap(canvas);
}
}
}
}
