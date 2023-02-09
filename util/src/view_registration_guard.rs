use axum_live_view::live_view::ViewHandle;

use crate::{ViewId, ViewKind};

/// ViewRegistrationGuard is used to track view components mounting and unmounting.
///
/// When a component is mounted, the ViewRegistrationGuard will notify TBD so that
/// messages from the game can be routed to the view component.
pub struct ViewRegistrationGuard {
    kind: ViewKind,
    /// An identifier that all components in the same View have
    view_id: ViewId,
}

impl ViewRegistrationGuard {
    pub fn new(kind: ViewKind, view_id: &ViewId) -> Self {
        let view_id = view_id.to_owned();
        Self { view_id, kind }
    }

    pub fn mount<V>(&self, _view_handle: ViewHandle<V>) {
        tracing::error!(?self.kind, "ViewLifecycle mounted");
        // Register viewhandle with a bastion child so that messages can be sent to it
    }
}

impl Drop for ViewRegistrationGuard {
    fn drop(&mut self) {
        tracing::error!("ViewLifecycle dropped!");
    }
}
