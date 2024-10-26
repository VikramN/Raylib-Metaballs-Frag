use raylib::math::{Vector2, Vector3};

pub fn random(min : f32, max : f32) -> f32 {
    unsafe {
        let r =  raylib::ffi::GetRandomValue(min as i32, max as i32);
        r as f32
    }
}

#[derive(Clone, Copy)]
pub struct Metaball {
    pub position : Vector2,
    size : f32,
    vel : Vector2,
    limit : Vector2
}

impl Metaball {
    pub fn new(limit: Vector2) -> Metaball {

        let size  = random(5., 30.);
        let position = Vector2 { x : random(size, limit.x - size),  y : random(size, limit.y - size) };

        Metaball {
            position, size,
            vel : Vector2 { x : random(1., 100.) , y: random(1., 100.) },
            limit
        }
    }

    pub fn move_to(self : &mut Self, pos : Vector2) -> &mut Self {
        self.position = pos;
        self
    }

    pub fn to_vec3(self : &Self) -> Vector3 {
        Vector3 {
            x : self.position.x,
            y : self.position.y,
            z : self.size            
        }
    }

    pub fn dist(self : &Metaball, x: f32, y : f32 ) -> f32{

        let p = Vector2 { x , y } ;
        let d = self.position - p;
        let s = d.length_sqr();
        if s == 0.0 { return 0.0; }
        let d = 1.0 * self.size /  s.sqrt();
        return d;
    }

    pub fn update(self : &mut Metaball, elapsed : f32) {
        self.position += self.vel * elapsed;

        if self.position.x >= self.limit.x && self.vel.x > 0. {
            self.vel.x = -1.0 * self.vel.x.abs();
        } else if self.position.x < 0.  && self.vel.x < 0.{
            self.vel.x = self.vel.x.abs();
        }

        if self.position.y >= self.limit.y && self.vel.y > 0. {
            self.vel.y = -1.0 * self.vel.y.abs();
        } else if self.position.y < 0. && self.vel.y < 0. {
            self.vel.y = self.vel.y.abs();
        }
    }
}