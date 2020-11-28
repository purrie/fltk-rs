/* automatically generated by rust-bindgen */

extern "C" {
    pub fn Fl_set_color_int(c: libc::c_uint);
}
extern "C" {
    pub fn Fl_set_color_rgb(r: libc::c_uchar, g: libc::c_uchar, b: libc::c_uchar);
}
extern "C" {
    pub fn Fl_get_color() -> libc::c_uint;
}
extern "C" {
    pub fn Fl_push_clip(x: libc::c_int, y: libc::c_int, w: libc::c_int, h: libc::c_int);
}
extern "C" {
    pub fn Fl_push_no_clip();
}
extern "C" {
    pub fn Fl_pop_clip();
}
extern "C" {
    pub fn Fl_not_clipped(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn Fl_clip_box(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        X: *mut libc::c_int,
        Y: *mut libc::c_int,
        W: *mut libc::c_int,
        H: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn Fl_restore_clip();
}
extern "C" {
    pub fn Fl_set_clip_region(r: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_clip_region() -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_point(x: libc::c_int, y: libc::c_int);
}
extern "C" {
    pub fn Fl_line_style(style: libc::c_int, width: libc::c_int, dashes: *mut libc::c_char);
}
extern "C" {
    pub fn Fl_rect(x: libc::c_int, y: libc::c_int, w: libc::c_int, h: libc::c_int);
}
extern "C" {
    pub fn Fl_focus_rect(x: libc::c_int, y: libc::c_int, w: libc::c_int, h: libc::c_int);
}
extern "C" {
    pub fn Fl_rect_with_color(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        c: libc::c_uint,
    );
}
extern "C" {
    pub fn Fl_rectf(x: libc::c_int, y: libc::c_int, w: libc::c_int, h: libc::c_int);
}
extern "C" {
    pub fn Fl_rectf_with_color(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        c: libc::c_uint,
    );
}
extern "C" {
    pub fn Fl_rectf_with_rgb(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        r: libc::c_uchar,
        g: libc::c_uchar,
        b: libc::c_uchar,
    );
}
extern "C" {
    pub fn Fl_line(x: libc::c_int, y: libc::c_int, x1: libc::c_int, y1: libc::c_int);
}
extern "C" {
    pub fn Fl_line2(
        x: libc::c_int,
        y: libc::c_int,
        x1: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y2: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_loop(
        x: libc::c_int,
        y: libc::c_int,
        x1: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y2: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_loop2(
        x: libc::c_int,
        y: libc::c_int,
        x1: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y2: libc::c_int,
        x3: libc::c_int,
        y3: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_polygon(
        x: libc::c_int,
        y: libc::c_int,
        x1: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y2: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_polygon2(
        x: libc::c_int,
        y: libc::c_int,
        x1: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y2: libc::c_int,
        x3: libc::c_int,
        y3: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_xyline(x: libc::c_int, y: libc::c_int, x1: libc::c_int);
}
extern "C" {
    pub fn Fl_xyline2(x: libc::c_int, y: libc::c_int, x1: libc::c_int, y2: libc::c_int);
}
extern "C" {
    pub fn Fl_xyline3(
        x: libc::c_int,
        y: libc::c_int,
        x1: libc::c_int,
        y2: libc::c_int,
        x3: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_yxline(x: libc::c_int, y: libc::c_int, y1: libc::c_int);
}
extern "C" {
    pub fn Fl_yxline2(x: libc::c_int, y: libc::c_int, y1: libc::c_int, x2: libc::c_int);
}
extern "C" {
    pub fn Fl_yxline3(
        x: libc::c_int,
        y: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y3: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_arc(x: libc::c_int, y: libc::c_int, w: libc::c_int, h: libc::c_int, a1: f64, a2: f64);
}
extern "C" {
    pub fn Fl_pie(x: libc::c_int, y: libc::c_int, w: libc::c_int, h: libc::c_int, a1: f64, a2: f64);
}
extern "C" {
    pub fn Fl_push_matrix();
}
extern "C" {
    pub fn Fl_pop_matrix();
}
extern "C" {
    pub fn Fl_scale(x: f64, y: f64);
}
extern "C" {
    pub fn Fl_scale2(x: f64);
}
extern "C" {
    pub fn Fl_translate(x: f64, y: f64);
}
extern "C" {
    pub fn Fl_rotate(d: f64);
}
extern "C" {
    pub fn Fl_mult_matrix(a: f64, b: f64, c: f64, d: f64, x: f64, y: f64);
}
extern "C" {
    pub fn Fl_begin_points();
}
extern "C" {
    pub fn Fl_begin_line();
}
extern "C" {
    pub fn Fl_begin_loop();
}
extern "C" {
    pub fn Fl_begin_polygon();
}
extern "C" {
    pub fn Fl_vertex(x: f64, y: f64);
}
extern "C" {
    pub fn Fl_curve(X0: f64, Y0: f64, X1: f64, Y1: f64, X2: f64, Y2: f64, X3: f64, Y3: f64);
}
extern "C" {
    pub fn Fl_arc2(x: f64, y: f64, r: f64, start: f64, end: f64);
}
extern "C" {
    pub fn Fl_circle(x: f64, y: f64, r: f64);
}
extern "C" {
    pub fn Fl_end_points();
}
extern "C" {
    pub fn Fl_end_line();
}
extern "C" {
    pub fn Fl_end_loop();
}
extern "C" {
    pub fn Fl_end_polygon();
}
extern "C" {
    pub fn Fl_begin_complex_polygon();
}
extern "C" {
    pub fn Fl_gap();
}
extern "C" {
    pub fn Fl_end_complex_polygon();
}
extern "C" {
    pub fn Fl_transform_x(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transform_y(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transform_dx(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transform_dy(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transformed_vertex(xf: f64, yf: f64);
}
extern "C" {
    pub fn Fl_end_offscreen();
}
extern "C" {
    pub fn Fl_set_draw_font(face: libc::c_int, fsize: libc::c_int);
}
extern "C" {
    pub fn Fl_font() -> libc::c_int;
}
extern "C" {
    pub fn Fl_size() -> libc::c_int;
}
extern "C" {
    pub fn Fl_height() -> libc::c_int;
}
extern "C" {
    pub fn Fl_set_height(font: libc::c_int, size: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn Fl_descent() -> libc::c_int;
}
extern "C" {
    pub fn Fl_width(txt: *const libc::c_char) -> f64;
}
extern "C" {
    pub fn Fl_width2(txt: *const libc::c_char, n: libc::c_int) -> f64;
}
extern "C" {
    pub fn Fl_width3(c: libc::c_uint) -> f64;
}
extern "C" {
    pub fn Fl_text_extents(
        arg1: *const libc::c_char,
        dx: *mut libc::c_int,
        dy: *mut libc::c_int,
        w: *mut libc::c_int,
        h: *mut libc::c_int,
    );
}
extern "C" {
    pub fn Fl_text_extents2(
        t: *const libc::c_char,
        n: libc::c_int,
        dx: *mut libc::c_int,
        dy: *mut libc::c_int,
        w: *mut libc::c_int,
        h: *mut libc::c_int,
    );
}
extern "C" {
    pub fn Fl_latin1_to_local(t: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
}
extern "C" {
    pub fn Fl_local_to_latin1(t: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
}
extern "C" {
    pub fn Fl_mac_roman_to_local(t: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
}
extern "C" {
    pub fn Fl_local_to_mac_roman(t: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
}
extern "C" {
    pub fn Fl_draw(str: *const libc::c_char, x: libc::c_int, y: libc::c_int);
}
extern "C" {
    pub fn Fl_draw2(angle: libc::c_int, str: *const libc::c_char, x: libc::c_int, y: libc::c_int);
}
extern "C" {
    pub fn Fl_draw3(str: *const libc::c_char, n: libc::c_int, x: libc::c_int, y: libc::c_int);
}
extern "C" {
    pub fn Fl_draw4(
        angle: libc::c_int,
        str: *const libc::c_char,
        n: libc::c_int,
        x: libc::c_int,
        y: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_rtl_draw(str: *const libc::c_char, n: libc::c_int, x: libc::c_int, y: libc::c_int);
}
extern "C" {
    pub fn Fl_measure(
        str: *const libc::c_char,
        x: *mut libc::c_int,
        y: *mut libc::c_int,
        draw_symbols: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_draw5(
        str: *const libc::c_char,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        align: libc::c_int,
        img: *mut *mut libc::c_void,
        draw_symbols: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_frame(
        s: *const libc::c_char,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_frame2(
        s: *const libc::c_char,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_draw_box(
        box_type: libc::c_int,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        arg1: libc::c_uint,
    );
}
extern "C" {
    pub fn Fl_draw_image(
        buf: *const libc::c_uchar,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
        D: libc::c_int,
        L: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_draw_image_mono(
        buf: *const libc::c_uchar,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
        D: libc::c_int,
        L: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_can_do_alpha_blending() -> libc::c_char;
}
extern "C" {
    pub fn Fl_read_image(
        p: *mut libc::c_uchar,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
        alpha: libc::c_int,
    ) -> *mut libc::c_uchar;
}
extern "C" {
    pub fn Fl_capture_window_part(
        win: *mut libc::c_void,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> *mut libc::c_uchar;
}
extern "C" {
    pub fn Fl_draw_pixmap(
        data: *const *const libc::c_char,
        x: libc::c_int,
        y: libc::c_int,
        bg: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn Fl_draw_pixmap2(
        data: *const *mut libc::c_char,
        x: libc::c_int,
        y: libc::c_int,
        bg: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn Fl_measure_pixmap(
        data: *const *mut libc::c_char,
        w: *mut libc::c_int,
        h: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn Fl_measure_pixmap2(
        cdata: *const *const libc::c_char,
        w: *mut libc::c_int,
        h: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn Fl_shortcut_label(shortcut: libc::c_uint) -> *const libc::c_char;
}
extern "C" {
    pub fn Fl_shortcut_label2(
        shortcut: libc::c_uint,
        eom: *mut *const libc::c_char,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn Fl_old_shortcut(s: *const libc::c_char) -> libc::c_uint;
}
extern "C" {
    pub fn Fl_overlay_rect(x: libc::c_int, y: libc::c_int, w: libc::c_int, h: libc::c_int);
}
extern "C" {
    pub fn Fl_overlay_clear();
}
extern "C" {
    pub fn Fl_set_cursor(cursor: libc::c_int);
}
extern "C" {
    pub fn Fl_set_cursor2(cursor: libc::c_int, fg: libc::c_int, bg: libc::c_int);
}
extern "C" {
    pub fn Fl_expand_text(
        from: *const libc::c_char,
        buf: *mut libc::c_char,
        maxbuf: libc::c_int,
        maxw: f64,
        n: *mut libc::c_int,
        width: *mut f64,
        wrap: libc::c_int,
        draw_symbols: libc::c_int,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn Fl_set_status(X: libc::c_int, Y: libc::c_int, W: libc::c_int, H: libc::c_int);
}
extern "C" {
    pub fn Fl_set_spot(
        font: libc::c_int,
        size: libc::c_int,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
        win: *mut libc::c_void,
    );
}
extern "C" {
    pub fn Fl_reset_spot();
}
extern "C" {
    pub fn Fl_raw_image_to_png(
        data: *mut libc::c_uchar,
        fname: *const libc::c_char,
        w: libc::c_int,
        h: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn Fl_raw_image_to_jpg(
        data: *mut libc::c_uchar,
        fname: *const libc::c_char,
        w: libc::c_int,
        h: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn Fl_raw_image_to_bmp(
        data: *mut libc::c_uchar,
        fname: *const libc::c_char,
        w: libc::c_int,
        h: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn Fl_show_colormap(old_col: libc::c_uint) -> libc::c_uint;
}
extern "C" {
    pub fn Fl_copy_offscreen(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        pixmap: *mut libc::c_void,
        srcx: libc::c_int,
        srcy: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_create_offscreen(w: libc::c_int, h: libc::c_int) -> *mut libc::c_void;
}
extern "C" {
    pub fn Fl_begin_offscreen(b: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_delete_offscreen(bitmap: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_rescale_offscreen(ctx: *mut libc::c_void);
}
extern "C" {
    pub fn Fl_draw_text2(
        str: *const libc::c_char,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        align: libc::c_int,
    );
}
