use input;
use stdweb::{self, Value};
use stdweb::web::{self, Element};
use stdweb::unstable::TryInto;
use std::f64::consts;

pub struct BallState {
    pub sprite:   stdweb::Value,
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



#[derive(PartialEq, PartialOrd, Clone)]
pub struct Collision {
    pub pos:    (f32, f32),
    pub vector: (f32, f32),
    pub valid: bool,
}

impl Collision {
    fn new() -> Collision {
        Collision {
            pos:    (0.0, 0.0),
            vector: (0.0, 0.0),
            valid: false,
        }
    }

    fn from(position: (f32, f32), vector: (f32, f32)) -> Collision {
        Collision {
            pos: position,
            vector: vector,
            valid: true,
        }
    }
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
                sprite:      js! {
                    var img = new Image();
                    img.src = "./sphere.png";
                    return img;
                },
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

    pub fn draw_sphere(&self, pos: (f32, f32)) {
        let pos = (pos.0 - (self.ball_state.diameter / 2.0),
                   pos.1 - (self.ball_state.diameter / 2.0));
        js! {
            @{&self.context}.drawImage(@{&self.ball_state.sprite},
                                       @{pos.0}, @{pos.1},
                                       @{&self.ball_state.diameter},
                                       @{&self.ball_state.diameter});
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
            let mut retrieved_collisions = vec![];
            {
                let ballstate = &self.ball_state;
                self.level_blocks.retain(|ref block| {
                    let tile_bounds = (block.pos.0 - tilesz.0,
                                       block.pos.0 + tilesz.0,
                                       block.pos.1 - tilesz.1,
                                       block.pos.1 + tilesz.1);
                    if let Some(collision) = World::collides(ballstate, block.pos, tile_bounds) {
                        retrieved_collisions.push(collision);
                        return false;
                    }
                    
                    true
                });
            }

            // Calculate resulting vector
            // Multiblock consensus
            let final_collision = retrieved_collisions.iter()
                .fold(Collision::new(),
                      |acc, ref val| {
                          let mut acc = acc;
                          acc.valid = true;
                          acc.vector.0 += val.vector.0;
                          acc.vector.1 += val.vector.1;
                          acc.vector.0 = acc.vector.0.signum();
                          acc.vector.1 = acc.vector.1.signum();
                          acc
                      });

            // Single-block "consensus"
            //let final_collision = match retrieved_collisions.first() {
            //    Some(collision) => collision.clone(),
            //    None => Collision::new(),
            //};
            
            // Apply result to ball.
            // Notice that the resulting vector only ensures that
            // the ball's speeds have the same signal as the result
            // vector.
            if final_collision.valid {
                let mut ball_spd = self.ball_state.spd;
                //js! { console.log("Final: " + @{format!("{:?}", final_collision.vector)}); }
                if final_collision.vector.0 != 0.0
                    && ball_spd.0.signum() != final_collision.vector.0.signum() {
                    ball_spd.0 *= -1.0;
                }
                if final_collision.vector.1 != 0.0
                    && ball_spd.1.signum() != final_collision.vector.1.signum() {
                    ball_spd.1 *= -1.0;
                }
                self.ball_state.spd = ball_spd;
            }
            
            
        } // End of pausable events

        // Give input to old
        self.input.old = self.input.new.clone();
        if self.tilt.active {
            self.tilt.old = self.tilt.new.clone();
        }
    }



    fn collides(ball_state: &BallState, tile_pos: (f32, f32),
                tile_bounds: (f32, f32, f32, f32)) -> Option<Collision> {
        // Bounds: (left, right, top, bottom)
        let ball_pos = ball_state.pos;
        let ball_radius = ball_state.diameter / 4.0 as f32;

        
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
        
        if square_distance < ball_radius * ball_radius {
            // Calculate vector speed transform.
            // The returned transform vector is peculiar. Instead of
            // determining whether we should change the speed on current
            // status, we inform the ball which direction it should be headed,
            // according to the current angle between the angle of the ball
            // and the block's center.
            let collision_point = (ball_pos.0 - closest_point.0, ball_pos.1 - closest_point.1);
            
            let collision_angle = collision_point.1.atan2(collision_point.0).to_degrees();
            let collision_angle =
                if collision_angle < 0.0 {
                    360.0 + collision_angle
                } else {
                    collision_angle % 360.0
                };

            // The angle determines the quadrant in which the ball is, related to the
            // block's center. We have clamped our angle at 0..360 and we don't need to
            // be very thorough about the non-integer part of our angle. Therefore:
            let mut vector = (0.0, 0.0);
            match collision_angle as u32 {
                // Vertices. Notice that we give them some
                // degrees of tolerance (more or less 20 degrees)
                25 ... 65 => {
                    // 45 degrees, bottom-right
                    vector = (1.0, 1.0);
                },
                115 ... 155 => {
                    // 135 degrees, bottom-left
                    vector = (-1.0, 1.0);
                },
                205 ... 245 => {
                    // 225 degrees, top-left
                    vector = (-1.0, -1.0);
                },
                295 ... 335 => {
                    // 315 degrees, top-right
                    vector = (1.0, -1.0);
                },

                
                // Edges. Notice that, even though they
                // might be in superposition with a vertex'
                // angles, we trust the way that the pattern
                // matching works so we don't have to break the
                // angles so we can better understand the code later.
                46 ... 134 => {
                    // Bottom quadrant.
                    // Make ball move down (positive Y)
                    vector.1 = 1.0;
                },
                0 ... 44 | 316 ... 360 => {
                    // Left quadrant.
                    // Make ball move left (negative X)
                    vector.0 = -1.0;
                },
                136 ... 224 => {
                    // Right quadrant.
                    // Make ball move right (positive X)
                    vector.0 = 1.0;
                },
                226 ... 314 => {
                    // Top quadrant.
                    // Make ball move up (negative Y)
                    vector.1 = -1.0;
                    js! { console.log("TOP!"); }
                },

                // What the heck
                _ => {
                    js! {
                        console.log("Errr, what the heck is going on? Angle is " + @{collision_angle});
                    }
                } // No transformation needed
            }
            
            Some(Collision::from(ball_pos, vector))
        } else {
            None
        }
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
            //self.draw_sphere(color, afterimage);
            i += 13;
        }
        
        // Actual ball
        //self.draw_circle("white", self.ball_state.pos, ball_radius);
        self.draw_sphere(self.ball_state.pos);
        
        // Paddle
        let paddle_pos = self.paddle_state.xpos - (self.paddle_state.sz.0 / 2.0);
        //self.draw_box("white",
        //              (paddle_pos, self.paddle_state.ypos),
        //              self.paddle_state.sz);
        self.draw_paddle((paddle_pos, self.paddle_state.ypos));

        // Testing tiles
        for block in &self.level_blocks {
            self.draw_tile(block.color.as_ref(), block.pos);
        }

        // FPS
        self.draw_text("white", "left",
                       (ball_radius, ball_radius + 4.0),
                       format!("FPS: {}", f64::floor(self.fps)).as_ref());

        // Copyright
        self.draw_text("white", "right",
                       ((self.vwpsize.0 as f32) - ball_radius, ball_radius + 8.0),
                       "©2018 Lucas Vieira");
        self.draw_text("white", "right",
                       ((self.vwpsize.0 as f32) - ball_radius, (ball_radius * 3.0) + 8.0),
                       "Prototype Version");

        // Pause text
        if self.pause {
            self.draw_text("white", "center",
                           (self.vwpsize.0 as f32 / 2.0, self.vwpsize.1 as f32 / 2.0),
                           "PAUSE");
        }
        
    }
}
