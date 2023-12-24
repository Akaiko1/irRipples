use rand::Rng;
use nannou::prelude::*;
use noise::NoiseFn;

const COLORS: [rgb::Srgb<u8>; 5] = [NAVAJOWHITE, LIGHTGOLDENRODYELLOW, ANTIQUEWHITE, BLANCHEDALMOND, WHITE];

pub fn random_color() -> rgb::Srgb<u8> {
    COLORS[rand::thread_rng().gen_range(0..COLORS.len())]
}

fn lerp(start: Rgb, end: Rgb, percent: f32) -> Rgb {
    rgb(
        start.red + percent * (end.red - start.red),
        start.green + percent * (end.green - start.green),
        start.blue + percent * (end.blue - start.blue),
    )
}

pub fn draw_water_background(draw: &Draw, app: &App, noise: noise::Perlin, time: f32) {
    let win = app.window_rect();

    for y in (win.bottom() as i32)..(win.top() as i32) {
        let depth_factor = (y as f32 - win.bottom()) / win.h();
        let base_color = lerp(rgb(0.0, 0.05, 0.2), rgb(0.0, 0.3, 0.6), depth_factor);

        let noise_value = (0..3).fold(0.0, |acc, i| {
            acc + 0.5_f32.powi(i) * noise.get([
                time as f64 * 0.1 + 0.5_f64.powi(i) * y as f64 * 0.02, 
                0.5_f64.powi(i) * y as f64 * 0.02
            ]) as f32
        });

        let light_factor = 1.0 + noise_value * 0.1;
        let color = rgb(
            (base_color.red * light_factor).min(1.0),
            (base_color.green * light_factor).min(1.0),
            (base_color.blue * light_factor).min(1.0),
        );

        draw.line()
            .start(pt2(win.left(), y as f32))
            .end(pt2(win.right(), y as f32))
            .color(color);
    }
}