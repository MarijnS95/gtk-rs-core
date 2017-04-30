// This file was generated by gir (14876d3) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use Stack;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct StackSidebar(Object<ffi::GtkStackSidebar>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_stack_sidebar_get_type(),
    }
}

impl StackSidebar {
    #[cfg(feature = "v3_16")]
    pub fn new() -> StackSidebar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_stack_sidebar_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_stack(&self) -> Option<Stack> {
        unsafe {
            from_glib_none(ffi::gtk_stack_sidebar_get_stack(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_stack(&self, stack: &Stack) {
        unsafe {
            ffi::gtk_stack_sidebar_set_stack(self.to_glib_none().0, stack.to_glib_none().0);
        }
    }

    pub fn get_property_stack(&self) -> Option<Stack> {
        let mut value = Value::from(None::<&Stack>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stack".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_stack(&self, stack: Option<&Stack>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stack".to_glib_none().0, Value::from(stack).to_glib_none().0);
        }
    }
}
