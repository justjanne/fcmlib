extern crate core;

pub mod cut_data;
mod error;
pub mod fcm_file;
pub mod file_header;
pub mod file_type;
mod file_variant;
pub mod generator;
pub mod outline;
mod outline_tag;
pub mod path;
pub mod path_rhinestones;
pub mod path_shape;
pub mod path_tool;
pub mod piece;
pub mod point;
pub mod segment_bezier;
pub mod segment_line;
mod util;

mod alignment_data;
mod encode;
mod piece_restrictions;
mod piece_table;
#[cfg(test)]
mod tests;
