// This file was generated by gir (14876d3) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use RevealerTransitionType;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::translate::*;
use gobject_ffi;
use std::mem::transmute;

glib_wrapper! {
    pub struct Revealer(Object<ffi::GtkRevealer>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_revealer_get_type(),
    }
}

impl Revealer {
    #[cfg(feature = "v3_10")]
    pub fn new() -> Revealer {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_revealer_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_child_revealed(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_revealer_get_child_revealed(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_reveal_child(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_revealer_get_reveal_child(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_revealer_get_transition_duration(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_transition_type(&self) -> RevealerTransitionType {
        unsafe {
            from_glib(ffi::gtk_revealer_get_transition_type(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_reveal_child(&self, reveal_child: bool) {
        unsafe {
            ffi::gtk_revealer_set_reveal_child(self.to_glib_none().0, reveal_child.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_revealer_set_transition_duration(self.to_glib_none().0, duration);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_transition_type(&self, transition: RevealerTransitionType) {
        unsafe {
            ffi::gtk_revealer_set_transition_type(self.to_glib_none().0, transition.to_glib());
        }
    }

    pub fn get_property_child_revealed(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "child-revealed".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn get_property_reveal_child(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "reveal-child".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_reveal_child(&self, reveal_child: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "reveal-child".to_glib_none().0, Value::from(&reveal_child).to_glib_none().0);
        }
    }

    pub fn get_property_transition_duration(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-duration".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_transition_duration(&self, transition_duration: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "transition-duration".to_glib_none().0, Value::from(&transition_duration).to_glib_none().0);
        }
    }

    pub fn get_property_transition_type(&self) -> RevealerTransitionType {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    pub fn set_property_transition_type(&self, transition_type: RevealerTransitionType) {
        let transition_type = transition_type.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "transition-type".to_glib_none().0, Value::from(&transition_type).to_glib_none().0);
        }
    }
}
