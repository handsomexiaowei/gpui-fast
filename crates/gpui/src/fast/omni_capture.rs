use crate::{Bounds, EntityId, Pixels, Point, Scene, Window, point, size};
use anyhow::{Context as _, Result};
use std::ops::Range;

#[derive(Clone)]
pub(super) struct RenderCaptureLayer {
    pub(super) owner: EntityId,
    pub(super) bounds: Bounds<Pixels>,
    pub(super) operations: Range<usize>,
}

impl Window {
    pub(crate) fn with_capture_layer(
        &mut self,
        owner: EntityId,
        bounds: Bounds<Pixels>,
        paint: impl FnOnce(&mut Self),
    ) {
        let start = self.next_frame.scene.len();
        paint(self);
        let end = self.next_frame.scene.len();
        self.next_frame.capture_layers.push(RenderCaptureLayer {
            owner,
            bounds,
            operations: start..end,
        });
    }

    /// Whether the window backend supports rendering marked layers to pixels.
    pub fn supports_render_capture(&self) -> bool {
        self.platform_window.supports_render_capture()
    }

    /// Logical window bounds of a marked capture layer in the last frame.
    pub fn capture_layer_bounds(&self, owner: EntityId) -> Option<Bounds<Pixels>> {
        self.rendered_frame
            .capture_layers
            .iter()
            .find(|layer| layer.owner == owner && !layer.operations.is_empty())
            .map(|layer| layer.bounds)
    }

    /// Renders marked, non-overlapping subtrees from the last frame to a
    /// transparent RGBA image cropped to their combined bounds.
    pub fn render_capture(&self, owners: &[EntityId]) -> Result<image::RgbaImage> {
        anyhow::ensure!(self.supports_render_capture(), "Render capture is unavailable");
        let frame = &self.rendered_frame;
        let mut layers = Vec::with_capacity(owners.len());
        for owner in owners {
            let layer = frame
                .capture_layers
                .iter()
                .find(|layer| layer.owner == *owner && !layer.operations.is_empty())
                .context("A requested capture layer is not visible")?;
            layers.push(layer);
        }
        layers.sort_by_key(|layer| layer.operations.start);
        let bounds = layers
            .iter()
            .map(|layer| layer.bounds)
            .reduce(|a, b| a.union(&b))
            .context("No capture layers requested")?
            .intersect(&Bounds::new(Point::default(), self.viewport_size()));
        let scale = self.scale_factor();
        let left = (f32::from(bounds.left()) * scale).round() as i32;
        let top = (f32::from(bounds.top()) * scale).round() as i32;
        let right = (f32::from(bounds.right()) * scale).round() as i32;
        let bottom = (f32::from(bounds.bottom()) * scale).round() as i32;
        anyhow::ensure!(right > left && bottom > top, "Capture bounds are empty");
        let region = Bounds::new(
            point(left.into(), top.into()),
            size((right - left).into(), (bottom - top).into()),
        );
        let mut scene = Scene::default();
        for layer in layers {
            scene.replay(layer.operations.clone(), &frame.scene);
        }
        scene.finish();
        self.platform_window.render_capture(&scene, region)
    }

}
