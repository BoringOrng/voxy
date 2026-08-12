use bevy::math::{IVec3, UVec3};

pub struct AxisSplit {
    only: Option<(i32, i32)>,
    overlap: (i32, i32),
}

impl AxisSplit {
    pub fn box_diff(a: IVec3, b: IVec3, r: u32) -> Option<Vec<IVec3>> {
        let a_min = a - UVec3::splat(r).as_ivec3();
        let a_max = a + UVec3::splat(r).as_ivec3();

        let b_min = b - UVec3::splat(r).as_ivec3();
        let b_max = b + UVec3::splat(r).as_ivec3();

        let sx = Self::try_new(a_min.x, a_max.x, b_min.x, b_max.x)?;
        let sy = Self::try_new(a_min.y, a_max.y, b_min.y, b_max.y)?;
        let sz = Self::try_new(a_min.z, a_max.z, b_min.z, b_max.z)?;

        let y_min_max = (a_min.y, a_max.y);
        let z_min_max = (a_min.z, a_max.z);

        let mut cells = Vec::new();

        if let Some(x_min_max) = sx.only {
            Self::push_box(&mut cells, x_min_max, y_min_max, z_min_max);
        }

        if let Some(y_min_max) = sy.only {
            Self::push_box(&mut cells, sx.overlap, y_min_max, z_min_max);
        }

        if let Some(z_min_max) = sz.only {
            Self::push_box(&mut cells, sx.overlap, sy.overlap, z_min_max);
        }

        Some(cells)
    }

    fn try_new(a_min: i32, a_max: i32, b_min: i32, b_max: i32) -> Option<Self> {
        let overlap_min = a_min.max(b_min);
        let overlap_max = a_max.min(b_max);

        if overlap_min > overlap_max {
            return None;
        }

        let only = if a_min < b_min {
            Some((a_min, b_min - 1))
        } else if a_max > b_max {
            Some((b_max + 1, a_max))
        } else {
            None
        };

        Some(Self {
            only,
            overlap: (overlap_min, overlap_max),
        })
    }

    fn push_box(out: &mut Vec<IVec3>, x: (i32, i32), y: (i32, i32), z: (i32, i32)) {
        let (x_min, x_max) = x;
        let (y_min, y_max) = y;
        let (z_min, z_max) = z;

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    out.push(IVec3::new(x, y, z));
                }
            }
        }
    }
}
