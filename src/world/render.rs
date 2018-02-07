use stdweb::web::Element;
use stdweb::Value;
use stdweb::unstable::TryInto;

pub struct Renderer {
    context: Value,
    pub size:    (u32, u32),
}

impl Renderer {
    pub fn new(canvas: &Element) -> Renderer {
        Renderer {
            context: js!( return @{&canvas}.getContext("2d"); ),
            size: {
                let sz: (u32, u32) = (js!( return window.innerWidth ).try_into().unwrap(),
                                      js!( return window.innerHeight ).try_into().unwrap());
                ( if sz.0 > 1280 { 1280 } else { sz.0 },
                  if sz.1 > 720  { 720  } else { sz.1 }  )
            },
        }
    }

    pub fn load_font(&self, font: &'static str, size: u32) {
        let real_size: u32 = (size as f32 * self.size.0 as f32 / 720.0) as u32;
        js! {
            @{&self.context}.font = @{real_size} + "px " + @{font};
        };
    }

    pub fn clear(&self) {
        js! {
            @{&self.context}.clearRect(0, 0, @{&self.size.0}, @{&self.size.1});
        };
    }



    
    // == DRAWING FUNCTIONS ==

    // Primitives

    pub fn draw_box(&self, color: &str, pos: (f32, f32), sz: (f32, f32)) {
        js!{
            @{&self.context}.beginPath();
            @{&self.context}.rect(@{pos.0}, @{pos.1}, @{sz.0}, @{sz.1});
            @{&self.context}.fillStyle = @{color};
            @{&self.context}.fill();
        };
    }

    pub fn draw_circle(&self, color: &str, pos: (f32, f32), radius: f32) {
        js! {
            @{&self.context}.beginPath();
            @{&self.context}.arc(@{pos.0}, @{pos.1}, @{radius}, 0, Math.PI * 2.0);
            @{&self.context}.fillStyle = @{color};
            @{&self.context}.fill();
            @{&self.context}.closePath();
        };
    }

    pub fn draw_tile(&self, color: &str, pos: (f32, f32), size: (f32, f32)) {
        js! (
            var ctx = @{&self.context};
            var pos_x = @{pos.0};
            var pos_y = @{pos.1};
            var size_x = @{size.0} / 2.0;
            var size_y = @{size.1} / 2.0;
            
            // Upper triangle
            ctx.beginPath();
            ctx.moveTo(pos_x - size_x,
                       pos_y - size_y);
            ctx.lineTo(pos_x, pos_y);
            ctx.lineTo(pos_x + size_x,
                       pos_y - size_y);

            ctx.fillStyle = @{color};
            ctx.fill();
            ctx.closePath();

            // Lower triangle
            ctx.beginPath();
            ctx.moveTo(pos_x - size_x,
                       pos_y + size_y);
            ctx.lineTo(pos_x, pos_y);
            ctx.lineTo(pos_x + size_x,
                       pos_y + size_y);
            ctx.fillStyle = "#666";
            ctx.fill();
            ctx.closePath();

            // Left triangle
            ctx.beginPath();
            ctx.moveTo(pos_x - size_x,
                       pos_y - size_y);
            ctx.lineTo(pos_x, pos_y);
            ctx.lineTo(pos_x - size_x,
                       pos_y + size_y);
            ctx.fillStyle = "#AAA";
            ctx.fill();
            ctx.closePath();

            // Right triangle
            ctx.beginPath();
            ctx.moveTo(pos_x + size_x,
                       pos_y - size_y);
            ctx.lineTo(pos_x, pos_y);
            ctx.lineTo(pos_x + size_x,
                       pos_y + size_y);
            ctx.fillStyle = "#AAA";
            ctx.fill();
            ctx.closePath();
        );
    }

    


    // Game objects
    
    pub fn draw_paddle(&self, sprite: &Value, pos: (f32, f32), size: (f32, f32)) {
        js! {
            @{&self.context}.drawImage(@{sprite},
                                       @{pos.0}, @{pos.1},
                                       @{size.0},
                                       @{size.1});
        };
    }

    pub fn draw_sphere(&self, sprite: &Value, pos: (f32, f32), diameter: f32) {
        let pos = (pos.0 - (diameter / 2.0),
                   pos.1 - (diameter / 2.0));
        js! {
            @{&self.context}.drawImage(@{sprite},
                                       @{pos.0}, @{pos.1},
                                       @{diameter},
                                       @{diameter});
        };
    }


    
    // Miscellaneous

    pub fn draw_text(&self, color: &str, align: &str, pos: (f32, f32), text: &str) {
        js! {
            @{&self.context}.fillStyle = @{color};
            @{&self.context}.textAlign = @{align};
            @{&self.context}.fillText(@{text}, @{pos.0}, @{pos.1});
        };
    }
}
