use world::state::BallState;

#[derive(PartialEq, PartialOrd, Clone)]
pub struct Collision {
    pub pos:    (f32, f32),
    pub vector: (f32, f32),
    pub valid: bool,
}

impl Collision {
    pub fn new() -> Collision {
        Collision {
            pos:    (0.0, 0.0),
            vector: (0.0, 0.0),
            valid: false,
        }
    }

    pub fn from(position: (f32, f32), vector: (f32, f32)) -> Collision {
        Collision {
            pos: position,
            vector: vector,
            valid: true,
        }
    }

    pub fn collides(ball_state: &BallState, tile_pos: (f32, f32),
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
}
