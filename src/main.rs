use nannou::prelude::*;
use rand::Rng;
use noise::NoiseFn;

const AMOUNT: i16 = 5;
const MAX_RADIUS: f32 = 200.;
const RADIUS_INCREMENT: f32 = 1.;
const INITIAL_RADIUS: f32 = 10.;
const COLOR_CHANGE_FACTOR: i16 = 7;
const COLORS: [rgb::Srgb<u8>; 5] = [NAVAJOWHITE, LIGHTGOLDENRODYELLOW, ANTIQUEWHITE, BLANCHEDALMOND, WHITE];

#[derive(Clone)]
struct Ripple {
    center: Point2,
    color_sequence: Vec<rgb::Srgb<u8>>,
    radius: f32,
    copies: i16,
}

struct Model {
    ripples: Vec<Ripple>,
    noise: noise::Perlin,
}

impl Ripple {
    fn grow(&mut self) {
        self.radius += RADIUS_INCREMENT;
    }

    fn copy(&mut self) {
        if self.copies < AMOUNT && self.radius as i16 > self.copies * COLOR_CHANGE_FACTOR {
            self.copies += 1;
            self.color_sequence.push(random_color());
        }
    }
}

fn random_color() -> rgb::Srgb<u8> {
    COLORS[rand::thread_rng().gen_range(0..COLORS.len())]
}

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model { 
        ripples: vec![],
        noise: noise::Perlin::new(),
    }
}


fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::Update(_) => {
            for ripple in model.ripples.iter_mut() {
                ripple.grow();
                ripple.copy();
            }
            model.ripples.retain(|ripple| ripple.radius < MAX_RADIUS);
        },
        Event::WindowEvent { simple: Some(MousePressed(_)), .. } => {
            model.ripples.push(Ripple {
                center: app.mouse.position(),
                color_sequence: vec![random_color()],
                radius: INITIAL_RADIUS,
                copies: 1,
            });
        },
        _ => {}
    }
}

fn lerp(start: Rgb, end: Rgb, percent: f32) -> Rgb {
    rgb(
        start.red + percent * (end.red - start.red),
        start.green + percent * (end.green - start.green),
        start.blue + percent * (end.blue - start.blue),
    )
}

fn draw_water_background(draw: &Draw, app: &App, model: &Model, time: f32) {
    let noise = &model.noise;
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

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // Draw the water background
    draw_water_background(&draw, app, model, app.time);

    for ripple in &model.ripples {
        for i in 0..ripple.copies {
            draw.ellipse()
                .xy(ripple.center)
                .w_h(ripple.radius - i as f32 * COLOR_CHANGE_FACTOR as f32, ripple.radius - i as f32 * COLOR_CHANGE_FACTOR as f32)
                .no_fill()
                .stroke(ripple.color_sequence[i as usize])
                .stroke_weight(3.0);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
