use rand::Rng;
use nannou::prelude::*;
use noise::NoiseFn;

// Color palette for ripple effects
const COLORS: [rgb::Srgb<u8>; 8] = [
    NAVAJOWHITE, 
    LIGHTGOLDENRODYELLOW, 
    ANTIQUEWHITE, 
    BLANCHEDALMOND, 
    WHITE,
    LIGHTCYAN,
    SKYBLUE,
    LIGHTSTEELBLUE
];

// Water effect settings
const WATER_DETAIL_LEVELS: usize = 4;  // How many octaves of noise
const WATER_ANIMATION_SPEED: f32 = 0.15;

// Lava effect settings
const LAVA_DETAIL_LEVELS: usize = 3;  // Fewer octaves for chunkier look
const LAVA_ANIMATION_SPEED: f32 = 0.05; // Slower movement for lava

// Generate a random color from the palette
pub fn random_color() -> rgb::Srgb<u8> {
    let index = rand::thread_rng().gen_range(0, COLORS.len());
    COLORS[index]
}

// Linear interpolation between two colors
fn lerp(start: Rgb, end: Rgb, percent: f32) -> Rgb {
    rgb(
        start.red + percent * (end.red - start.red),
        start.green + percent * (end.green - start.green),
        start.blue + percent * (end.blue - start.blue),
    )
}

// Draw the animated water background
pub fn draw_water_background(draw: &Draw, app: &App, noise: noise::Perlin, time: f32) {
    let win = app.window_rect();
    let resolution = 3; // Draw every Nth row for performance
    
    // Define colors
    let water_surface_color = rgb(0.0, 0.3, 0.6);
    let water_deep_color = rgb(0.0, 0.05, 0.2);

    for y in (win.bottom() as i32..win.top() as i32).step_by(resolution) {
        let depth_factor = (y as f32 - win.bottom()) / win.h();
        let base_color = lerp(water_deep_color, water_surface_color, depth_factor);

        // Multi-octave noise for more natural water appearance
        let noise_value = (0..WATER_DETAIL_LEVELS).fold(0.0, |acc, i| {
            let amplitude = 0.5_f32.powi(i as i32);
            let scale = 0.01 * (i + 1) as f64;
            
            acc + amplitude * noise.get([
                time as f64 * WATER_ANIMATION_SPEED as f64 * (i + 1) as f64,
                y as f64 * scale,
                time as f64 * 0.05
            ]) as f32
        });

        // Apply light shimmering effect
        let light_factor = 1.0 + noise_value * 0.2;
        let color = rgb(
            (base_color.red * light_factor).min(1.0),
            (base_color.green * light_factor).min(1.0),
            (base_color.blue * light_factor).min(1.0),
        );

        // Draw water line
        draw.line()
            .start(pt2(win.left(), y as f32))
            .end(pt2(win.right(), y as f32))
            .color(color);
    }
    
    // Adds some highlight specks on the water surface for extra effect
    let speck_count = 500;
    let mut rng = rand::thread_rng();
    
    for _ in 0..speck_count {
        let x = rng.gen_range(win.left(), win.right());
        let y = rng.gen_range(win.bottom(), win.top());
        
        // Use noise to determine visibility of speck (makes them flicker)
        let noise_val = noise.get([
            x as f64 * 0.01, 
            y as f64 * 0.01, 
            time as f64 * 0.5
        ]) as f32;
        
        if noise_val > 0.7 {
            let size = rng.gen_range(1.0, 3.0);
            let brightness = rng.gen_range(0.7, 1.0);
            
            draw.ellipse()
                .xy(pt2(x, y))
                .w_h(size, size)
                .color(rgba(brightness, brightness, brightness, 0.6));
        }
    }
}

// Draw the animated lava background
pub fn draw_lava_background(draw: &Draw, app: &App, noise: noise::Perlin, time: f32) {
    let win = app.window_rect();
    let resolution = 4; // Slightly chunkier resolution for lava
    
    // Define colors
    let lava_surface_color = rgb(0.9, 0.3, 0.0);  // Bright orange-red
    let lava_deep_color = rgb(0.4, 0.0, 0.0);     // Dark red

    for y in (win.bottom() as i32..win.top() as i32).step_by(resolution) {
        let depth_factor = (y as f32 - win.bottom()) / win.h();
        let base_color = lerp(lava_deep_color, lava_surface_color, depth_factor);

        // Multi-octave noise for bubbling lava appearance
        let noise_value = (0..LAVA_DETAIL_LEVELS).fold(0.0, |acc, i| {
            let amplitude = 0.6_f32.powi(i as i32); // Higher amplitude for more contrast
            let scale = 0.008 * (i + 1) as f64;     // Larger features
            
            acc + amplitude * noise.get([
                time as f64 * LAVA_ANIMATION_SPEED as f64 * (i + 1) as f64,
                y as f64 * scale,
                time as f64 * 0.03
            ]) as f32
        });

        // Apply glowing/bubbling effect
        let glow_factor = 1.0 + noise_value * 0.4; // More intense variation
        let color = rgb(
            (base_color.red * glow_factor).min(1.0),
            (base_color.green * glow_factor).min(1.0),
            (base_color.blue * glow_factor).min(1.0),
        );

        // Draw lava line
        draw.line()
            .start(pt2(win.left(), y as f32))
            .end(pt2(win.right(), y as f32))
            .color(color);
    }
    
    // Add bubbles and sparks to the lava
    let bubble_count = 300;
    let mut rng = rand::thread_rng();
    
    for _ in 0..bubble_count {
        let x = rng.gen_range(win.left(), win.right());
        let y = rng.gen_range(win.bottom(), win.top());
        
        // Use noise to determine visibility (bubbling effect)
        let noise_val = noise.get([
            x as f64 * 0.015, 
            y as f64 * 0.015, 
            time as f64 * 0.2
        ]) as f32;
        
        if noise_val > 0.65 {
            let size = rng.gen_range(1.0, 5.0);
            
            // Brighter at the top (rising heat)
            let y_factor = (y - win.bottom()) / win.h();
            let brightness = 0.7 + y_factor * 0.3;
            
            // Yellow-orange glow
            draw.ellipse()
                .xy(pt2(x, y))
                .w_h(size, size)
                .color(rgba(brightness, brightness * 0.6, 0.0, 0.7));
        }
    }
    
    // Add a few bright sparks
    let spark_count = 50;
    for _ in 0..spark_count {
        let x = rng.gen_range(win.left(), win.right());
        let y = rng.gen_range(win.bottom(), win.top());
        
        let noise_val = noise.get([
            x as f64 * 0.02, 
            y as f64 * 0.02, 
            time as f64 * 1.5
        ]) as f32;
        
        if noise_val > 0.8 {
            let size = rng.gen_range(1.0, 2.5);
            
            draw.ellipse()
                .xy(pt2(x, y))
                .w_h(size, size)
                .color(rgba(1.0, 1.0, 0.3, 0.9)); // Bright yellow spark
        }
    }
}