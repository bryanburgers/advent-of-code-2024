#[allow(dead_code)]
pub mod aoc2024 {
    #[allow(dead_code)]
    pub mod day10 {
        #[allow(dead_code, clippy::all)]
        pub mod solver {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn solve_a(input: &[_rt::Vec<u8>]) -> u64 {
                unsafe {
                    let vec1 = input;
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
                    #[link(wasm_import_module = "aoc2024:day10/solver")]
                    extern "C" {
                        #[link_name = "solve-a"]
                        fn wit_import(_: *mut u8, _: usize) -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import(result1, len1);
                    if layout1.size() != 0 {
                        _rt::alloc::dealloc(result1.cast(), layout1);
                    }
                    ret as u64
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn solve_b(input: &[_rt::Vec<u8>]) -> u64 {
                unsafe {
                    let vec1 = input;
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
                    #[link(wasm_import_module = "aoc2024:day10/solver")]
                    extern "C" {
                        #[link_name = "solve-b"]
                        fn wit_import(_: *mut u8, _: usize) -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import(result1, len1);
                    if layout1.size() != 0 {
                        _rt::alloc::dealloc(result1.cast(), layout1);
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
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Runner {
                    handle: _rt::Resource<Runner>,
                }
                type _RunnerRep<T> = Option<T>;
                impl Runner {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Runner`.
                    pub fn new<T: GuestRunner>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _RunnerRep<T> = Some(val);
                        let ptr: *mut _RunnerRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestRunner>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestRunner>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestRunner>(self) -> T {
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
                        let _ = _rt::Box::from_raw(handle as *mut _RunnerRep<T>);
                    }
                    fn as_ptr<T: GuestRunner>(&self) -> *mut _RunnerRep<T> {
                        Runner::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Runner`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct RunnerBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Runner>,
                }
                impl<'a> RunnerBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestRunner>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _RunnerRep<T> {
                        Runner::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Runner {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]aoc:base/day")]
                            extern "C" {
                                #[link_name = "[resource-drop]runner"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_runner_cabi<T: GuestRunner>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = Runner::new(T::new(_rt::string_lift(bytes0)));
                    (result1).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_runner_solve_a_cabi<T: GuestRunner>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::solve_a(
                        RunnerBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_runner_solve_a<T: GuestRunner>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_runner_solve_b_cabi<T: GuestRunner>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::solve_b(
                        RunnerBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_runner_solve_b<T: GuestRunner>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    type Runner: GuestRunner;
                }
                pub trait GuestRunner: 'static {
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
                            #[link(wasm_import_module = "[export]aoc:base/day")]
                            extern "C" {
                                #[link_name = "[resource-new]runner"]
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
                            #[link(wasm_import_module = "[export]aoc:base/day")]
                            extern "C" {
                                #[link_name = "[resource-rep]runner"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(input: _rt::String) -> Self;
                    fn solve_a(&self) -> _rt::String;
                    fn solve_b(&self) -> _rt::String;
                }
                #[doc(hidden)]
                macro_rules! __export_aoc_base_day_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "aoc:base/day#[constructor]runner"] unsafe extern "C" fn
                        export_constructor_runner(arg0 : * mut u8, arg1 : usize,) -> i32
                        { $($path_to_types)*:: _export_constructor_runner_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Runner > (arg0, arg1) }
                        #[export_name = "aoc:base/day#[method]runner.solve-a"] unsafe
                        extern "C" fn export_method_runner_solve_a(arg0 : * mut u8,) -> *
                        mut u8 { $($path_to_types)*::
                        _export_method_runner_solve_a_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Runner > (arg0) } #[export_name =
                        "cabi_post_aoc:base/day#[method]runner.solve-a"] unsafe extern
                        "C" fn _post_return_method_runner_solve_a(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_method_runner_solve_a::<<$ty
                        as $($path_to_types)*:: Guest >::Runner > (arg0) } #[export_name
                        = "aoc:base/day#[method]runner.solve-b"] unsafe extern "C" fn
                        export_method_runner_solve_b(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_runner_solve_b_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Runner > (arg0) } #[export_name =
                        "cabi_post_aoc:base/day#[method]runner.solve-b"] unsafe extern
                        "C" fn _post_return_method_runner_solve_b(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_method_runner_solve_b::<<$ty
                        as $($path_to_types)*:: Guest >::Runner > (arg0) } const _ : () =
                        { #[doc(hidden)] #[export_name = "aoc:base/day#[dtor]runner"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Runner::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Runner > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_aoc_base_day_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::vec::Vec;
    pub use alloc_crate::alloc;
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
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::string::String;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
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
#[link_section = "component-type:wit-bindgen:0.35.0:aoc2024:day10-parser:parser:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 371] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xf6\x01\x01A\x02\x01\
A\x04\x01B\x05\x01p}\x01p\0\x01@\x01\x05input\x01\0w\x04\0\x07solve-a\x01\x02\x04\
\0\x07solve-b\x01\x02\x03\0\x14aoc2024:day10/solver\x05\0\x01B\x08\x04\0\x06runn\
er\x03\x01\x01i\0\x01@\x01\x05inputs\0\x01\x04\0\x13[constructor]runner\x01\x02\x01\
h\0\x01@\x01\x04self\x03\0s\x04\0\x16[method]runner.solve-a\x01\x04\x04\0\x16[me\
thod]runner.solve-b\x01\x04\x04\0\x0caoc:base/day\x05\x01\x04\0\x1baoc2024:day10\
-parser/parser\x04\0\x0b\x0c\x01\0\x06parser\x03\0\0\0G\x09producers\x01\x0cproc\
essed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
