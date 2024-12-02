#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_solve_a_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> i32 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let result1 = T::solve_a(_rt::Vec::from_raw_parts(arg0.cast(), len0, len0));
    _rt::as_i32(result1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_solve_b_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> i32 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let result1 = T::solve_b(_rt::Vec::from_raw_parts(arg0.cast(), len0, len0));
    _rt::as_i32(result1)
}
pub trait Guest {
    fn solve_a(input: _rt::Vec<(i32, i32)>) -> i32;
    fn solve_b(input: _rt::Vec<(i32, i32)>) -> i32;
}
#[doc(hidden)]
macro_rules! __export_world_solver_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "solve-a"] unsafe extern "C" fn
        export_solve_a(arg0 : * mut u8, arg1 : usize,) -> i32 { $($path_to_types)*::
        _export_solve_a_cabi::<$ty > (arg0, arg1) } #[export_name = "solve-b"] unsafe
        extern "C" fn export_solve_b(arg0 : * mut u8, arg1 : usize,) -> i32 {
        $($path_to_types)*:: _export_solve_b_cabi::<$ty > (arg0, arg1) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_solver_cabi;
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_solver_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_solver_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_solver_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:aoc2024:day01:solver:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 199] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07K\x01A\x02\x01A\x05\x01\
o\x02zz\x01p\0\x01@\x01\x05input\x01\0z\x04\0\x07solve-a\x01\x02\x04\0\x07solve-\
b\x01\x02\x04\0\x14aoc2024:day01/solver\x04\0\x0b\x0c\x01\0\x06solver\x03\0\0\0G\
\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen\
-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
