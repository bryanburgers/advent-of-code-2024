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
        pub mod day12 {
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
                    let base4 = arg0;
                    let len4 = arg1;
                    let mut result4 = _rt::Vec::with_capacity(len4);
                    for i in 0..len4 {
                        let base = base4.add(i * 8);
                        let e4 = {
                            let l0 = *base.add(0).cast::<*mut u8>();
                            let l1 = *base.add(4).cast::<usize>();
                            let base3 = l0;
                            let len3 = l1;
                            let mut result3 = _rt::Vec::with_capacity(len3);
                            for i in 0..len3 {
                                let base = base3.add(i * 4);
                                let e3 = {
                                    let l2 = *base.add(0).cast::<i32>();
                                    _rt::char_lift(l2 as u32)
                                };
                                result3.push(e3);
                            }
                            _rt::cabi_dealloc(base3, len3 * 4, 4);
                            result3
                        };
                        result4.push(e4);
                    }
                    _rt::cabi_dealloc(base4, len4 * 8, 4);
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
                        let base = base4.add(i * 8);
                        let e4 = {
                            let l0 = *base.add(0).cast::<*mut u8>();
                            let l1 = *base.add(4).cast::<usize>();
                            let base3 = l0;
                            let len3 = l1;
                            let mut result3 = _rt::Vec::with_capacity(len3);
                            for i in 0..len3 {
                                let base = base3.add(i * 4);
                                let e3 = {
                                    let l2 = *base.add(0).cast::<i32>();
                                    _rt::char_lift(l2 as u32)
                                };
                                result3.push(e3);
                            }
                            _rt::cabi_dealloc(base3, len3 * 4, 4);
                            result3
                        };
                        result4.push(e4);
                    }
                    _rt::cabi_dealloc(base4, len4 * 8, 4);
                    let result5 = T::solve_b(result4);
                    _rt::as_i64(result5)
                }
                pub trait Guest {
                    fn solve_a(input: _rt::Vec<_rt::Vec<char>>) -> u64;
                    fn solve_b(input: _rt::Vec<_rt::Vec<char>>) -> u64;
                }
                #[doc(hidden)]
                macro_rules! __export_aoc2024_day12_solver_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "aoc2024:day12/solver#solve-a"]
                        unsafe extern "C" fn export_solve_a(arg0 : * mut u8, arg1 :
                        usize,) -> i64 { $($path_to_types)*:: _export_solve_a_cabi::<$ty
                        > (arg0, arg1) } #[export_name = "aoc2024:day12/solver#solve-b"]
                        unsafe extern "C" fn export_solve_b(arg0 : * mut u8, arg1 :
                        usize,) -> i64 { $($path_to_types)*:: _export_solve_b_cabi::<$ty
                        > (arg0, arg1) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_aoc2024_day12_solver_cabi;
            }
        }
    }
}
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn char_lift(val: u32) -> char {
        if cfg!(debug_assertions) {
            core::char::from_u32(val).unwrap()
        } else {
            core::char::from_u32_unchecked(val)
        }
    }
    pub use alloc_crate::vec::Vec;
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
        exports::aoc2024::day12::solver::__export_aoc2024_day12_solver_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::aoc2024::day12::solver);
    };
}
#[doc(inline)]
pub(crate) use __export_solver_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:aoc2024:day12-solver:solver:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 276] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x97\x01\x01A\x02\x01\
A\x04\x01B\x02\x01@\x01\x05inputs\x01\0\x04\0\x04info\x01\0\x03\0\x0eaoc:base/de\
bug\x05\0\x01B\x05\x01pt\x01p\0\x01@\x01\x05input\x01\0w\x04\0\x07solve-a\x01\x02\
\x04\0\x07solve-b\x01\x02\x04\0\x14aoc2024:day12/solver\x05\x01\x04\0\x1baoc2024\
:day12-solver/solver\x04\0\x0b\x0c\x01\0\x06solver\x03\0\0\0G\x09producers\x01\x0c\
processed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
