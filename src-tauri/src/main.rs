// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tkl_snap_lib::capture::{get_image_inner, GlobalState};


use std::sync::Arc;


#[cfg(test)]
mod tests  {
    use super::*;

    #[test]
    fn test_inner() {
        let _ = get_image_inner();
    }
}


fn main() {
    tkl_snap_lib::run();
}
