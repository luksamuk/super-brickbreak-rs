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

// ==============================

pub mod input;
pub mod world;


// =========================

lazy_static! {
    static ref WORLD: Mutex<world::World> = Mutex::new(world::World::new());
}




fn on_key(key: &str, _location: KeyboardLocation, pressed: bool) -> bool {
    match key {
        "ArrowRight" =>
            WORLD.lock().unwrap().input_dispatch(input::KeyType::Right, pressed),
        "ArrowLeft" =>
            WORLD.lock().unwrap().input_dispatch(input::KeyType::Left, pressed),
        "s" | " " =>
            WORLD.lock().unwrap().input_dispatch(input::KeyType::S, pressed),
        "a" =>
            WORLD.lock().unwrap().input_dispatch(input::KeyType::A, pressed),
        "Enter" =>
            WORLD.lock().unwrap().input_dispatch(input::KeyType::Enter, pressed),
        "F4" => {
            // Fullscreen toggling can only be done by an user-generated
            // event, so we have to dispatch the keystate and then check
            // for keypresses using async vs. new, instead of new vs. old.
            // It is a hack, but it is also effective
            WORLD.lock().unwrap().input_dispatch(input::KeyType::FullScreen, pressed);
            WORLD.lock().unwrap().toggle_fullscreen();
        },
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

fn on_device_tilt(absolute: f64, alpha: f64, beta: f64, gamma: f64) {
    let mut world = WORLD.lock().unwrap();
    world.tilt.active = true;

    // Convert to radians
    world.tilt.async.abs = absolute.to_radians();
    world.tilt.async.alpha = alpha.to_radians();
    world.tilt.async.beta = beta.to_radians();
    world.tilt.async.gamma = gamma.to_radians();
}

fn on_touch(pressed: bool) {
    // WIP
    WORLD.lock().unwrap().input_dispatch(input::KeyType::S, pressed);
}


fn game_loop(last_call: f64) {
    let now: f64 = js!( return Date.now(); ).try_into().unwrap();
    let dt:  f64 = now - last_call;

    // World handling only on this scope.
    {
        let mut world = WORLD.lock().unwrap();

        world.fps = 1000.0f64 / dt;
        world.update(dt);
        world.render();
    }

    // Tail recursion. Ha!
    web::window().request_animation_frame( move |_| {
        game_loop(now);
    });
}

fn main() {
    stdweb::initialize();

    //WORLD.lock().unwrap().draw_box("red",   (20.0, 20.0), (150.0, 100.0));
    //WORLD.lock().unwrap().draw_box("blue",  (40.0, 40.0), (150.0, 100.0));
    //WORLD.lock().unwrap().draw_box("green", (60.0, 60.0), (150.0, 100.0));

    // Bind event listeners
    // Key down event
    web::window().add_event_listener(|event: KeydownEvent| {
        if on_key(&event.key(), event.location(), true) {
            event.prevent_default();
        }
    });
    
    // Key up event
    web::window().add_event_listener(|event: KeyupEvent| {
        if on_key(&event.key(), event.location(), false) {
            event.prevent_default();
        }
    });

    // Device orientation event.
    // Needs to be done in pure JS, since we still don't have Rust
    // bindings...
    js! {
        // Expose handler functions
        Module.exports.deviceTiltCallback  = @{on_device_tilt};
        Module.exports.deviceTouchCallback = @{on_touch};

        // Outsource events to WASM framework by using an event listener
        /*if (@{web::window()}.DeviceOrientationEvent) {
            @{web::window()}.addEventListener("deviceorientation", function (e) {
                e.preventDefault();

                // Sorry, I know this is horrible, but this is the only
                // way I found to "cast" these values to floats in JS.
                // Damn untyped languages.
                var abs = 0.0;
                var alpha = 0.0;
                var beta = 0.0;
                var gamma = 0.0;
                
                abs += e.absolute;
                alpha += e.alpha;
                beta += e.beta;
                gamma += e.gamma;
                
                Module.exports.deviceTiltCallback(abs, alpha, beta, gamma);
            }, false);
        }*/
        if (@{web::window()}.DeviceMotionEvent) {
            @{web::window()}.addEventListener("devicemotion", function (e) {
                e.preventDefault();

                // Same as above, but I didn't test these.
                // Might not be needed.
                var alpha = 0.0;
                var beta = 0.0;
                var gamma = 0.0;
                
                /*alpha += e.acceleration.z * 2.0;
                beta += e.acceleration.x * 2.0;
                gamma += e.acceleration.y * 2.0;*/
                alpha += e.rotationRate.alpha;
                beta += e.rotationRate.beta;
                gamma += e.rotationRate.gamma;
                
                Module.exports.deviceTiltCallback(0.0,
                                                  alpha,
                                                  beta,
                                                  gamma);
            }, false);
        } else {
            alert("Sorry, your phone sucks");
        }

        if (@{web::window()}.TouchEvent) {
            @{web::window()}.addEventListener("touchstart", (e) => {
                Module.exports.deviceTouchCallback(true);
            }, false);

            @{web::window()}.addEventListener("touchend", (e) => {
                Module.exports.deviceTouchCallback(false);
            }, false);
        }
    };

    // This starts game loop by calling it on the
    // next available animation frame
    web::window().request_animation_frame( |_| {
        game_loop(0.0);
    });
    
    stdweb::event_loop();
}
