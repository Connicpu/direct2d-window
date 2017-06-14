use winapi::*;

macro_rules! flag_toggle {
    ($($flag_field:ident $flag:ident { $(#[$setter_meta:meta])* fn $setter_name:ident; $(#[$getter_meta:meta])* fn $getter_name:ident; })*) => {
        $(
            $(#[$setter_meta])*
            pub fn $setter_name(mut self, enabled: bool) -> Self {
                if enabled {
                    self.$flag_field |= $flag;
                } else {
                    self.$flag_field &= $flag;
                }
                self
            }
        )*

        $(
            $(#[$getter_meta])*
            pub fn $getter_name(self) -> bool {
                (self.$flag_field & $flag) == $flag
            }
        )*
    }
}

#[derive(Copy, Clone)]
pub struct WindowStyle {
    flag: DWORD,
    flag_ex: DWORD,
}

impl WindowStyle {
    pub fn new() -> Self {
        WindowStyle {
                flag: 0,
                flag_ex: 0,
            }
            .overlapped_window(true)
            .overlapped_window_ex(true)
    }

    flag_toggle! {
        flag WS_BORDER {
            fn border;
            fn has_border;
        }
        flag WS_CAPTION {
            fn caption;
            fn has_caption;
        }
        flag WS_CHILD {
            fn child;
            fn is_child;
        }
        flag WS_CLIPCHILDREN {
            fn clip_children;
            fn get_clip_children;
        }
        flag WS_CLIPSIBLINGS {
            fn clip_siblings;
            fn get_clip_siblings;
        }
        flag WS_DISABLED {
            fn disabled;
            fn is_disabled;
        }
        flag WS_DLGFRAME {
            fn dialog_frame;
            fn is_dialog_frame;
        }
        flag WS_GROUP {
            fn group;
            fn is_group;
        }
        flag WS_HSCROLL {
            fn h_scroll;
            fn has_h_scroll;
        }
        flag WS_VSCROLL {
            fn v_scroll;
            fn has_v_scroll;
        }
        flag WS_MAXIMIZE {
            fn init_maximized;
            fn is_init_maximized;
        }
        flag WS_MINIMIZE {
            fn init_minized;
            fn is_init_minimized;
        }
        flag WS_MAXIMIZEBOX {
            fn maximize_box;
            fn has_maximize_box;
        }
        flag WS_MINIMIZEBOX {
            fn minimize_box;
            fn has_minimize_box;
        }
        flag WS_OVERLAPPEDWINDOW {
            fn overlapped_window;
            fn is_overlapped_window;
        }
        flag WS_POPUP {
            fn popup;
            fn is_popup;
        }
        flag WS_POPUPWINDOW {
            fn popup_window;
            fn is_popup_window;
        }
        flag WS_SIZEBOX {
            fn size_box;
            fn has_size_box;
        }
        flag WS_SYSMENU {
            fn sys_menu;
            fn has_sys_menu;
        }
        flag WS_TABSTOP {
            fn tabstop;
            fn is_tabstop;
        }
        flag WS_VISIBLE {
            fn init_visible;
            fn is_init_visible;
        }
        
        flag_ex WS_EX_ACCEPTFILES {
            fn accept_files;
            fn does_accept_files;
        }
        flag_ex WS_EX_APPWINDOW {
            fn app_window;
            fn is_app_window;
        }
        flag_ex WS_EX_CLIENTEDGE {
            fn client_edge;
            fn has_client_edge;
        }
        flag_ex WS_EX_COMPOSITED {
            fn composited;
            fn is_composited;
        }
        flag_ex WS_EX_CONTEXTHELP {
            fn context_help;
            fn has_context_help;
        }
        flag_ex WS_EX_CONTROLPARENT {
            fn control_parent;
            fn is_control_parent;
        }
        flag_ex WS_EX_DLGMODALFRAME {
            fn dialog_modal_frame;
            fn is_dialog_modal_frame;
        }
        flag_ex WS_EX_LAYERED {
            fn layered;
            fn is_layered;
        }
        flag_ex WS_EX_LAYOUTRTL {
            fn layout_rtl;
            fn is_layout_rtl;
        }
        flag_ex WS_EX_LEFTSCROLLBAR {
            fn left_scrollbar;
            fn has_left_scrollbar;
        }
        flag_ex WS_EX_MDICHILD {
            fn mdi_child;
            fn is_mdi_child;
        }
        flag_ex WS_EX_NOACTIVATE {
            fn no_activate;
            fn is_no_activate;
        }
        flag_ex WS_EX_NOINHERITLAYOUT {
            fn no_inherit_layout;
            fn is_no_inherit_layout;
        }
        flag_ex WS_EX_NOPARENTNOTIFY {
            fn no_parent_notify;
            fn is_no_parent_notify;
        }
        flag_ex WS_EX_NOREDIRECTIONBITMAP {
            fn no_redirection_bitmap;
            fn has_no_redirection_bitmap;
        }
        flag_ex WS_EX_OVERLAPPEDWINDOW {
            fn overlapped_window_ex;
            fn is_overlapped_window_ex;
        }
        flag_ex WS_EX_PALETTEWINDOW {
            fn palette_window;
            fn is_palette_window;
        }
        flag_ex WS_EX_RIGHT {
            fn right;
            fn is_right;
        }
        flag_ex WS_EX_RTLREADING {
            fn rtl_reading;
            fn is_rtl_reading;
        }
        flag_ex WS_EX_STATICEDGE {
            fn static_edge;
            fn has_static_edge;
        }
        flag_ex WS_EX_TOOLWINDOW {
            fn tool_window;
            fn is_tool_window;
        }
        flag_ex WS_EX_TOPMOST {
            fn topmost;
            fn is_topmost;
        }
        flag_ex WS_EX_TRANSPARENT {
            fn transparent;
            fn is_transparent;
        }
        flag_ex WS_EX_WINDOWEDGE {
            fn window_edge;
            fn has_window_edge;
        }
    }
}

impl Default for WindowStyle {
    /// Enables double_clicks, v_redraw, and h_redraw by default
    fn default() -> Self {
        WindowStyle::new()
    }
}

#[derive(Copy, Clone)]
pub struct WindowClassStyle {
    flag: DWORD,
}

impl WindowClassStyle {
    /// Enables double_clicks, v_redraw, and h_redraw by default
    pub fn new() -> Self {
        WindowClassStyle { flag: 0 }
            .double_clicks(true)
            .v_redraw(true)
            .h_redraw(true)
    }

    pub fn style_flags(&self) -> DWORD {
        self.flag
    }

    flag_toggle! {
        flag CS_DBLCLKS {
            /// Sets whether this window should accept double-clicks
            fn double_clicks;
            /// Gets whether this window will accept double-clicks
            fn get_double_clicks;
        }
        flag CS_DROPSHADOW {
            fn drop_shadow;
            fn get_drop_shadow;
        }
        flag CS_HREDRAW {
            fn h_redraw;
            fn get_h_redraw;
        }
        flag CS_VREDRAW {
            fn v_redraw;
            fn get_v_redraw;
        }
        flag CS_NOCLOSE {
            fn no_close;
            fn get_no_close;
        }
    }
}

impl Default for WindowClassStyle {
    /// Enables double_clicks, v_redraw, and h_redraw by default
    fn default() -> Self {
        WindowClassStyle::new()
    }
}
