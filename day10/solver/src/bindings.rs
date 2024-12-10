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
pub mod aoc2024 {
    #[allow(dead_code)]
    pub mod day10 {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct TopographicalMap {
                handle: _rt::Resource<TopographicalMap>,
            }
            impl TopographicalMap {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for TopographicalMap {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "aoc2024:day10/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]topographical-map"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl TopographicalMap {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(map: &[_rt::Vec<u8>]) -> Self {
                    unsafe {
                        let vec1 = map;
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
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                *base.add(4).cast::<usize>() = len0;
                                *base.add(0).cast::<*mut u8>() = ptr0.cast_mut();
                            }
                        }
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "aoc2024:day10/types")]
                        extern "C" {
                            #[link_name = "[constructor]topographical-map"]
                            fn wit_import(_: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(result1, len1);
                        if layout1.size() != 0 {
                            _rt::alloc::dealloc(result1.cast(), layout1);
                        }
                        TopographicalMap::from_handle(ret as u32)
                    }
                }
            }
            impl TopographicalMap {
                #[allow(unused_unsafe, clippy::all)]
                pub fn map_width(&self) -> u32 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "aoc2024:day10/types")]
                        extern "C" {
                            #[link_name = "[method]topographical-map.map-width"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u32
                    }
                }
            }
            impl TopographicalMap {
                #[allow(unused_unsafe, clippy::all)]
                pub fn map_height(&self) -> u32 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "aoc2024:day10/types")]
                        extern "C" {
                            #[link_name = "[method]topographical-map.map-height"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u32
                    }
                }
            }
            impl TopographicalMap {
                #[allow(unused_unsafe, clippy::all)]
                pub fn height_at_location(&self, x: u32, y: u32) -> u8 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "aoc2024:day10/types")]
                        extern "C" {
                            #[link_name = "[method]topographical-map.height-at-location"]
                            fn wit_import(_: i32, _: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&x),
                            _rt::as_i32(&y),
                        );
                        ret as u8
                    }
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
        pub mod day10 {
            #[allow(dead_code, clippy::all)]
            pub mod solver {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type TopographicalMap = super::super::super::super::aoc2024::day10::types::TopographicalMap;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_a_cabi<T: Guest>(arg0: i32) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle0;
                    let result1 = T::solve_a({
                        handle0 = super::super::super::super::aoc2024::day10::types::TopographicalMap::from_handle(
                            arg0 as u32,
                        );
                        &handle0
                    });
                    _rt::as_i64(result1)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_b_cabi<T: Guest>(arg0: i32) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let handle0;
                    let result1 = T::solve_b({
                        handle0 = super::super::super::super::aoc2024::day10::types::TopographicalMap::from_handle(
                            arg0 as u32,
                        );
                        &handle0
                    });
                    _rt::as_i64(result1)
                }
                pub trait Guest {
                    fn solve_a(input: &TopographicalMap) -> u64;
                    fn solve_b(input: &TopographicalMap) -> u64;
                }
                #[doc(hidden)]
                macro_rules! __export_aoc2024_day10_solver_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "aoc2024:day10/solver#solve-a"]
                        unsafe extern "C" fn export_solve_a(arg0 : i32,) -> i64 {
                        $($path_to_types)*:: _export_solve_a_cabi::<$ty > (arg0) }
                        #[export_name = "aoc2024:day10/solver#solve-b"] unsafe extern "C"
                        fn export_solve_b(arg0 : i32,) -> i64 { $($path_to_types)*::
                        _export_solve_b_cabi::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_aoc2024_day10_solver_cabi;
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
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
        exports::aoc2024::day10::solver::__export_aoc2024_day10_solver_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::aoc2024::day10::solver);
    };
}
#[doc(inline)]
pub(crate) use __export_solver_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:aoc2024:day10-solver:solver:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 587] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xce\x03\x01A\x02\x01\
A\x07\x01B\x02\x01@\x01\x05inputs\x01\0\x04\0\x04info\x01\0\x03\0\x0eaoc:base/de\
bug\x05\0\x01B\x0c\x04\0\x11topographical-map\x03\x01\x01p}\x01p\x01\x01i\0\x01@\
\x01\x03map\x02\0\x03\x04\0\x1e[constructor]topographical-map\x01\x04\x01h\0\x01\
@\x01\x04self\x05\0y\x04\0#[method]topographical-map.map-width\x01\x06\x04\0$[me\
thod]topographical-map.map-height\x01\x06\x01@\x03\x04self\x05\x01xy\x01yy\0}\x04\
\0,[method]topographical-map.height-at-location\x01\x07\x03\0\x13aoc2024:day10/t\
ypes\x05\x01\x02\x03\0\x01\x11topographical-map\x01B\x06\x02\x03\x02\x01\x02\x04\
\0\x11topographical-map\x03\0\0\x01h\x01\x01@\x01\x05input\x02\0w\x04\0\x07solve\
-a\x01\x03\x04\0\x07solve-b\x01\x03\x04\0\x14aoc2024:day10/solver\x05\x03\x04\0\x1b\
aoc2024:day10-solver/solver\x04\0\x0b\x0c\x01\0\x06solver\x03\0\0\0G\x09producer\
s\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.3\
5.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
