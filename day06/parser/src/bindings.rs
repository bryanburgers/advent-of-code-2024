#[allow(dead_code)]
pub mod aoc2024 {
    #[allow(dead_code)]
    pub mod day06 {
        #[allow(dead_code, clippy::all)]
        pub mod solver {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Area = _rt::Vec<_rt::Vec<bool>>;
            pub type Position = (u32, u32);
            #[allow(unused_unsafe, clippy::all)]
            pub fn solve_a(area: &Area, position: Position) -> u32 {
                unsafe {
                    let mut cleanup_list = _rt::Vec::new();
                    let vec1 = area;
                    let len1 = vec1.len();
                    let layout1 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec1.len() * 8,
                        4,
                    );
                    let result1 = if layout1.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout1).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout1);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec1.into_iter().enumerate() {
                        let base = result1.add(i * 8);
                        {
                            let vec0 = e;
                            let len0 = vec0.len();
                            let layout0 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec0.len() * 1,
                                1,
                            );
                            let result0 = if layout0.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout0).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout0);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec0.into_iter().enumerate() {
                                let base = result0.add(i * 1);
                                {
                                    *base.add(0).cast::<u8>() = (match e {
                                        true => 1,
                                        false => 0,
                                    }) as u8;
                                }
                            }
                            *base.add(4).cast::<usize>() = len0;
                            *base.add(0).cast::<*mut u8>() = result0;
                            cleanup_list.extend_from_slice(&[(result0, layout0)]);
                        }
                    }
                    let (t2_0, t2_1) = position;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "aoc2024:day06/solver")]
                    extern "C" {
                        #[link_name = "solve-a"]
                        fn wit_import(_: *mut u8, _: usize, _: i32, _: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: i32, _: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(
                        result1,
                        len1,
                        _rt::as_i32(t2_0),
                        _rt::as_i32(t2_1),
                    );
                    if layout1.size() != 0 {
                        _rt::alloc::dealloc(result1.cast(), layout1);
                    }
                    for (ptr, layout) in cleanup_list {
                        if layout.size() != 0 {
                            _rt::alloc::dealloc(ptr.cast(), layout);
                        }
                    }
                    ret as u32
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn solve_b(area: &Area, position: Position) -> u32 {
                unsafe {
                    let mut cleanup_list = _rt::Vec::new();
                    let vec1 = area;
                    let len1 = vec1.len();
                    let layout1 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec1.len() * 8,
                        4,
                    );
                    let result1 = if layout1.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout1).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout1);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec1.into_iter().enumerate() {
                        let base = result1.add(i * 8);
                        {
                            let vec0 = e;
                            let len0 = vec0.len();
                            let layout0 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec0.len() * 1,
                                1,
                            );
                            let result0 = if layout0.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout0).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout0);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec0.into_iter().enumerate() {
                                let base = result0.add(i * 1);
                                {
                                    *base.add(0).cast::<u8>() = (match e {
                                        true => 1,
                                        false => 0,
                                    }) as u8;
                                }
                            }
                            *base.add(4).cast::<usize>() = len0;
                            *base.add(0).cast::<*mut u8>() = result0;
                            cleanup_list.extend_from_slice(&[(result0, layout0)]);
                        }
                    }
                    let (t2_0, t2_1) = position;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "aoc2024:day06/solver")]
                    extern "C" {
                        #[link_name = "solve-b"]
                        fn wit_import(_: *mut u8, _: usize, _: i32, _: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: i32, _: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(
                        result1,
                        len1,
                        _rt::as_i32(t2_0),
                        _rt::as_i32(t2_1),
                    );
                    if layout1.size() != 0 {
                        _rt::alloc::dealloc(result1.cast(), layout1);
                    }
                    for (ptr, layout) in cleanup_list {
                        if layout.size() != 0 {
                            _rt::alloc::dealloc(ptr.cast(), layout);
                        }
                    }
                    ret as u32
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
    pub use alloc_crate::vec::Vec;
    pub use alloc_crate::alloc;
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
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
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
#[link_section = "component-type:wit-bindgen:0.35.0:aoc2024:day06-parser:parser:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 316] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xbf\x01\x01A\x02\x01\
A\x04\x01B\x08\x01p\x7f\x01p\0\x04\0\x04area\x03\0\x01\x01o\x02yy\x04\0\x08posit\
ion\x03\0\x03\x01@\x02\x04area\x02\x08position\x04\0y\x04\0\x07solve-a\x01\x05\x04\
\0\x07solve-b\x01\x05\x03\0\x14aoc2024:day06/solver\x05\0\x01B\x03\x01o\x02ss\x01\
@\x01\x05inputs\0\0\x04\0\x03run\x01\x01\x04\0\x0caoc:base/day\x05\x01\x04\0\x1b\
aoc2024:day06-parser/parser\x04\0\x0b\x0c\x01\0\x06parser\x03\0\0\0G\x09producer\
s\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.3\
5.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
