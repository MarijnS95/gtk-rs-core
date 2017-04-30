// This file was generated by gir (14876d3) from gir-files (71d73f0)
// DO NOT EDIT

use EventController;
use Gesture;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct GestureSingle(Object<ffi::GtkGestureSingle>): Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_single_get_type(),
    }
}

pub trait GestureSingleExt {
    #[cfg(feature = "v3_14")]
    fn get_button(&self) -> u32;

    #[cfg(feature = "v3_14")]
    fn get_current_button(&self) -> u32;

    //#[cfg(feature = "v3_14")]
    //fn get_current_sequence(&self) -> /*Ignored*/Option<gdk::EventSequence>;

    #[cfg(feature = "v3_14")]
    fn get_exclusive(&self) -> bool;

    #[cfg(feature = "v3_14")]
    fn get_touch_only(&self) -> bool;

    #[cfg(feature = "v3_14")]
    fn set_button(&self, button: u32);

    #[cfg(feature = "v3_14")]
    fn set_exclusive(&self, exclusive: bool);

    #[cfg(feature = "v3_14")]
    fn set_touch_only(&self, touch_only: bool);
}

impl<O: IsA<GestureSingle>> GestureSingleExt for O {
    #[cfg(feature = "v3_14")]
    fn get_button(&self) -> u32 {
        unsafe {
            ffi::gtk_gesture_single_get_button(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_current_button(&self) -> u32 {
        unsafe {
            ffi::gtk_gesture_single_get_current_button(self.to_glib_none().0)
        }
    }

    //#[cfg(feature = "v3_14")]
    //fn get_current_sequence(&self) -> /*Ignored*/Option<gdk::EventSequence> {
    //    unsafe { TODO: call ffi::gtk_gesture_single_get_current_sequence() }
    //}

    #[cfg(feature = "v3_14")]
    fn get_exclusive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_single_get_exclusive(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_touch_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_single_get_touch_only(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_button(&self, button: u32) {
        unsafe {
            ffi::gtk_gesture_single_set_button(self.to_glib_none().0, button);
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_exclusive(&self, exclusive: bool) {
        unsafe {
            ffi::gtk_gesture_single_set_exclusive(self.to_glib_none().0, exclusive.to_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_touch_only(&self, touch_only: bool) {
        unsafe {
            ffi::gtk_gesture_single_set_touch_only(self.to_glib_none().0, touch_only.to_glib());
        }
    }
}
