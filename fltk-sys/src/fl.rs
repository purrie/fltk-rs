/* automatically generated by rust-bindgen */

pub type Fl_Awake_Handler =
    ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn Fl_run() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_lock() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_unlock();
}
extern "C" {
    pub fn Fl_awake(
        handler: Fl_Awake_Handler,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_key() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_text() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_event_button() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_clicks() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_x() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_y() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_is_click() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_length() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_state() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_screen_h() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_screen_w() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_paste(arg1: *mut ::std::os::raw::c_void, src: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_set_scheme(scheme: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_get_color(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_get_font(idx: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_set_fonts(c: *const ::std::os::raw::c_char) -> ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn Fl_add_handler(
        ev_handler: ::std::option::Option<
            unsafe extern "C" fn(ev: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn Fl_awake_msg(msg: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_thread_msg() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_wait() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_add_timeout(
        t: f64,
        arg1: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_repeat_timeout(
        t: f64,
        arg1: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_remove_timeout(
        arg1: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg2: *mut ::std::os::raw::c_void,
    );
}
