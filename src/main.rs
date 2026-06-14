use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_4X6},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use rpi_led_matrix::{LedMatrix, LedMatrixOptions, LedRuntimeOptions};

const DELAY: std::time::Duration = std::time::Duration::from_secs(20);

fn main() {
    println!("Hello, world!");
    let mut options = LedMatrixOptions::new();
    options.set_hardware_mapping("adafruit-hat");
    options.set_rows(32);
    options.set_cols(64);
    options.set_hardware_pulsing(true);  // hardware PWM (needs default sound off)

    let mut rt_options = LedRuntimeOptions::new();
    rt_options.set_gpio_slowdown(3);     // increase if you see glitches on Pi 4/5

    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.canvas();

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(Rgb888::CSS_SALMON, 1);
    let text_style = MonoTextStyle::new(&FONT_4X6, Rgb888::CSS_DARK_SEA_GREEN);


    //Draw a 1px wide outline around the matrix.
    let (width, height) = canvas.canvas_size();
    Rectangle::with_corners(
        Point::zero(),
        Point::new(width as i32 - 1, height as i32 - 1),
    )
    .into_styled(thin_stroke)
    .draw(&mut canvas)
    .unwrap();

    //Draw a 1px wide outline around the matrix.
    let (width, height) = canvas.canvas_size();
    Rectangle::with_corners(
        Point::new(1, 1),
        Point::new(width as i32 - 2, height as i32 - 2),
    )
        .into_styled(thin_stroke)
        .draw(&mut canvas)
        .unwrap();

    let text = "Hello World!";
    Text::new(text, Point::new(10, 16), text_style)
        .draw(&mut canvas)
        .unwrap();

    std::thread::sleep(DELAY);
}
