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
        pub mod day07 {
            #[allow(dead_code, clippy::all)]
            pub mod solver {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct CalibrationEquation {
                    pub test_value: u64,
                    pub parameters: _rt::Vec<u64>,
                }
                impl ::core::fmt::Debug for CalibrationEquation {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("CalibrationEquation")
                            .field("test-value", &self.test_value)
                            .field("parameters", &self.parameters)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_a_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let base4 = arg0;
                    let len4 = arg1;
                    let mut result4 = _rt::Vec::with_capacity(len4);
                    for i in 0..len4 {
                        let base = base4.add(i * 16);
                        let e4 = {
                            let l0 = *base.add(0).cast::<i64>();
                            let l1 = *base.add(8).cast::<*mut u8>();
                            let l2 = *base.add(12).cast::<usize>();
                            let len3 = l2;
                            CalibrationEquation {
                                test_value: l0 as u64,
                                parameters: _rt::Vec::from_raw_parts(l1.cast(), len3, len3),
                            }
                        };
                        result4.push(e4);
                    }
                    _rt::cabi_dealloc(base4, len4 * 16, 8);
                    let result5 = T::solve_a(result4);
                    _rt::as_i64(result5)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_b_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let base4 = arg0;
                    let len4 = arg1;
                    let mut result4 = _rt::Vec::with_capacity(len4);
                    for i in 0..len4 {
                        let base = base4.add(i * 16);
                        let e4 = {
                            let l0 = *base.add(0).cast::<i64>();
                            let l1 = *base.add(8).cast::<*mut u8>();
                            let l2 = *base.add(12).cast::<usize>();
                            let len3 = l2;
                            CalibrationEquation {
                                test_value: l0 as u64,
                                parameters: _rt::Vec::from_raw_parts(l1.cast(), len3, len3),
                            }
                        };
                        result4.push(e4);
                    }
                    _rt::cabi_dealloc(base4, len4 * 16, 8);
                    let result5 = T::solve_b(result4);
                    _rt::as_i64(result5)
                }
                pub trait Guest {
                    fn solve_a(input: _rt::Vec<CalibrationEquation>) -> u64;
                    fn solve_b(input: _rt::Vec<CalibrationEquation>) -> u64;
                }
                #[doc(hidden)]
                macro_rules! __export_aoc2024_day07_solver_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "aoc2024:day07/solver#solve-a"]
                        unsafe extern "C" fn export_solve_a(arg0 : * mut u8, arg1 :
                        usize,) -> i64 { $($path_to_types)*:: _export_solve_a_cabi::<$ty
                        > (arg0, arg1) } #[export_name = "aoc2024:day07/solver#solve-b"]
                        unsafe extern "C" fn export_solve_b(arg0 : * mut u8, arg1 :
                        usize,) -> i64 { $($path_to_types)*:: _export_solve_b_cabi::<$ty
                        > (arg0, arg1) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_aoc2024_day07_solver_cabi;
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::vec::Vec;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
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
    pub use alloc_crate::alloc;
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
        exports::aoc2024::day07::solver::__export_aoc2024_day07_solver_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::aoc2024::day07::solver);
    };
}
#[doc(inline)]
pub(crate) use __export_solver_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:aoc2024:day07-solver:solver:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 329] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xcc\x01\x01A\x02\x01\
A\x04\x01B\x02\x01@\x01\x05inputs\x01\0\x04\0\x04info\x01\0\x03\0\x0eaoc:base/de\
bug\x05\0\x01B\x07\x01pw\x01r\x02\x0atest-valuew\x0aparameters\0\x04\0\x14calibr\
ation-equation\x03\0\x01\x01p\x02\x01@\x01\x05input\x03\0w\x04\0\x07solve-a\x01\x04\
\x04\0\x07solve-b\x01\x04\x04\0\x14aoc2024:day07/solver\x05\x01\x04\0\x1baoc2024\
:day07-solver/solver\x04\0\x0b\x0c\x01\0\x06solver\x03\0\0\0G\x09producers\x01\x0c\
processed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
