#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod app;

pub mod cipher_panel;
pub mod code_panel;

pub mod pages;

pub mod ui_elements;
