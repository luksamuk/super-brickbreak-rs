#![recursion_limit="2048"]

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate lazy_static;

use stdweb::web::{
    self,
    IEventTarget,
//    INode,
//    IElement,
//    FileReader,
//    FileReaderResult,
//    Element,
//    ArrayBuffer
};

use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
//    ClickEvent,
//    ChangeEvent,
//    ProgressLoadEvent,
    KeydownEvent,
    KeyupEvent,
    KeyboardLocation,
};

use std::sync::Mutex;
use stdweb::unstable::TryInto;
use std::collections::HashMap;

// ==============================

#[derive(Hash, Eq, PartialEq, Clone)]
enum KeyType {
    Right,
    Left,
    S,
    A,
    Enter,
//    LMB,
//    Q,
}

struct KeyState {
    old: HashMap<KeyType, bool>,
    new: HashMap<KeyType, bool>,
}

impl KeyState {
    fn new() -> KeyState {
        KeyState {
            old: HashMap::new(),
            new: HashMap::new(),
        }
    }
}

struct BallState {
    diameter: f32,
    pos:      (f32, f32),
    spd:      (f32, f32),
    stopped:  bool,
    basespd:  f32,
}



struct PaddleState {
    xpos:     f32,
    ypos:     f32,
    spd:      f32,
    sz:       (f32, f32),
    basespd:  f32,
}

// =================

struct World {
    canvas:       web::Element,
    vwpsize:      (u32, u32),
    context:      stdweb::Value,
    fps:          f64,

    input:        KeyState,
    ball_state:   BallState,
    paddle_state: PaddleState,
}

impl World {
    fn new() -> World {
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
            input: KeyState::new(),
            ball_state: BallState {
                diameter: 0.0,
                pos:      (0.0, 0.0),
                spd:      (0.0, 0.0),
                stopped:  true,
                basespd:  0.0,
            },
            paddle_state: PaddleState {
                xpos:     0.0,
                ypos:     0.0,
                spd:      0.0,
                basespd:  0.0,
                sz:       (0.0, 0.0),
            },
        };
        
        world.fit_viewport();
        world.paddle_state.xpos = world.vwpsize.0 as f32 / 2.0;
        world.ball_state.diameter = world.vwpsize.1 as f32 * 0.034723;
        world.ball_state.basespd = world.vwpsize.1 as f32 / 72.0;

        world.paddle_state.ypos = 11.0 * world.vwpsize.1 as f32 / 12.0;
        world.paddle_state.basespd = world.vwpsize.1 as f32 / 72.0 * 0.75;
        world.paddle_state.spd = world.paddle_state.basespd;
        world.paddle_state.sz = (world.vwpsize.0 as f32 * 0.12,
                                 world.vwpsize.1 as f32 * 0.034723);
        
        world
    }

    fn clear(&self) {
        js! {
            @{&self.context}.clearRect(0, 0,
                                       @{&self.vwpsize.0},
                                       @{&self.vwpsize.1});
        };
    }
    
    fn draw_box(&self, color: &str, pos: (f32, f32), sz: (f32, f32)) {
        js!{
            @{&self.context}.beginPath();
            @{&self.context}.rect(@{pos.0}, @{pos.1}, @{sz.0}, @{sz.1});
            @{&self.context}.fillStyle = @{color};
            @{&self.context}.fill();
        };
    }

    fn draw_circle(&self, color: &str, pos: (f32, f32), radius: f32) {
        js! {
            @{&self.context}.beginPath();
            @{&self.context}.arc(@{pos.0}, @{pos.1}, @{radius}, 0, Math.PI * 2.0);
            @{&self.context}.fillStyle = @{color};
            @{&self.context}.fill();
            @{&self.context}.closePath();
        };
    }

    fn draw_text(&self, color: &str, align: &str, pos: (f32, f32), text: &str) {
        js! {
            @{&self.context}.fillStyle = @{color};
            @{&self.context}.textAlign = @{align};
            @{&self.context}.fillText(@{text}, @{pos.0}, @{pos.1});
        };
    }

    fn fit_viewport(&self) {
        js!( @{&self.canvas}.width = @{&self.vwpsize.0};
             @{&self.canvas}.height = @{&self.vwpsize.1}; );
    }

    fn input_dispatch(&mut self, key: KeyType, pressed: bool) {
        self.input.new.insert(key, pressed);
    }






    fn update(&mut self, dt: f64) {
        // Process new input
        for (key, state) in &self.input.new {
            match (key, state) {
                (&KeyType::Left,  &true) => self.paddle_state.xpos -= self.paddle_state.spd,
                (&KeyType::Right, &true) => self.paddle_state.xpos += self.paddle_state.spd,
                (&KeyType::S, &true)     => {
                    if self.ball_state.stopped == true {
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
                (&KeyType::A, &true) => self.paddle_state.spd = self.paddle_state.basespd * 2.0,
                (&KeyType::A, &false) => self.paddle_state.spd = self.paddle_state.basespd,
                _ => {},
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
        }

        // Give input to old
        self.input.old = self.input.new.clone(); // Review this. Could it go bad?
    }


    fn render(&self) {
        self.clear();

        // Ball
        let ball_radius = self.ball_state.diameter / 2.0;
        self.draw_circle("white", self.ball_state.pos, ball_radius);
        
        // Paddle
        let paddle_pos = self.paddle_state.xpos - (self.paddle_state.sz.0 / 2.0);
        self.draw_box("white",
                      (paddle_pos, self.paddle_state.ypos),
                      self.paddle_state.sz);

        // FPS
        self.draw_text("white", "left",
                       (ball_radius, ball_radius + 4.0),
                       format!("FPS: {}", f64::floor(self.fps)).as_ref());
    }
}

// =========================

lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(World::new());
}




fn on_key(key: &str, _location: KeyboardLocation, pressed: bool) -> bool {
    match key {
        "ArrowRight" =>
            WORLD.lock().unwrap().input_dispatch(KeyType::Right, pressed),
        "ArrowLeft" =>
            WORLD.lock().unwrap().input_dispatch(KeyType::Left, pressed),
        "s" | " " =>
            WORLD.lock().unwrap().input_dispatch(KeyType::S, pressed),
        "a" =>
            WORLD.lock().unwrap().input_dispatch(KeyType::A, pressed),
        "Enter" =>
            WORLD.lock().unwrap().input_dispatch(KeyType::Enter, pressed),
        _ => {
            js! { console.log("Key " + @{key} + ", state: " + @{pressed}); };
            return false;
        },
    };
    
    /*let location = format!("{:?}", location);
    js!( console.log("Key: " + @{key} +
                     ", location: " + @{location} +
                     ", pressed: " + @{pressed}); );*/
    true
}

fn game_loop(last_call: f64) {
    let now: f64 = js!( return Date.now(); ).try_into().unwrap();
    let dt:  f64 = now - last_call;

    WORLD.lock().unwrap().fps = 1000.0f64 / dt;
    WORLD.lock().unwrap().update(dt);
    WORLD.lock().unwrap().render();

    // Tail recursion. Ha!
    web::window().request_animation_frame( move |_| {
        game_loop(now);
    });
}

fn main() {
    stdweb::initialize();

    WORLD.lock().unwrap().draw_box("red",   (20.0, 20.0), (150.0, 100.0));
    WORLD.lock().unwrap().draw_box("blue",  (40.0, 40.0), (150.0, 100.0));
    WORLD.lock().unwrap().draw_box("green", (60.0, 60.0), (150.0, 100.0));

    // Bind event listeners
    web::window().add_event_listener(|event: KeydownEvent| {
        if on_key(&event.key(), event.location(), true) {
            event.prevent_default();
        }
    });

    web::window().add_event_listener(|event: KeyupEvent| {
        if on_key(&event.key(), event.location(), false) {
            event.prevent_default();
        }
    });

    web::window().request_animation_frame( |_| {
        game_loop(0.0);
    });
    
    stdweb::event_loop();
}