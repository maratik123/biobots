#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod bot;
pub mod consts;
pub mod draw;
mod geom;
pub mod images;

pub use app::TemplateApp;
pub use geom::{Point, Rect, Size};
