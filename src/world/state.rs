use stdweb::Value;

pub struct BallState {
    pub sprite:   Value,
    pub diameter: f32,
    pub pos:      (f32, f32),
    pub spd:      (f32, f32),
    pub stopped:  bool,
    pub basespd:  f32,
    pub afterimages: Vec<(f32, f32)>,
}

impl BallState {
    pub fn new() -> BallState {
        BallState {
            sprite:      load_sprite("./sphere.png"),
            diameter:    0.0,
            pos:         (0.0, 0.0),
            spd:         (0.0, 0.0),
            stopped:     true,
            basespd:     0.0,
            afterimages: Vec::with_capacity(7),
        }
    }
}





pub struct PaddleState {
    pub sprite:   Value,
    pub xpos:     f32,
    pub ypos:     f32,
    pub spd:      f32,
    pub sz:       (f32, f32),
    pub basespd:  f32,
}

impl PaddleState {
    pub fn new() -> PaddleState {
        PaddleState {
            sprite:   load_sprite("./paddle.png"),
            xpos:     0.0,
            ypos:     0.0,
            spd:      0.0,
            basespd:  0.0,
            sz:       (0.0, 0.0),
        }
    }
}

pub struct Block {
    pub pos:    (f32, f32),
    pub color:  String,
    pub active: bool,
}




fn load_sprite(path: &'static str) -> Value {
    let sprite = js! {
        var img = new Image();
        img.src = @{path};
        return img;
    };
    sprite
}
