//! The settings module provides the ability to customize components and various
//! other settings.

mod alignment;
mod color;
mod field;
mod gradient;
mod semantic_color;
mod settings_description;
mod value;

pub use self::{
    alignment::Alignment, color::Color, field::Field, gradient::Gradient,
    semantic_color::SemanticColor, settings_description::SettingsDescription,
    value::{Error as ValueError, Result as ValueResult, Value},
};
