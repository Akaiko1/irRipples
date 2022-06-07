use nannou::prelude::*;

#[derive(Copy, Clone)]
struct Ripple {
    center: Point2,
    radius: f32
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
            radius: 5.
        });
    }

    match event {
        Event::Update(_) => {
            let mut ripples = vec![];
            for ripple in model.ripples.iter_mut() {
                ripple.grow();
                if ripple.radius < 80. { ripples.push(*ripple)}
            }

            model.ripples = ripples;
        }
        _ => {}
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLUE);

    for ripple in model.ripples.iter() {
        draw.ellipse()
            .xy(ripple.center)
            .w_h(ripple.radius, ripple.radius)
            .no_fill()
            .stroke(WHITE)
            .stroke_weight(3.0);
    }
    
    draw.to_frame(app, &frame).unwrap();
}