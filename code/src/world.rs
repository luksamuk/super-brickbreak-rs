use input;
use stdweb::{self, Value};
use stdweb::web::{self, Element};
use stdweb::unstable::TryInto;
use std::f64::consts;

pub struct BallState {
    pub diameter: f32,
    pub pos:      (f32, f32),
    pub spd:      (f32, f32),
    pub stopped:  bool,
    pub basespd:  f32,
    pub afterimages: Vec<(f32, f32)>,
}


pub struct PaddleState {
    pub sprite:   stdweb::Value,
    pub xpos:     f32,
    pub ypos:     f32,
    pub spd:      f32,
    pub sz:       (f32, f32),
    pub basespd:  f32,
}

pub struct Block {
    pub pos:    (f32, f32),
    pub color:  String,
    pub active: bool,
}


pub struct World {
    pub canvas:       web::Element,
    pub vwpsize:      (u32, u32),
    pub context:      stdweb::Value,
    pub fps:          f64,
    pub pause:        bool,
    pub fullscreen:   bool,

    pub input:        input::KeyState,
    pub tilt:         input::TiltState,
    pub ball_state:   BallState,
    pub paddle_state: PaddleState,

    pub block_size:   (f32, f32),
    pub level_blocks: Vec<Block>,
    pub collided:     bool,
}


impl World {
    pub fn new() -> World {
        let canvas = web::document().get_element_by_id("viewport").unwrap();
        let mut world = World {
            canvas:     canvas.clone(),
            vwpsize: {
                let sz: (u32, u32) = (js!( return window.innerWidth ).try_into().unwrap(),
                                      js!( return window.innerHeight ).try_into().unwrap());
                ( if sz.0 > 1280 { 1280 } else { sz.0 },
                  if sz.1 > 720  { 720  } else { sz.1 }  )
            },
            context:    js!( return @{&canvas}.getContext("2d"); ),
            fps: 0.0,
            pause: false,
            // Before you say "the document keeps track of fullscreen state":
            // I already tried using that.
            fullscreen: false,
            input: input::KeyState::new(),
            tilt:  input::TiltState::new(),
            ball_state: BallState {
                diameter:    0.0,
                pos:         (0.0, 0.0),
                spd:         (0.0, 0.0),
                stopped:     true,
                basespd:     0.0,
                afterimages: Vec::with_capacity(7),
            },
            paddle_state: PaddleState {
                sprite:   js! {
                    var img = new Image();
                    img.src = "./paddle.png";
                    return img;
                },
                xpos:     0.0,
                ypos:     0.0,
                spd:      0.0,
                basespd:  0.0,
                sz:       (0.0, 0.0),
            },

            block_size: (0.0, 0.0),
            level_blocks: vec![],
            collided: false,
        };
        
        world.fit_viewport();
        world.paddle_state.xpos = world.vwpsize.0 as f32 / 2.0;

        // Just some spare code to actually load the game font. Don't mind it.
        js! {
            @{&world.context}.font = (14 * @{&world.canvas}.width / 720) + "px GohuFont";
        };

        // TEST: Create blocks
        world.block_size = (world.vwpsize.0 as f32 * 0.06,
                            world.vwpsize.1 as f32 * 0.0520845);
        for i in 0..9 {
            for j in 0..9 {
                let pos = ((world.vwpsize.0 as f32 / 4.0) + (i as f32 * world.block_size.0) as f32,
                           (world.vwpsize.1 as f32 / 4.0) + (j as f32 * world.block_size.1) as f32);
                world.level_blocks.push(Block {
                    pos: pos,
                    color: "#fff".to_string(),
                    active: true
                });
            }
        }
        
        world
    }

    pub fn clear(&self) {
        js! {
            @{&self.context}.clearRect(0, 0,
                                       @{&self.vwpsize.0},
                                       @{&self.vwpsize.1});
        };
    }
    
    pub fn draw_box(&self, color: &str, pos: (f32, f32), sz: (f32, f32)) {
        js!{
            @{&self.context}.beginPath();
            @{&self.context}.rect(@{pos.0}, @{pos.1}, @{sz.0}, @{sz.1});
            @{&self.context}.fillStyle = @{color};
            @{&self.context}.fill();
        };
    }

    pub fn draw_paddle(&self, pos: (f32, f32)) {
        js! {
            @{&self.context}.drawImage(@{&self.paddle_state.sprite},
                                       @{pos.0}, @{pos.1},
                                       @{&self.paddle_state.sz.0},
                                       @{&self.paddle_state.sz.1});
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

    pub fn draw_text(&self, color: &str, align: &str, pos: (f32, f32), text: &str) {
        js! {
            @{&self.context}.fillStyle = @{color};
            @{&self.context}.textAlign = @{align};
            @{&self.context}.fillText(@{text}, @{pos.0}, @{pos.1});
        };
    }


    pub fn draw_tile(&self, color: &str, pos: (f32, f32)) {
        js! (
            var ctx = @{&self.context};
            var pos_x = @{pos.0};
            var pos_y = @{pos.1};
            var size_x = @{self.block_size.0} / 2.0;
            var size_y = @{self.block_size.1} / 2.0;
            
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


    

    pub fn fit_viewport(&mut self) {
        js!( @{&self.canvas}.width = @{&self.vwpsize.0};
             @{&self.canvas}.height = @{&self.vwpsize.1}; );

        // Fix some values which are viewport-dependent
        //self.paddle_state.xpos = self.vwpsize.0 as f32 / 2.0;
        self.ball_state.diameter = self.vwpsize.1 as f32 * 0.034723;
        self.ball_state.basespd = self.vwpsize.1 as f32 / 72.0;

        self.paddle_state.ypos = 11.0 * self.vwpsize.1 as f32 / 12.0;
        self.paddle_state.basespd = self.vwpsize.1 as f32 / 72.0 * 0.75;
        self.paddle_state.spd = self.paddle_state.basespd;
        self.paddle_state.sz = (self.vwpsize.0 as f32 * 0.12,
                                self.vwpsize.1 as f32 * 0.034723);

        // TODO: Also reposition paddle and ball, if we get any problem
        // coming back from fullscreen
    }

    // NOTE: This only works in event handlers.
    pub fn toggle_fullscreen(&mut self) {
        let fullscreen_press = {
            let newstate = match self.input.async.get(&input::KeyType::FullScreen) {
                Some(&state) => state,
                None => false,
            };
            let oldstate = match self.input.new.get(&input::KeyType::FullScreen) {
                Some(&state) => state,
                None => false,
            };
            newstate && !oldstate
        };
        
        if fullscreen_press {
            js! {
                if (typeof document.webkitCancelFullScreen !== "undefined") {
                    if (@{&self.fullscreen}) {
                        console.log("Exiting fullscreen mode");
                        document.webkitCancelFullScreen();
                    } else {
                        console.log("Entering fullscreen mode");
                        @{&self.canvas}.webkitRequestFullScreen(Element.ALLOW_KEYBOARD_INPUT);
                    }
                } else if (typeof document.mozCancelFullScreen !== "undefined") {
                    if(@{&self.fullscreen}) {
                        console.log("Exiting fullscreen mode");
                        document.mozCancelFullScreen();
                    } else {
                        console.log("Entering fullscreen mode");
                        @{&self.canvas}.mozRequestFullScreen();
                    }
                }
            };
            self.fullscreen = !self.fullscreen;
        }

        self.fit_viewport();
    }

    pub fn input_dispatch(&mut self, key: input::KeyType, pressed: bool) {
        self.input.async.insert(key, pressed);
    }






    pub fn update(&mut self, dt: f64) {
        // Collect input state
        self.input.new = self.input.async.clone();
        if self.tilt.active {
            self.tilt.new = self.tilt.async.clone();
            // Collect device orientation
            let is_portrait: bool = js!( return (window.innerHeight > window.innerWidth); )
                .try_into()
                .unwrap();
            if is_portrait {
                self.tilt.orient = input::OrientationType::Portrait;
            } else {
                self.tilt.orient = input::OrientationType::Landscape;
            }
        }
        
        // Process new input
        for (key, state) in &self.input.new {
            match (key, state) {
                (&input::KeyType::Left,  &true) => {
                    if !self.pause {
                        self.paddle_state.xpos -= self.paddle_state.spd;
                    }
                },
                (&input::KeyType::Right, &true) => {
                    if !self.pause {
                        self.paddle_state.xpos += self.paddle_state.spd;
                    }
                }
                (&input::KeyType::S, &true)     => {
                    if !self.pause && self.ball_state.stopped == true {
                        // Eh well, something funny was going on with the rand crate, so
                        // what the heck, might as well use js.
                        let initial_angle: f64 = js!( return 67.5 + (Math.random() * 46); )
                            .try_into()
                            .unwrap();
                        let initial_angle = initial_angle as f32; // We lose precision, but meh
                        self.ball_state.spd =
                            (self.ball_state.basespd * f32::cos(initial_angle.to_radians()),
                             -self.ball_state.basespd * f32::sin(initial_angle.to_radians()) );

                        self.ball_state.stopped = false;
                    }
                },

                // Paddle move speed depends on whether you're holding A or not
                (&input::KeyType::A, &true) => self.paddle_state.spd = self.paddle_state.basespd * 2.0,
                (&input::KeyType::A, &false) => self.paddle_state.spd = self.paddle_state.basespd,
                _ => {},
            }
        }

        // Check for single-press of pause key
        {
            let enter_press = {
                let newstate = match self.input.new.get(&input::KeyType::Enter) {
                    Some(&state) => state,
                    None => false,
                };
                let oldstate = match self.input.old.get(&input::KeyType::Enter) {
                    Some(&state) => state,
                    None => false,
                };
                newstate && !oldstate
            };
            
            if enter_press {
                self.pause = !self.pause;
            }
        }

        // ======
        
        // The following events will only happen if the game is not paused.
        if !self.pause {

            // Process mobile input
            {
                if self.tilt.orient != input::OrientationType::Unknown {
                    // Calculate beta and gamma rotations, respectively
                    // Works well with landscape, and tilting up/down
                    // instead of left/right
                    // moving Y axis (pitch; gamma) around X axis (roll; beta)
                    let rotations = (self.tilt.new.beta, // main angle
                                     self.tilt.new.gamma);
                    let spin = (rotations.0.cos() * rotations.1.sin())
                        .atan2(rotations.0.sin());

                    // moving X axis (roll; beta) around Y axis (pitch; gamma)
                    let rotations = (self.tilt.new.gamma,
                                     self.tilt.new.beta);
                    let spin = (rotations.0.cos() * rotations.1.sin())
                        .atan2(rotations.0.sin());

                    // moving Z axis (yaw; alpha) around Y axis (pitch; gamma)
                    let rotations = (self.tilt.new.gamma,
                                     self.tilt.new.alpha);
                    let spin = (rotations.0.cos() * rotations.1.sin())
                        .atan2(rotations.0.sin());

                    let spin = self.tilt.new.beta;

                    
                    
                    let ratio = spin as f32 *
                    // Make the tilt a little more violent by narrowing
                    // the distance down, depending on orientation
                        if self.tilt.orient == input::OrientationType::Portrait {
                            3.0
                        } else { 3.5 };

                    let halfwidth = self.vwpsize.0 as f32 / 2.0;
                    self.paddle_state.xpos =  0.0 +  (halfwidth * ratio);
                }
            }

            // Clamp paddle position
            let paddle_halfwidth = self.paddle_state.sz.0 / 2.0;
            let paddle_bounds = (self.paddle_state.xpos - paddle_halfwidth,         // left
                                 self.paddle_state.xpos + paddle_halfwidth,         // right
                                 self.paddle_state.ypos,                            // top
                                 self.paddle_state.ypos + self.paddle_state.sz.1 ); // bottom

            if paddle_bounds.0 < 0.0 {
                self.paddle_state.xpos = paddle_halfwidth;
            } else if paddle_bounds.1 > self.vwpsize.0 as f32 {
                self.paddle_state.xpos = self.vwpsize.0 as f32 - paddle_halfwidth;
            }

            // Handle ball state
            if self.ball_state.stopped {
                self.ball_state.pos.0 = self.paddle_state.xpos;
                self.ball_state.pos.1 = 21.0 * self.vwpsize.1 as f32 / 24.0;
                if self.ball_state.afterimages.len() > 0 {
                    self.ball_state.afterimages.clear();
                }
            } else {
                // Transform position
                self.ball_state.pos.0 += self.ball_state.spd.0;
                self.ball_state.pos.1 += self.ball_state.spd.1;

                // Handle basic boundary collision
                let ball_radius = self.ball_state.diameter / 2.0;
                let ball_boundary = (self.ball_state.pos.0 - ball_radius,   // left
                                     self.ball_state.pos.0 + ball_radius,   // right
                                     self.ball_state.pos.1 - ball_radius,   // top
                                     self.ball_state.pos.1 + ball_radius ); // bottom

                // Handle X axis
                if ball_boundary.0 < 0.0 && self.ball_state.spd.0 < 0.0 {
                    self.ball_state.pos.0 = ball_radius;
                    self.ball_state.spd.0 *= -1.0;
                } else if ball_boundary.1 > self.vwpsize.0 as f32 && self.ball_state.spd.0 > 0.0 {
                    self.ball_state.pos.0 = self.vwpsize.0 as f32 - ball_radius;
                    self.ball_state.spd.0 *= -1.0;
                }

                // Handle Y axis
                if ball_boundary.2 < 0.0 && self.ball_state.spd.1 < 0.0 {
                    self.ball_state.pos.1 = ball_radius;
                    self.ball_state.spd.1 *= -1.0;
                } else if ball_boundary.2 > self.vwpsize.1 as f32 && self.ball_state.spd.1 > 0.0 {
                    // Respawn ball
                    self.ball_state.stopped = true;
                }

                // Handle paddle collision
                // Check if we're within Y and X range, respectively.
                if self.ball_state.spd.1 > 0.0 // If we're descending, and...
                    && ((ball_boundary.3 >= paddle_bounds.2) // We're at least intersecting...
                        && (ball_boundary.3 <= paddle_bounds.3)) // the paddle in any way...
                    // Then we verify if we're within X range...
                    && (ball_boundary.1 >= paddle_bounds.0 && ball_boundary.0 <= paddle_bounds.1) {
                        // We kind of bounce proportionally to the relative paddle position.
                        // The further away from the center of the paddle, the more open the
                        // bouncing angle is, scaling to 0.0 to 45.0 towards the edge.
                        // We first calculate a ratio [-1.0, 1.0], 0.0 being the paddle center.
                        let ratio = (-2.0 * ((self.ball_state.pos.0 - paddle_bounds.0)
                                             / (paddle_bounds.1 - paddle_bounds.0)))
                            + 1.0;
                        
                        // We compute the angle by assuming 90 degrees and then adding an angle
                        // in range [-45, 45]
                        let theta: f32 = ((90.0 + (ratio * 45.0)) as f32).to_radians();

                        // And now we apply theta to our ball's base speed, distributing it to
                        // the axis
                        self.ball_state.spd = ( self.ball_state.basespd * f32::cos(theta),
                                                -self.ball_state.basespd * f32::sin(theta) );
                    }

                // Afterimages
                if self.ball_state.afterimages.len() >= 7 {
                    self.ball_state.afterimages.drain(0..1);
                }
                self.ball_state.afterimages.push(self.ball_state.pos);
            } // End of moving ball events



            // Basic collision
            let ballpos = self.ball_state.pos;
            let ballradius = self.ball_state.diameter / 2.0;
            let tilesz = self.block_size;

            self.collided = false;

            // We iterate over all blocks and only
            // keep those who have not been collided.
            // This is not the best way to handle collision,
            // but it's enough for the amount of onscreen objs
            self.level_blocks.retain(|ref block| {
                !World::collides(ballpos, ballradius, block.pos,
                               (block.pos.0 - tilesz.0,
                                block.pos.0 + tilesz.0,
                                block.pos.1 - tilesz.1,
                                block.pos.1 + tilesz.1))
            });

            // Calculate resulting vector for all collisions and apply
            // to ball
            
            
            
        } // End of pausable events

        // Give input to old
        self.input.old = self.input.new.clone();
        if self.tilt.active {
            self.tilt.old = self.tilt.new.clone();
        }
    }



    fn collides(ball_pos: (f32, f32), ball_radius: f32, tile_pos: (f32, f32),
                tile_bounds: (f32, f32, f32, f32)) -> bool {
        // Bounds: (left, right, top, bottom)

        
        let closest_point = (
            if ball_pos.0 < tile_pos.0 {
                ball_pos.0.max(tile_bounds.0)
            } else {
                ball_pos.0.min(tile_bounds.1)
            },

            if ball_pos.1 < tile_pos.1 {
                ball_pos.1.max(tile_bounds.2)
            } else {
                ball_pos.1.min(tile_bounds.3)
            });

        let delta_pos = ((closest_point.0 - ball_pos.0),
                         (closest_point.1 - ball_pos.1));
        let square_distance = (delta_pos.0 * delta_pos.0) + (delta_pos.1 * delta_pos.1);
        
        square_distance < ball_radius * ball_radius
    }


    


    pub fn render(&self) {
        self.clear();

        let ball_radius = self.ball_state.diameter / 2.0;

        // Afterimages
        let mut i: u8 = 0;
        for &afterimage in &self.ball_state.afterimages {
            let color = format!("#{:02X}{:02X}{:02X}", i, i, i);
            let color = color.as_ref();
            self.draw_circle(color, afterimage, ball_radius);
            i += 13;
        }
        
        // Actual ball
        self.draw_circle("white", self.ball_state.pos, ball_radius);
        
        // Paddle
        let paddle_pos = self.paddle_state.xpos - (self.paddle_state.sz.0 / 2.0);
        //self.draw_box("white",
        //              (paddle_pos, self.paddle_state.ypos),
        //              self.paddle_state.sz);
        self.draw_paddle((paddle_pos, self.paddle_state.ypos));

        // FPS
        self.draw_text("white", "left",
                       (ball_radius, ball_radius + 4.0),
                       format!("FPS: {}", f64::floor(self.fps)).as_ref());

        // Pause text
        if self.pause {
            self.draw_text("white", "center",
                           (self.vwpsize.0 as f32 / 2.0, self.vwpsize.1 as f32 / 2.0),
                           "PAUSE");
        }

        // Testing tiles
        for block in &self.level_blocks {
            self.draw_tile(block.color.as_ref(), block.pos);
        }
        
    }
}
