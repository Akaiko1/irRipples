use nannou::prelude::*;
use noise;

mod effects;

// Configuration constants
const AMOUNT: i16 = 5;                  // Maximum number of rings per ripple
const MAX_RADIUS: f32 = 200.0;          // Maximum radius before ripple disappears
const RADIUS_INCREMENT: f32 = 1.5;      // How fast ripples grow
const INITIAL_RADIUS: f32 = 10.0;       // Starting radius for ripples
const COLOR_CHANGE_FACTOR: i16 = 7;     // Spacing between rings
const STROKE_WEIGHT: f32 = 3.0;         // Line thickness for ripples
const FADE_DISTANCE: f32 = 50.0;        // Distance over which ripples fade out

// Special effect configuration
const ENABLE_WOBBLE: bool = true;       // Enable wobble effect on rings
const WOBBLE_AMOUNT: f32 = 0.8;         // How much rings wobble
const WOBBLE_SPEED: f32 = 2.0;          // Speed of wobble animation
const ENABLE_FADE: bool = true;         // Enable opacity fade as ripples grow

#[derive(Clone)]
struct Ripple {
    center: Point2,                     // Center point of the ripple
    color_sequence: Vec<rgb::Srgb<u8>>, // Colors for each concentric ring
    radius: f32,                        // Current radius of the outermost ring
    copies: i16,                        // Number of concentric rings
}

struct Model {
    ripples: Vec<Ripple>,               // List of active ripples
    noise: noise::Perlin,               // Noise generator for effects
    time: f32,                          // Application time
    mouse_down: bool,                   // Tracks if mouse is currently pressed
    last_ripple_time: f32,              // Time when last ripple was created
}

impl Ripple {
    // Create a new ripple at the specified position
    fn new(position: Point2, _time: f32) -> Self {
        let mut colors = Vec::new();
        colors.push(effects::random_color());
        
        Self {
            center: position,
            color_sequence: colors,
            radius: INITIAL_RADIUS,
            copies: 1,
        }
    }

    // Update ripple state (growth and spawning new rings)
    fn update(&mut self, _time: f32) {
        // Grow the ripple
        self.radius += RADIUS_INCREMENT;
        
        // Add new color rings as the ripple grows
        if self.copies < AMOUNT && self.radius as i16 > self.copies * COLOR_CHANGE_FACTOR {
            self.copies += 1;
            self.color_sequence.push(effects::random_color());
        }
    }
    
    // Calculate opacity based on ripple age
    fn opacity(&self, max_radius: f32) -> f32 {
        if ENABLE_FADE {
            // Fade out as the ripple approaches maximum size
            let fade_start = max_radius - FADE_DISTANCE;
            if self.radius > fade_start {
                return 1.0 - (self.radius - fade_start) / FADE_DISTANCE;
            }
        }
        return 1.0;
    }
    
    // Draw the ripple
    fn draw(&self, draw: &Draw, _app: &App, time: f32) {
        let opacity = self.opacity(MAX_RADIUS);
        
        for i in 0..self.copies {
            let ring_radius = self.radius - i as f32 * COLOR_CHANGE_FACTOR as f32;
            let  color = self.color_sequence[i as usize];
            
            // Apply opacity
            let color_with_alpha = rgba(
                color.red as f32 / 255.0, 
                color.green as f32 / 255.0, 
                color.blue as f32 / 255.0, 
                opacity
            );
            
            if ENABLE_WOBBLE {
                // Create a wobbly effect by drawing points around the circle
                let points = (0..=360).step_by(5).map(|deg| {
                    let radian = deg_to_rad(deg as f32);
                    let wobble = WOBBLE_AMOUNT * (time * WOBBLE_SPEED + deg as f32 / 30.0).sin();
                    let wobble_radius = ring_radius * (1.0 + wobble * 0.01);
                    let x = self.center.x + wobble_radius * radian.cos();
                    let y = self.center.y + wobble_radius * radian.sin();
                    (pt2(x, y), color_with_alpha)
                }).collect::<Vec<_>>();
                
                draw.polyline()
                    .weight(STROKE_WEIGHT)
                    .points_colored(points);
            } else {
                // Draw a regular circle
                draw.ellipse()
                    .xy(self.center)
                    .w_h(ring_radius * 2.0, ring_radius * 2.0)
                    .no_fill()
                    .stroke(color_with_alpha)
                    .stroke_weight(STROKE_WEIGHT);
            }
        }
    }
    
    // Check if ripple should be removed
    fn is_expired(&self) -> bool {
        self.radius >= MAX_RADIUS
    }
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model { 
        ripples: vec![],
        noise: noise::Perlin::new(),
        time: 0.0,
        mouse_down: false,
        last_ripple_time: 0.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.time = app.time;
    
    // Update all ripples
    for ripple in model.ripples.iter_mut() {
        ripple.update(model.time);
    }
    
    // Remove expired ripples
    model.ripples.retain(|ripple| !ripple.is_expired());
    
    // Create new ripples while mouse is held down
    if model.mouse_down {
        let current_time = app.time;
        // Create ripples with some spacing in time (every 0.1 seconds)
        if current_time - model.last_ripple_time > 0.1 {
            model.ripples.push(Ripple::new(app.mouse.position(), current_time));
            model.last_ripple_time = current_time;
        }
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { simple: Some(MousePressed(button)), .. } => {
            if button == MouseButton::Left {
                model.mouse_down = true;
                model.ripples.push(Ripple::new(app.mouse.position(), app.time));
                model.last_ripple_time = app.time;
            }
        },
        Event::WindowEvent { simple: Some(MouseReleased(button)), .. } => {
            if button == MouseButton::Left {
                model.mouse_down = false;
            }
        },
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    // Draw background
    draw.background().color(BLACK);
    effects::draw_water_background(&draw, app, model.noise, model.time);

    // Draw all ripples
    for ripple in &model.ripples {
        ripple.draw(&draw, app, model.time);
    }

    // Render everything
    draw.to_frame(app, &frame).unwrap();
}