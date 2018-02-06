use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum KeyType {
    Right,
    Left,
    S,
    A,
    Enter,
    FullScreen,
//    LMB,
//    Q,
}

#[derive(PartialEq)]
pub enum OrientationType {
    Unknown,
    Portrait,
    Landscape,
}

#[derive(Clone)]
pub struct DeviceTilt {
    pub abs:   f64,
    pub alpha: f64,
    pub beta:  f64,
    pub gamma: f64,
}

pub struct TiltState {
    pub active: bool,
    pub orient: OrientationType,
    pub async:  DeviceTilt,
    pub old:    DeviceTilt,
    pub new:    DeviceTilt,
}

impl TiltState {
    pub fn new() -> TiltState {
        let tilt = DeviceTilt {
            abs:   0.0,
            alpha: 0.0, // Z = yaw
            beta:  0.0, // X = roll
            gamma: 0.0, // Y = pitch
        };
        
        TiltState {
            active: false,
            orient: OrientationType::Unknown,
            async:  tilt.clone(),
            old:    tilt.clone(),
            new:    tilt.clone(),
        }
    }
}

pub struct KeyState {
    pub async: HashMap<KeyType, bool>,
    pub old:   HashMap<KeyType, bool>,
    pub new:   HashMap<KeyType, bool>,
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState {
            async: HashMap::new(),
            old:   HashMap::new(),
            new:   HashMap::new(),
        }
    }
}
