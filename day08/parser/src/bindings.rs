#[allow(dead_code)]
pub mod aoc2024 {
    #[allow(dead_code)]
    pub mod day08 {
        #[allow(dead_code, clippy::all)]
        pub mod solver {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Position {
                pub x: i32,
                pub y: i32,
            }
            impl ::core::fmt::Debug for Position {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Position")
                        .field("x", &self.x)
                        .field("y", &self.y)
                        .finish()
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Antena {
                pub frequency: char,
                pub position: Position,
            }
            impl ::core::fmt::Debug for Antena {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Antena")
                        .field("frequency", &self.frequency)
                        .field("position", &self.position)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct Input {
                pub area_width: i32,
                pub area_height: i32,
                pub antenas: _rt::Vec<Antena>,
            }
            impl ::core::fmt::Debug for Input {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Input")
                        .field("area-width", &self.area_width)
                        .field("area-height", &self.area_height)
                        .field("antenas", &self.antenas)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn solve_a(input: &Input) -> u64 {
                unsafe {
                    let Input {
                        area_width: area_width0,
                        area_height: area_height0,
                        antenas: antenas0,
                    } = input;
                    let vec3 = antenas0;
                    let len3 = vec3.len();
                    let layout3 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec3.len() * 12,
                        4,
                    );
                    let result3 = if layout3.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout3);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec3.into_iter().enumerate() {
                        let base = result3.add(i * 12);
                        {
                            let Antena { frequency: frequency1, position: position1 } = e;
                            *base.add(0).cast::<i32>() = _rt::as_i32(frequency1);
                            let Position { x: x2, y: y2 } = position1;
                            *base.add(4).cast::<i32>() = _rt::as_i32(x2);
                            *base.add(8).cast::<i32>() = _rt::as_i32(y2);
                        }
                    }
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "aoc2024:day08/solver")]
                    extern "C" {
                        #[link_name = "solve-a"]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import(
                        _rt::as_i32(area_width0),
                        _rt::as_i32(area_height0),
                        result3,
                        len3,
                    );
                    if layout3.size() != 0 {
                        _rt::alloc::dealloc(result3.cast(), layout3);
                    }
                    ret as u64
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn solve_b(input: &Input) -> u64 {
                unsafe {
                    let Input {
                        area_width: area_width0,
                        area_height: area_height0,
                        antenas: antenas0,
                    } = input;
                    let vec3 = antenas0;
                    let len3 = vec3.len();
                    let layout3 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec3.len() * 12,
                        4,
                    );
                    let result3 = if layout3.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout3);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec3.into_iter().enumerate() {
                        let base = result3.add(i * 12);
                        {
                            let Antena { frequency: frequency1, position: position1 } = e;
                            *base.add(0).cast::<i32>() = _rt::as_i32(frequency1);
                            let Position { x: x2, y: y2 } = position1;
                            *base.add(4).cast::<i32>() = _rt::as_i32(x2);
                            *base.add(8).cast::<i32>() = _rt::as_i32(y2);
                        }
                    }
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "aoc2024:day08/solver")]
                    extern "C" {
                        #[link_name = "solve-b"]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import(
                        _rt::as_i32(area_width0),
                        _rt::as_i32(area_height0),
                        result3,
                        len3,
                    );
                    if layout3.size() != 0 {
                        _rt::alloc::dealloc(result3.cast(), layout3);
                    }
                    ret as u64
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
                    match t3_1 {
                        Some(e) => {
                            *ptr2.add(8).cast::<u8>() = (1i32) as u8;
                            let vec5 = (e.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr2.add(16).cast::<usize>() = len5;
                            *ptr2.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                        None => {
                            *ptr2.add(8).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_run<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                    let l2 = i32::from(*arg0.add(8).cast::<u8>());
                    match l2 {
                        0 => {}
                        _ => {
                            let l3 = *arg0.add(12).cast::<*mut u8>();
                            let l4 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                        }
                    }
                }
                pub trait Guest {
                    fn run(input: _rt::String) -> (_rt::String, Option<_rt::String>);
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
                struct _RetArea([::core::mem::MaybeUninit<u8>; 20]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 20],
                );
            }
        }
    }
}
mod _rt {
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
    pub use alloc_crate::alloc;
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
#[link_section = "component-type:wit-bindgen:0.35.0:aoc2024:day08-parser:parser:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 385] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x84\x02\x01A\x02\x01\
A\x04\x01B\x0a\x01r\x02\x01xz\x01yz\x04\0\x08position\x03\0\0\x01r\x02\x09freque\
ncyt\x08position\x01\x04\0\x06antena\x03\0\x02\x01p\x03\x01r\x03\x0aarea-widthz\x0b\
area-heightz\x07antenas\x04\x04\0\x05input\x03\0\x05\x01@\x01\x05input\x06\0w\x04\
\0\x07solve-a\x01\x07\x04\0\x07solve-b\x01\x07\x03\0\x14aoc2024:day08/solver\x05\
\0\x01B\x04\x01ks\x01o\x02s\0\x01@\x01\x05inputs\0\x01\x04\0\x03run\x01\x02\x04\0\
\x0caoc:base/day\x05\x01\x04\0\x1baoc2024:day08-parser/parser\x04\0\x0b\x0c\x01\0\
\x06parser\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070\
.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
