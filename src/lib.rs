#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_fmt_panics)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]
#![allow(unsafe_code)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]

//! 八叉树地图 

// PointCloud模块
pub mod PointCloud;

// ScanGraph模块
pub mod ScanGraph;

// OcTree模块
pub mod OcTree;

// Types模块(使用nalgebra库)
pub mod Types;