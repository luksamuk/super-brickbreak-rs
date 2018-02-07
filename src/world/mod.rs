use input;
use stdweb::web;
use stdweb::unstable::TryInto;

mod state;
mod render;
mod physics;


use self::state::{BallState, PaddleState, Block};
use self::render::Renderer;
use self::physics::Collision;







pub struct World {
    pub canvas:       web::Element,
    pub renderer:     Renderer,

    
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
            renderer: Renderer::new(&canvas),
            fps: 0.0,
            pause: false,
            
            // Before you say "the document keeps track of fullscreen state":
            // I already tried using that.
            fullscreen: false,
            input:        input::KeyState::new(),
            tilt:         input::TiltState::new(),
            ball_state:   BallState::new(),
            paddle_state: PaddleState::new(),

            block_size: (0.0, 0.0),
            level_blocks: vec![],
            collided: false,
        };
        
        world.fit_viewport();
        world.paddle_state.xpos = world.renderer.size.0 as f32 / 2.0;

        // Load font
        world.renderer.load_font("GohuFont", 14);

        // == TEST: Create blocks
        world.block_size = (world.renderer.size.0 as f32 * 0.06,
                            world.renderer.size.1 as f32 * 0.0520845);
        for i in 0..9 {
            for j in 0..9 {
                let pos = ((world.renderer.size.0 as f32 / 4.0) + (i as f32 * world.block_size.0) as f32,
                           (world.renderer.size.1 as f32 / 4.0) + (j as f32 * world.block_size.1) as f32);
                world.level_blocks.push(Block {
                    pos: pos,
                    color: "#fff".to_string(),
                    active: true
                });
            }
        }
        // ==
        
        world
    }


    

    pub fn fit_viewport(&mut self) {
        js!( @{&self.canvas}.width = @{&self.renderer.size.0};
             @{&self.canvas}.height = @{&self.renderer.size.1}; );

        // Fix some values which are viewport-dependent
        //self.paddle_state.xpos = self.renderer.size.0 as f32 / 2.0;
        self.ball_state.diameter = self.renderer.size.1 as f32 * 0.034723;
        self.ball_state.basespd = self.renderer.size.1 as f32 / 72.0;

        self.paddle_state.ypos = 11.0 * self.renderer.size.1 as f32 / 12.0;
        self.paddle_state.basespd = self.renderer.size.1 as f32 / 72.0 * 0.75;
        self.paddle_state.spd = self.paddle_state.basespd;
        self.paddle_state.sz = (self.renderer.size.0 as f32 * 0.12,
                                self.renderer.size.1 as f32 * 0.034723);

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
                    //let rotations = (self.tilt.new.beta, // main angle
                    //                 self.tilt.new.gamma);
                    //let spin = (rotations.0.cos() * rotations.1.sin())
                    //    .atan2(rotations.0.sin());

                    // moving X axis (roll; beta) around Y axis (pitch; gamma)
                    //let rotations = (self.tilt.new.gamma,
                    //                 self.tilt.new.beta);
                    //let spin = (rotations.0.cos() * rotations.1.sin())
                    //    .atan2(rotations.0.sin());

                    // moving Z axis (yaw; alpha) around Y axis (pitch; gamma)
                    //let rotations = (self.tilt.new.gamma,
                    //                 self.tilt.new.alpha);
                    //let spin = (rotations.0.cos() * rotations.1.sin())
                    //    .atan2(rotations.0.sin());

                    let spin = self.tilt.new.beta;

                    
                    
                    let ratio = spin as f32 *
                    // Make the tilt a little more violent by narrowing
                    // the distance down, depending on orientation
                        if self.tilt.orient == input::OrientationType::Portrait {
                            3.0
                        } else { 3.5 };

                    let halfwidth = self.renderer.size.0 as f32 / 2.0;
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
            } else if paddle_bounds.1 > self.renderer.size.0 as f32 {
                self.paddle_state.xpos = self.renderer.size.0 as f32 - paddle_halfwidth;
            }

            // Handle ball state
            if self.ball_state.stopped {
                self.ball_state.pos.0 = self.paddle_state.xpos;
                self.ball_state.pos.1 = 21.0 * self.renderer.size.1 as f32 / 24.0;
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
                } else if ball_boundary.1 > self.renderer.size.0 as f32 && self.ball_state.spd.0 > 0.0 {
                    self.ball_state.pos.0 = self.renderer.size.0 as f32 - ball_radius;
                    self.ball_state.spd.0 *= -1.0;
                }

                // Handle Y axis
                if ball_boundary.2 < 0.0 && self.ball_state.spd.1 < 0.0 {
                    self.ball_state.pos.1 = ball_radius;
                    self.ball_state.spd.1 *= -1.0;
                } else if ball_boundary.2 > self.renderer.size.1 as f32 && self.ball_state.spd.1 > 0.0 {
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
                    if let Some(collision) = Collision::collides(ballstate, block.pos, tile_bounds) {
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



    


    pub fn render(&self) {
        self.renderer.clear();

        let ball_radius = self.ball_state.diameter / 2.0;

        // Afterimages
        let mut i: u8 = 0;
        for &afterimage in &self.ball_state.afterimages {
            let color = format!("#{:02X}{:02X}{:02X}", i, i, i);
            let color = color.as_ref();
            self.renderer.draw_circle(color, afterimage, ball_radius);
            i += 13;
        }
        
        // Actual ball
        {
            let sprite = &self.ball_state.sprite;
            let pos = self.ball_state.pos;
            let diameter = self.ball_state.diameter;
            self.renderer.draw_sphere(sprite, pos, diameter);
        }
        
        // Paddle
        {
            let sprite = &self.paddle_state.sprite;
            let pos = ( self.paddle_state.xpos - (self.paddle_state.sz.0 / 2.0),
                        self.paddle_state.ypos );
            let size = self.paddle_state.sz;
            self.renderer.draw_paddle(sprite, pos, size);
        }

        // Testing tiles
        for block in &self.level_blocks {
            self.renderer.draw_tile(block.color.as_ref(),
                                    block.pos,
                                    self.block_size);
        }

        // FPS
        self.renderer.draw_text("white", "left",
                                (ball_radius, ball_radius + 4.0),
                                format!("FPS: {}", f64::floor(self.fps)).as_ref());

        // Copyright
        self.renderer.draw_text("white", "right",
                                ((self.renderer.size.0 as f32) - ball_radius, ball_radius + 8.0),
                                "©2018 Lucas Vieira");
        self.renderer.draw_text("white", "right",
                                ((self.renderer.size.0 as f32) - ball_radius, (ball_radius * 3.0) + 8.0),
                                "Prototype Version");

        // Pause text
        if self.pause {
            self.renderer.draw_text("white", "center",
                                    (self.renderer.size.0 as f32 / 2.0, self.renderer.size.1 as f32 / 2.0),
                                    "PAUSE");
        }
        
    }
}
