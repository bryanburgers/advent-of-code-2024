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
        pub mod day10 {
            #[allow(dead_code, clippy::all)]
            pub mod types {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct TopographicalMap {
                    handle: _rt::Resource<TopographicalMap>,
                }
                type _TopographicalMapRep<T> = Option<T>;
                impl TopographicalMap {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `TopographicalMap`.
                    pub fn new<T: GuestTopographicalMap>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _TopographicalMapRep<T> = Some(val);
                        let ptr: *mut _TopographicalMapRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestTopographicalMap>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestTopographicalMap>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestTopographicalMap>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(
                            handle as *mut _TopographicalMapRep<T>,
                        );
                    }
                    fn as_ptr<T: GuestTopographicalMap>(
                        &self,
                    ) -> *mut _TopographicalMapRep<T> {
                        TopographicalMap::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`TopographicalMap`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct TopographicalMapBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a TopographicalMap>,
                }
                impl<'a> TopographicalMapBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestTopographicalMap>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _TopographicalMapRep<T> {
                        TopographicalMap::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for TopographicalMap {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]aoc2024:day10/types")]
                            extern "C" {
                                #[link_name = "[resource-drop]topographical-map"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_topographical_map_cabi<
                    T: GuestTopographicalMap,
                >(arg0: *mut u8, arg1: usize) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let base3 = arg0;
                    let len3 = arg1;
                    let mut result3 = _rt::Vec::with_capacity(len3);
                    for i in 0..len3 {
                        let base = base3.add(i * 8);
                        let e3 = {
                            let l0 = *base.add(0).cast::<*mut u8>();
                            let l1 = *base.add(4).cast::<usize>();
                            let len2 = l1;
                            _rt::Vec::from_raw_parts(l0.cast(), len2, len2)
                        };
                        result3.push(e3);
                    }
                    _rt::cabi_dealloc(base3, len3 * 8, 4);
                    let result4 = TopographicalMap::new(T::new(result3));
                    (result4).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_topographical_map_map_width_cabi<
                    T: GuestTopographicalMap,
                >(arg0: *mut u8) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::map_width(
                        TopographicalMapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_topographical_map_map_height_cabi<
                    T: GuestTopographicalMap,
                >(arg0: *mut u8) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::map_height(
                        TopographicalMapBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_topographical_map_height_at_location_cabi<
                    T: GuestTopographicalMap,
                >(arg0: *mut u8, arg1: i32, arg2: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::height_at_location(
                        TopographicalMapBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u32,
                    );
                    _rt::as_i32(result0)
                }
                pub trait Guest {
                    type TopographicalMap: GuestTopographicalMap;
                }
                pub trait GuestTopographicalMap: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]aoc2024:day10/types")]
                            extern "C" {
                                #[link_name = "[resource-new]topographical-map"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]aoc2024:day10/types")]
                            extern "C" {
                                #[link_name = "[resource-rep]topographical-map"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(map: _rt::Vec<_rt::Vec<u8>>) -> Self;
                    fn map_width(&self) -> u32;
                    fn map_height(&self) -> u32;
                    fn height_at_location(&self, x: u32, y: u32) -> u8;
                }
                #[doc(hidden)]
                macro_rules! __export_aoc2024_day10_types_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "aoc2024:day10/types#[constructor]topographical-map"] unsafe
                        extern "C" fn export_constructor_topographical_map(arg0 : * mut
                        u8, arg1 : usize,) -> i32 { $($path_to_types)*::
                        _export_constructor_topographical_map_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TopographicalMap > (arg0, arg1) }
                        #[export_name =
                        "aoc2024:day10/types#[method]topographical-map.map-width"] unsafe
                        extern "C" fn export_method_topographical_map_map_width(arg0 : *
                        mut u8,) -> i32 { $($path_to_types)*::
                        _export_method_topographical_map_map_width_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TopographicalMap > (arg0) }
                        #[export_name =
                        "aoc2024:day10/types#[method]topographical-map.map-height"]
                        unsafe extern "C" fn
                        export_method_topographical_map_map_height(arg0 : * mut u8,) ->
                        i32 { $($path_to_types)*::
                        _export_method_topographical_map_map_height_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::TopographicalMap > (arg0) }
                        #[export_name =
                        "aoc2024:day10/types#[method]topographical-map.height-at-location"]
                        unsafe extern "C" fn
                        export_method_topographical_map_height_at_location(arg0 : * mut
                        u8, arg1 : i32, arg2 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_topographical_map_height_at_location_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::TopographicalMap > (arg0, arg1,
                        arg2) } const _ : () = { #[doc(hidden)] #[export_name =
                        "aoc2024:day10/types#[dtor]topographical-map"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: TopographicalMap::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::TopographicalMap > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_aoc2024_day10_types_cabi;
            }
            #[allow(dead_code, clippy::all)]
            pub mod solver {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type TopographicalMap = super::super::super::super::exports::aoc2024::day10::types::TopographicalMap;
                pub type TopographicalMapBorrow<'a> = super::super::super::super::exports::aoc2024::day10::types::TopographicalMapBorrow<
                    'a,
                >;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_a_cabi<T: Guest>(arg0: i32) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::solve_a(
                        TopographicalMapBorrow::lift(arg0 as u32 as usize),
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_b_cabi<T: Guest>(arg0: i32) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::solve_b(
                        TopographicalMapBorrow::lift(arg0 as u32 as usize),
                    );
                    _rt::as_i64(result0)
                }
                pub trait Guest {
                    fn solve_a(input: TopographicalMapBorrow<'_>) -> u64;
                    fn solve_b(input: TopographicalMapBorrow<'_>) -> u64;
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
    pub use alloc_crate::boxed::Box;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
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
        exports::aoc2024::day10::types::__export_aoc2024_day10_types_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::aoc2024::day10::types);
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
\0,[method]topographical-map.height-at-location\x01\x07\x04\0\x13aoc2024:day10/t\
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
