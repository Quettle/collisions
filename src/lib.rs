use draw;
use draw::Color;
use draw::Shape;
use draw::Drawing;
use draw::Style;
use fastrand;

#[derive(Debug, PartialEq)]
pub struct Disk {
    pub pos: [f32; 2],
    v: [f32; 2],
    size: f32,
    color: (u8, u8, u8),
}

impl Disk {
    pub fn new() -> Disk {
        let pos = [fastrand::f32(), fastrand::f32()];
        let v = [fastrand::f32(), fastrand::f32()];
        let size = fastrand::f32() * 0.1;
        let color = (0, 0, 0);
        Disk {
            pos,
            v,
            size,
            color,
        }
    }
    pub fn draw(self, canvas: &mut draw::Canvas) {
        let circ = Drawing::new()
            // give it a shape
            .with_shape(Shape::Circle {
                radius: (self.size*20.0).round() as u32,
            })
            // move it around
            .with_xy(25.0, 25.0)
            // give it a cool style
            .with_style(Style::stroked(5, Color::black()));
        canvas.display_list.add(circ);
    }
    pub fn distance(&self, other: &Disk) -> f32 {
        ((self.pos[0] - other.pos[0]).powi(2) + (self.pos[0] - other.pos[0]).powi(2)).sqrt()
    }

    pub fn pusher(&mut self, dt: f32, intensity: f32) {
        for i in 0..2 {
            self.v[i] += dt * intensity * self.pos[i];
        }
        for i in 0..2 {
            self.pos[i] += dt * self.v[i];
        }
    }
}

pub fn intersect(x: &Disk, y: &Disk) -> bool {
    assert_ne!(x, y);
    x.distance(y) < x.size + y.size
}
