use nannou::prelude::*;
use rand::Rng;

static AMOUNT: i16 = 5;

struct Ripple {
    center: Point2,
    copy_color: Vec<rgb::Srgb<u8>>,
    radius: f32,
    copies: i16
}

struct Model {
    ripples: Vec<Ripple>
}

impl Ripple {
    fn grow(&mut self) {
        self.radius += 1.;
    }
}

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model {
        ripples: vec![]
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent{ id: _id, simple: Some(MousePressed(_)) } = event
    {
        model.ripples.push(
            Ripple {
            center: app.mouse.position(),
            copy_color: vec![WHITE],
            radius: 10.,
            copies: 1
        });
    }

    match event {
        Event::Update(_) => {
            for ripple in model.ripples.iter_mut() {
                ripple.grow();
                if ripple.copies < AMOUNT && ripple.radius as i16 > ripple.copies * 7  {
                    ripple.copies += 1;
                    ripple.copy_color.push(vec![NAVAJOWHITE, LIGHTGOLDENRODYELLOW, ANTIQUEWHITE, BLANCHEDALMOND]
                        [rand::thread_rng().gen_range(0..4)])
                }
            }

            model.ripples.retain(|x| x.radius < 200.)
        }
        _ => {}
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLUE);

    for ripple in model.ripples.iter() {
        for i in 0..ripple.copies {
            draw.ellipse()
            .xy(ripple.center)
            .w_h(ripple.radius - i as f32 * 7., ripple.radius - i as f32 * 7.)
            .no_fill()
            .stroke(ripple.copy_color[i as usize])
            .stroke_weight(3.0);
        }
    }
    
    draw.to_frame(app, &frame).unwrap();
}