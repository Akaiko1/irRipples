use nannou::prelude::*;

// UI state
pub struct Menu {
    pub visible: bool,
    pub toggle_button_rect: Rect,
    pub wobble_enabled: bool,
    pub fade_enabled: bool,
    pub wobble_button_rect: Rect,
    pub fade_button_rect: Rect,
    pub background_type: BackgroundType,
    pub bg_type_button_rect: Rect,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BackgroundType {
    None,
    Water,
    Lava,
    Radial,
}

impl Menu {
    pub fn new(window_rect: Rect, wobble_enabled: bool, fade_enabled: bool) -> Self {
        // Position in top left corner with some padding
        let padding = 10.0;
        let button_size = 30.0;
        let toggle_button_rect = Rect::from_x_y_w_h(
            window_rect.left() + padding + button_size/2.0,
            window_rect.top() - padding - button_size/2.0,
            button_size,
            button_size,
        );
        
        // Define the other button positions (initially hidden)
        let button_spacing = 10.0;
        let button_width = 120.0;
        let button_height = 25.0;
        
        let wobble_button_rect = Rect::from_x_y_w_h(
            window_rect.left() + padding + button_width/2.0,
            toggle_button_rect.bottom() - button_spacing - button_height/2.0,
            button_width,
            button_height,
        );
        
        let fade_button_rect = Rect::from_x_y_w_h(
            window_rect.left() + padding + button_width/2.0,
            wobble_button_rect.bottom() - button_spacing - button_height/2.0,
            button_width,
            button_height,
        );

        let bg_type_button_rect = Rect::from_x_y_w_h(
            window_rect.left() + padding + button_width/2.0,
            fade_button_rect.bottom() - button_spacing - button_height/2.0,
            button_width,
            button_height,
        );
        
        Menu {
            visible: false,
            toggle_button_rect,
            wobble_enabled,      // Use provided parameter
            fade_enabled,        // Use provided parameter
            wobble_button_rect,
            fade_button_rect,
            background_type: BackgroundType::Water,
            bg_type_button_rect,
        }
    }
    
    // Check if a point is inside the toggle button
    pub fn is_in_toggle_button(&self, point: Point2) -> bool {
        self.toggle_button_rect.contains(point)
    }
    
    // Check if a point is inside the wobble button
    pub fn is_in_wobble_button(&self, point: Point2) -> bool {
        self.visible && self.wobble_button_rect.contains(point)
    }

    pub fn is_in_bg_type_button(&self, point: Point2) -> bool {
        self.visible && self.bg_type_button_rect.contains(point)
    }
    
    // Check if a point is inside the fade button
    pub fn is_in_fade_button(&self, point: Point2) -> bool {
        self.visible && self.fade_button_rect.contains(point)
    }
    
    // Draw the menu
    pub fn draw(&self, draw: &Draw) {
        // Always draw the toggle button
        draw.rect()
            .xy(self.toggle_button_rect.xy())
            .wh(self.toggle_button_rect.wh())
            .color(rgba(0.1, 0.1, 0.2, 0.8));
            
        // Draw the icon (hamburger menu)
        let line_width = self.toggle_button_rect.w() * 0.6;
        let line_height = 2.0;
        let line_spacing = 4.0;
        
        // Define y-positions for the three lines
        let y_positions = [
            self.toggle_button_rect.y() + line_spacing,  // top line
            self.toggle_button_rect.y(),                 // middle line
            self.toggle_button_rect.y() - line_spacing,  // bottom line
        ];
        
        // Draw hamburger menu lines using iteration
        for &y in &y_positions {
            draw.line()
                .start(pt2(self.toggle_button_rect.x() - line_width/2.0, y))
                .end(pt2(self.toggle_button_rect.x() + line_width/2.0, y))
                .weight(line_height)
                .color(WHITE);
        }
        
        // Draw the rest of the menu if visible
        if self.visible {
            // Background panel for menu
            let padding = 5.0;
            let panel_rect = Rect::from_x_y_w_h(
                self.toggle_button_rect.x(),
                (self.toggle_button_rect.bottom() + self.bg_type_button_rect.bottom()) / 2.0,
                self.wobble_button_rect.w() + padding * 2.0,
                self.toggle_button_rect.bottom() - self.bg_type_button_rect.bottom() + padding * 2.0,
            );
            
            draw.rect()
                .xy(panel_rect.xy())
                .wh(panel_rect.wh())
                .color(rgba(0.05, 0.05, 0.1, 0.8));
                
            // Wobble toggle button
            let wobble_color = if self.wobble_enabled { 
                rgba(0.2, 0.8, 0.3, 0.9) 
            } else { 
                rgba(0.8, 0.2, 0.2, 0.9) 
            };
            
            draw.rect()
                .xy(self.wobble_button_rect.xy())
                .wh(self.wobble_button_rect.wh())
                .color(wobble_color);
                
            // Wobble button text
            let wobble_text = if self.wobble_enabled { "Wobble: ON" } else { "Wobble: OFF" };
            draw.text(wobble_text)
                .xy(self.wobble_button_rect.xy())
                .font_size(14)
                .color(WHITE)
                .align_text_middle_y();
                
            // Fade toggle button
            let fade_color = if self.fade_enabled { 
                rgba(0.2, 0.8, 0.3, 0.9) 
            } else { 
                rgba(0.8, 0.2, 0.2, 0.9) 
            };
            
            draw.rect()
                .xy(self.fade_button_rect.xy())
                .wh(self.fade_button_rect.wh())
                .color(fade_color);
                
            // Fade button text
            let fade_text = if self.fade_enabled { "Fade: ON" } else { "Fade: OFF" };
            draw.text(fade_text)
                .xy(self.fade_button_rect.xy())
                .font_size(14)
                .color(WHITE)
                .align_text_middle_y();
                 

            // Background type button
            let bg_color = match self.background_type {
                BackgroundType::None => rgba(0.8, 0.2, 0.2, 0.9),
                BackgroundType::Water => rgba(0.0, 0.4, 0.8, 0.9),
                BackgroundType::Lava => rgba(0.9, 0.3, 0.0, 0.9),
                BackgroundType::Radial => rgba(0.8, 0.4, 0.8, 0.9),
            };
            
            draw.rect()
            .xy(self.bg_type_button_rect.xy())
            .wh(self.bg_type_button_rect.wh())
            .color(bg_color);
                
            // Background type text
            let bg_text = match self.background_type {
                BackgroundType::None => "BG: OFF",
                BackgroundType::Water => "BG: WATER",
                BackgroundType::Lava => "BG: LAVA",
                BackgroundType::Radial => "BG: RADIAL",
            };
            
            draw.text(bg_text)
                .xy(self.bg_type_button_rect.xy())
                .font_size(14)
                .color(WHITE)
                .align_text_middle_y();
        }
    }
}