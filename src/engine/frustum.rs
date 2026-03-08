use cgmath::{InnerSpace, Matrix4, Vector3, Vector4};

pub struct Plane {
    pub normal: Vector3<f32>,
    pub distance: f32,
}

impl Plane {
    fn distance_to_point(&self, point: Vector3<f32>) -> f32 {
        self.normal.dot(point) + self.distance
    }
}

pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn from_matrix(m: Matrix4<f32>) -> Self {
        // cgmath is col-major, so we manually get rows
        let x = Vector4::new(m[0][0], m[1][0], m[2][0], m[3][0]);
        let y = Vector4::new(m[0][1], m[1][1], m[2][1], m[3][1]);
        let z = Vector4::new(m[0][2], m[1][2], m[2][2], m[3][2]);
        let w = Vector4::new(m[0][3], m[1][3], m[2][3], m[3][3]);

        let make_plane = |v: Vector4<f32>| {
            let xyz = Vector3::new(v.x, v.y, v.z);
            let mag = xyz.magnitude();
            Plane { normal: xyz / mag, distance: v.w / mag }
        };

        Self {
            planes: [
                make_plane(w + x), // left
                make_plane(w - x), // right
                make_plane(w + y), // bottom
                make_plane(w - y), // top
                make_plane(z),     // near (standard wgpu [0,1])
                make_plane(w - z), // far
            ],
        }
    }

    pub fn contains_chunk(&self, min: Vector3<f32>, size: f32) -> bool {
        let max = min + Vector3::new(size, size, size);
        
        for plane in &self.planes {
            let mut p = min;
            if plane.normal.x >= 0.0 { p.x = max.x; }
            if plane.normal.y >= 0.0 { p.y = max.y; }
            if plane.normal.z >= 0.0 { p.z = max.z; }

            if plane.distance_to_point(p) < 0.0 {
                return false; // culled
            }
        }
        true
    }
}