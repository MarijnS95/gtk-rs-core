// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Initable;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GPowerProfileMonitor")]
    pub struct PowerProfileMonitor(Interface<ffi::GPowerProfileMonitor, ffi::GPowerProfileMonitorInterface>) @requires Initable;

    match fn {
        type_ => || ffi::g_power_profile_monitor_get_type(),
    }
}

impl PowerProfileMonitor {
    #[doc(alias = "g_power_profile_monitor_dup_default")]
    #[doc(alias = "dup_default")]
    pub fn get_default() -> PowerProfileMonitor {
        unsafe { from_glib_full(ffi::g_power_profile_monitor_dup_default()) }
    }
}

pub const NONE_POWER_PROFILE_MONITOR: Option<&PowerProfileMonitor> = None;

pub trait PowerProfileMonitorExt: 'static {
    #[doc(alias = "g_power_profile_monitor_get_power_saver_enabled")]
    #[doc(alias = "get_power_saver_enabled")]
    fn is_power_saver_enabled(&self) -> bool;

    #[cfg(any(feature = "v2_70", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_70")))]
    #[doc(alias = "power-saver-enabled")]
    fn connect_power_saver_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PowerProfileMonitor>> PowerProfileMonitorExt for O {
    fn is_power_saver_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_power_profile_monitor_get_power_saver_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_70", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_70")))]
    fn connect_power_saver_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_power_saver_enabled_trampoline<
            P: IsA<PowerProfileMonitor>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GPowerProfileMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PowerProfileMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::power-saver-enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_power_saver_enabled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PowerProfileMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PowerProfileMonitor")
    }
}
