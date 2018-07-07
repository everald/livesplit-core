//! The layout module provides everything necessary for working with Layouts. A
//! Layout allows you to combine multiple components together to visualize a
//! variety of information the runner is interested in.

mod component;
mod component_settings;
mod component_state;
pub mod editor;
mod general_settings;
mod layout;
mod layout_settings;
mod layout_state;

pub use self::{
    component::Component, component_settings::ComponentSettings, component_state::ComponentState,
    editor::Editor, general_settings::GeneralSettings, layout::Layout,
    layout_settings::LayoutSettings, layout_state::LayoutState,
};
