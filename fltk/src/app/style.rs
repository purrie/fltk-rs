use crate::enums::FrameType;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::fl;
use std::{ffi::CString, sync::Mutex};

lazy_static! {
    /// The currently chosen frame type
    pub(crate) static ref CURRENT_FRAME: Mutex<i32> = Mutex::new(2);
}

/// Set the app scheme
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Scheme {
    /// Base fltk scheming
    Base,
    /// inspired by the Aqua user interface on Mac OS X
    Plastic,
    /// inspired by the GTK+ theme
    Gtk,
    /// inspired by the Clearlooks Glossy scheme
    Gleam,
}

/// sets the scheme of the application
pub fn set_scheme(scheme: Scheme) {
    let name_str = match scheme {
        Scheme::Base => "base",
        Scheme::Gtk => "gtk+",
        Scheme::Gleam => "gleam",
        Scheme::Plastic => "plastic",
    };
    let name_str = CString::safe_new(name_str);
    unsafe { fl::Fl_set_scheme(name_str.as_ptr()) }
}

/// Gets the scheme of the application
pub fn scheme() -> Scheme {
    unsafe {
        use Scheme::{Base, Gleam, Gtk, Plastic};
        match fl::Fl_scheme() {
            0 => Base,
            1 => Gtk,
            2 => Gleam,
            3 => Plastic,
            _ => unreachable!(),
        }
    }
}

/// Alias Scheme to `AppScheme`
pub type AppScheme = Scheme;

/// Set the application's scrollbar size
pub fn set_scrollbar_size(sz: i32) {
    unsafe { fl::Fl_set_scrollbar_size(sz as i32) }
}

/// Get the app's scrollbar size
pub fn scrollbar_size() -> i32 {
    unsafe { fl::Fl_scrollbar_size() as i32 }
}

/// Return whether visible focus is shown
pub fn visible_focus() -> bool {
    unsafe { fl::Fl_visible_focus() != 0 }
}

/// Show focus around widgets
pub fn set_visible_focus(flag: bool) {
    unsafe { fl::Fl_set_visible_focus(flag as i32) }
}

/// Set the app's default frame type
pub fn set_frame_type(new_frame: FrameType) {
    unsafe {
        let new_frame = new_frame as i32;
        let mut curr = CURRENT_FRAME.lock().unwrap();
        fl::Fl_set_box_type(*curr, new_frame);
        *curr = new_frame;
    }
}

/// Get the app's frame type
pub fn frame_type() -> FrameType {
    let curr = CURRENT_FRAME.lock().unwrap();
    FrameType::by_index(*curr as _)
}

/// Swap the default frame type with a new frame type
pub fn swap_frame_type(new_frame: FrameType) {
    unsafe {
        let new_frame = new_frame as i32;
        let mut curr = CURRENT_FRAME.lock().unwrap();
        fl::Fl_set_box_type(56, *curr);
        fl::Fl_set_box_type(*curr, new_frame);
        fl::Fl_set_box_type(new_frame, 56);
        *curr = new_frame;
    }
}

/// Makes FLTK use its own colormap. This may make FLTK display better
pub fn own_colormap() {
    unsafe { fl::Fl_own_colormap() }
}

/// Set the foreground color
pub fn foreground(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_foreground(r, g, b) }
}

/// Set the background color
pub fn background(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_background(r, g, b) }
}

/// Set the background color for input and text widgets
pub fn background2(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_background2(r, g, b) }
}

/// Sets the app's default selection color
pub fn set_selection_color(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_selection_color(r, g, b) }
}
/// Sets the app's default selection color
pub fn set_inactive_color(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_inactive_color(r, g, b) }
}

/// Gets the system colors
pub fn get_system_colors() {
    unsafe { fl::Fl_get_system_colors() }
}

/// Reload the app scheme
pub fn reload_scheme() -> Result<(), FltkError> {
    unsafe {
        match fl::Fl_reload_scheme() {
            1 => Ok(()),
            _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
        }
    }
}

/// Get the default menu linespacing
pub fn menu_linespacing() -> i32 {
    unsafe { fl::Fl_menu_linespacing() }
}

/// Set the menu linespacing
pub fn set_menu_linespacing(val: i32) {
    unsafe { fl::Fl_set_menu_linespacing(val) }
}
