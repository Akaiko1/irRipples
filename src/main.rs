use nannou::prelude::*;
use rand::Rng;

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
    Model { ripples: vec![] }
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

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLUE);

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
