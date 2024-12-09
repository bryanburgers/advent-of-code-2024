#[allow(dead_code)]
pub mod aoc {
    #[allow(dead_code)]
    pub mod base {
        #[allow(dead_code, clippy::all)]
        pub mod debug {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            pub fn info(input: &str) {
                unsafe {
                    let vec0 = input;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "aoc:base/debug")]
                    extern "C" {
                        #[link_name = "info"]
                        fn wit_import(_: *mut u8, _: usize);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0);
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod aoc2024 {
        #[allow(dead_code)]
        pub mod day09 {
            #[allow(dead_code, clippy::all)]
            pub mod solver {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_a_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::solve_a(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                    );
                    _rt::as_i64(result1)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_b_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::solve_b(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                    );
                    _rt::as_i64(result1)
                }
                pub trait Guest {
                    fn solve_a(input: _rt::Vec<u8>) -> u64;
                    fn solve_b(input: _rt::Vec<u8>) -> u64;
                }
                #[doc(hidden)]
                macro_rules! __export_aoc2024_day09_solver_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "aoc2024:day09/solver#solve-a"]
                        unsafe extern "C" fn export_solve_a(arg0 : * mut u8, arg1 :
                        usize,) -> i64 { $($path_to_types)*:: _export_solve_a_cabi::<$ty
                        > (arg0, arg1) } #[export_name = "aoc2024:day09/solver#solve-b"]
                        unsafe extern "C" fn export_solve_b(arg0 : * mut u8, arg1 :
                        usize,) -> i64 { $($path_to_types)*:: _export_solve_b_cabi::<$ty
                        > (arg0, arg1) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_aoc2024_day09_solver_cabi;
            }
        }
    }
}
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
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
        $($path_to_types_root)*::
        exports::aoc2024::day09::solver::__export_aoc2024_day09_solver_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::aoc2024::day09::solver);
    };
}
#[doc(inline)]
pub(crate) use __export_solver_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:aoc2024:day09-solver:solver:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 273] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x94\x01\x01A\x02\x01\
A\x04\x01B\x02\x01@\x01\x05inputs\x01\0\x04\0\x04info\x01\0\x03\0\x0eaoc:base/de\
bug\x05\0\x01B\x04\x01p}\x01@\x01\x05input\0\0w\x04\0\x07solve-a\x01\x01\x04\0\x07\
solve-b\x01\x01\x04\0\x14aoc2024:day09/solver\x05\x01\x04\0\x1baoc2024:day09-sol\
ver/solver\x04\0\x0b\x0c\x01\0\x06solver\x03\0\0\0G\x09producers\x01\x0cprocesse\
d-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
