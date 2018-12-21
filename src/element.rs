// TODO make an element either a pure one or a combined formula


#[derive(Clone)]
pub struct Element {
    pub stickiness: f64,
    pub name: String,
    pub color: [u8; 3],
}

impl Element {
    pub fn hydrogen() -> Element {
        return Element {
            name: String::from("hydrogen"),
            stickiness: 0.6,
            color: [255, 0, 0],
        };
    }

    pub fn oxygen() -> Element {
        return Element {
            name: String::from("oxygen"),
            stickiness: 0.2,
            color: [0, 255, 0],
        };
    }

    pub fn carbon() -> Element {
        return Element {
            name: String::from("carbon"),
            stickiness: 0.34,
            color: [0, 0, 255]
        }
    }
}
