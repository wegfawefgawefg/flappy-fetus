use glam::{IVec2, UVec2};

pub struct Bounds {
    top_left: IVec2,
    bottom_right: IVec2,
}

pub trait Bounded {
    fn get_pos(&self) -> IVec2;
    fn get_size(&self) -> UVec2;
    fn get_bounds(&self) -> Bounds {
        let pos = self.get_pos();
        let size = self.get_size();
        Bounds {
            top_left: pos,
            bottom_right: pos + size.as_ivec2(),
        }
    }
}

pub fn is_intersection(bounds: &Bounds, other: &Bounds) -> bool {
    if other.bottom_right.x < bounds.top_left.x
        || other.top_left.x > bounds.bottom_right.x
        || other.bottom_right.y < bounds.top_left.y
        || other.top_left.y > bounds.bottom_right.y
    {
        return false;
    }
    true
}
