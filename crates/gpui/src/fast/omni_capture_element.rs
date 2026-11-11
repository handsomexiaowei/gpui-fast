use crate::{
    AnyElement, App, Bounds, Element, ElementId, EntityId, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, Pixels, Window,
};

/// Marks an element subtree as a source for [`Window::render_capture`].
/// Popovers and other deferred draws are excluded from the captured subtree.
pub fn capture_layer(owner: EntityId, child: impl IntoElement) -> CaptureLayer {
    CaptureLayer {
        owner,
        child: child.into_any_element(),
    }
}

/// An element that records its paint range for an explicit render capture.
pub struct CaptureLayer {
    owner: EntityId,
    child: AnyElement,
}

impl IntoElement for CaptureLayer {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for CaptureLayer {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.child.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        window.with_capture_layer(self.owner, bounds, |window| self.child.paint(window, cx));
    }
}
