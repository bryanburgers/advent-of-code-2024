#[allow(dead_code)]
pub mod aoc2024 {
    #[allow(dead_code)]
    pub mod day01 {
        #[allow(dead_code, clippy::all)]
        pub mod solver {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            pub fn solve_a(input: &[(i32, i32)]) -> i32 {
                unsafe {
                    let vec0 = input;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "aoc2024:day01/solver")]
                    extern "C" {
                        #[link_name = "solve-a"]
                        fn wit_import(_: *mut u8, _: usize) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(ptr0.cast_mut(), len0);
                    ret
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn solve_b(input: &[(i32, i32)]) -> i32 {
                unsafe {
                    let vec0 = input;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "aoc2024:day01/solver")]
                    extern "C" {
                        #[link_name = "solve-b"]
                        fn wit_import(_: *mut u8, _: usize) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(ptr0.cast_mut(), len0);
                    ret
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod aoc {
        #[allow(dead_code)]
        pub mod base {
            #[allow(dead_code, clippy::all)]
            pub mod day {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_run_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::run(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let (t3_0, t3_1) = result1;
                    let vec4 = (t3_0.into_bytes()).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr2.add(4).cast::<usize>() = len4;
                    *ptr2.add(0).cast::<*mut u8>() = ptr4.cast_mut();
                    let vec5 = (t3_1.into_bytes()).into_boxed_slice();
                    let ptr5 = vec5.as_ptr().cast::<u8>();
                    let len5 = vec5.len();
                    ::core::mem::forget(vec5);
                    *ptr2.add(12).cast::<usize>() = len5;
                    *ptr2.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_run<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                    let l2 = *arg0.add(8).cast::<*mut u8>();
                    let l3 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                }
                pub trait Guest {
                    fn run(input: _rt::String) -> (_rt::String, _rt::String);
                }
                #[doc(hidden)]
                macro_rules! __export_aoc_base_day_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "aoc:base/day#run"] unsafe
                        extern "C" fn export_run(arg0 : * mut u8, arg1 : usize,) -> * mut
                        u8 { $($path_to_types)*:: _export_run_cabi::<$ty > (arg0, arg1) }
                        #[export_name = "cabi_post_aoc:base/day#run"] unsafe extern "C"
                        fn _post_return_run(arg0 : * mut u8,) { $($path_to_types)*::
                        __post_return_run::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_aoc_base_day_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
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
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
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
macro_rules! __export_parser_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::aoc::base::day::__export_aoc_base_day_cabi!($ty with_types_in
        $($path_to_types_root)*:: exports::aoc::base::day);
    };
}
#[doc(inline)]
pub(crate) use __export_parser_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:aoc2024:day01-parser:parser:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 280] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x9b\x01\x01A\x02\x01\
A\x04\x01B\x05\x01o\x02zz\x01p\0\x01@\x01\x05input\x01\0z\x04\0\x07solve-a\x01\x02\
\x04\0\x07solve-b\x01\x02\x03\0\x14aoc2024:day01/solver\x05\0\x01B\x03\x01o\x02s\
s\x01@\x01\x05inputs\0\0\x04\0\x03run\x01\x01\x04\0\x0caoc:base/day\x05\x01\x04\0\
\x1baoc2024:day01-parser/parser\x04\0\x0b\x0c\x01\0\x06parser\x03\0\0\0G\x09prod\
ucers\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x06\
0.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
