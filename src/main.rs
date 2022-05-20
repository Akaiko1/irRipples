use nannou::prelude::*;

// struct Ripple {
//
// }

struct Model {
    ripples: Vec<Point2>
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
        model.ripples.push( app.mouse.position());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for circle in model.ripples.iter() {
        draw.ellipse()
            .xy(*circle)
            .no_fill()
            .stroke(SALMON)
            .stroke_weight(3.0);
    }

    draw.to_frame(app, &frame).unwrap();
}