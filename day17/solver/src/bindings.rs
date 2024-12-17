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
        pub mod day17 {
            #[allow(dead_code, clippy::all)]
            pub mod solver {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct Machine {
                    pub register_a: u64,
                    pub register_b: u64,
                    pub register_c: u64,
                    pub program: _rt::Vec<u8>,
                }
                impl ::core::fmt::Debug for Machine {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Machine")
                            .field("register-a", &self.register_a)
                            .field("register-b", &self.register_b)
                            .field("register-c", &self.register_c)
                            .field("program", &self.program)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_a_cabi<T: Guest>(
                    arg0: i64,
                    arg1: i64,
                    arg2: i64,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg4;
                    let result1 = T::solve_a(Machine {
                        register_a: arg0 as u64,
                        register_b: arg1 as u64,
                        register_c: arg2 as u64,
                        program: _rt::Vec::from_raw_parts(arg3.cast(), len0, len0),
                    });
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec3 = (result1).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr2.add(4).cast::<usize>() = len3;
                    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_solve_a<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_b_cabi<T: Guest>(
                    arg0: i64,
                    arg1: i64,
                    arg2: i64,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg4;
                    let result1 = T::solve_b(Machine {
                        register_a: arg0 as u64,
                        register_b: arg1 as u64,
                        register_c: arg2 as u64,
                        program: _rt::Vec::from_raw_parts(arg3.cast(), len0, len0),
                    });
                    _rt::as_i64(result1)
                }
                pub trait Guest {
                    fn solve_a(input: Machine) -> _rt::Vec<u8>;
                    fn solve_b(input: Machine) -> u64;
                }
                #[doc(hidden)]
                macro_rules! __export_aoc2024_day17_solver_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "aoc2024:day17/solver#solve-a"]
                        unsafe extern "C" fn export_solve_a(arg0 : i64, arg1 : i64, arg2
                        : i64, arg3 : * mut u8, arg4 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_solve_a_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4) } #[export_name =
                        "cabi_post_aoc2024:day17/solver#solve-a"] unsafe extern "C" fn
                        _post_return_solve_a(arg0 : * mut u8,) { $($path_to_types)*::
                        __post_return_solve_a::<$ty > (arg0) } #[export_name =
                        "aoc2024:day17/solver#solve-b"] unsafe extern "C" fn
                        export_solve_b(arg0 : i64, arg1 : i64, arg2 : i64, arg3 : * mut
                        u8, arg4 : usize,) -> i64 { $($path_to_types)*::
                        _export_solve_b_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_aoc2024_day17_solver_cabi;
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
        exports::aoc2024::day17::solver::__export_aoc2024_day17_solver_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::aoc2024::day17::solver);
    };
}
#[doc(inline)]
pub(crate) use __export_solver_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:aoc2024:day17-solver:solver:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 346] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xdd\x01\x01A\x02\x01\
A\x04\x01B\x02\x01@\x01\x05inputs\x01\0\x04\0\x04info\x01\0\x03\0\x0eaoc:base/de\
bug\x05\0\x01B\x07\x01p}\x01r\x04\x0aregister-aw\x0aregister-bw\x0aregister-cw\x07\
program\0\x04\0\x07machine\x03\0\x01\x01@\x01\x05input\x02\0\0\x04\0\x07solve-a\x01\
\x03\x01@\x01\x05input\x02\0w\x04\0\x07solve-b\x01\x04\x04\0\x14aoc2024:day17/so\
lver\x05\x01\x04\0\x1baoc2024:day17-solver/solver\x04\0\x0b\x0c\x01\0\x06solver\x03\
\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-\
bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
