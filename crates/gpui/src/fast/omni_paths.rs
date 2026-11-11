use crate::{Bounds, Path, ScaledPixels};

impl Path<ScaledPixels> {
    /// Returns whether compositing each path's part of a shared texture
    /// separately would sample every covered pixel at most once.
    pub fn can_composite_separately(paths: &[Self]) -> bool {
        const MAX_PATHS: usize = 64;
        if paths.len() > MAX_PATHS {
            return false;
        }
        let mut bounds = [Bounds::default(); MAX_PATHS];
        for (index, path) in paths.iter().enumerate() {
            let current = path.clipped_bounds();
            if bounds[..index].iter().any(|other| {
                let overlap = current.intersect(other);
                overlap.size.width.0 > 0.0 && overlap.size.height.0 > 0.0
            }) {
                return false;
            }
            bounds[index] = current;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContentMask, Pixels, point, px, size};

    fn path(left: f32, top: f32, width: f32, height: f32) -> Path<ScaledPixels> {
        let mut path = Path::<Pixels>::new(point(px(left), px(top)));
        path.bounds.size = size(px(width), px(height));
        path.content_mask = ContentMask { bounds: path.bounds };
        path.scale(1.)
    }

    #[test]
    fn overlapping_path_sprites_use_one_composite() {
        let a = path(0., 0., 10., 10.);
        assert!(!Path::can_composite_separately(&[
            a.clone(),
            path(9., 8., 10., 10.),
        ]));
        assert!(Path::can_composite_separately(&[
            a.clone(),
            path(10., 8., 10., 10.),
            path(0., 12., 10., 10.),
        ]));
        let mut b = path(5., 1., 10., 10.);
        b.content_mask = path(10., 1., 5., 10.).content_mask;
        assert!(Path::can_composite_separately(&[a, b]));
    }
}
