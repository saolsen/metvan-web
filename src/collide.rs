use super::Aabb;

// @TODO: Maybe a bad name becuase it isn't a rust Result type.
pub struct SweepResult {
    pub hit: bool,
    pub hit_time: f32,
    pub hit_normal: glm::Vec2,
}

pub fn sweep_aabb(a: &Aabb, b: &Aabb, ray: &glm::Vec2, dt: f32) -> SweepResult {
    let mut hit = false;
    let mut hit_time = std::f32::INFINITY;
    let mut hit_normal = glm::vec2(0.0, 0.0);
    let minkowski_extent = a.extent + b.extent;
    let min = b.center - minkowski_extent;
    let max = b.center + minkowski_extent;
    let o = a.center;

    // tile top
    if ray.y < 0.0 {
        let t = (max.y - o.y) / ray.y;
        if t > 0.0 && t < dt {
            let x = o.x + ray.x * t;
            if x >= min.x && x <= max.x {
                if t < hit_time {
                    hit = true;
                    hit_time = t;
                    hit_normal = glm::vec2(0.0, 1.0);
                }
            }
        }
    }

    // tile bottom
    if ray.y > 0.0 {
        let t = (min.y - o.y) / ray.y;
        if t > 0.0 && t < dt {
            let x = o.x + ray.x * t;
            if x >= min.x && x <= max.x {
                if t < hit_time {
                    hit = true;
                    hit_time = t;
                    hit_normal = glm::vec2(0.0, -1.0);
                }
            }
        }
    }

    // tile left
    if ray.x > 0.0 {
        let t = (min.x - o.x) / ray.x;
        if t > 0.0 && t < dt {
            let y = o.y + ray.y * t;
            if y >= min.y && y <= max.y {
                if t < hit_time {
                    hit = true;
                    hit_time = t;
                    hit_normal = glm::vec2(-1.0, 0.0);
                }
            }
        }
    }

    // tile right
    if ray.x < 0.0 {
        let t = (max.x - o.x) / ray.x;
        if t > 0.0 && t < dt {
            let y = o.y + ray.y * t;
            if y >= min.y && y <= max.y {
                if t < hit_time {
                    hit = true;
                    hit_time = t;
                    hit_normal = glm::vec2(1.0, 0.0);
                }
            }
        }
    }

    SweepResult {
        hit,
        hit_time,
        hit_normal,
    }
}

pub fn test_aabb(a: &Aabb, b: &Aabb) -> bool {
    if ((a.center.x - b.center.x).abs() > (a.extent.x + b.extent.x))
        || ((a.center.y - b.center.y).abs() > (a.extent.y + b.extent.y))
    {
        false
    } else {
        true
    }
}
