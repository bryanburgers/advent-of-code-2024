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
        pub mod day15 {
            #[allow(dead_code, clippy::all)]
            pub mod solver {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct Position {
                    pub x: i64,
                    pub y: i64,
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
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
                pub enum TileType {
                    Wall,
                    Box,
                }
                impl ::core::fmt::Debug for TileType {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            TileType::Wall => f.debug_tuple("TileType::Wall").finish(),
                            TileType::Box => f.debug_tuple("TileType::Box").finish(),
                        }
                    }
                }
                impl TileType {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> TileType {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => TileType::Wall,
                            1 => TileType::Box,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct Tile {
                    pub position: Position,
                    pub type_: TileType,
                }
                impl ::core::fmt::Debug for Tile {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Tile")
                            .field("position", &self.position)
                            .field("type", &self.type_)
                            .finish()
                    }
                }
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
                pub enum Move {
                    North,
                    South,
                    East,
                    West,
                }
                impl ::core::fmt::Debug for Move {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Move::North => f.debug_tuple("Move::North").finish(),
                            Move::South => f.debug_tuple("Move::South").finish(),
                            Move::East => f.debug_tuple("Move::East").finish(),
                            Move::West => f.debug_tuple("Move::West").finish(),
                        }
                    }
                }
                impl Move {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> Move {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => Move::North,
                            1 => Move::South,
                            2 => Move::East,
                            3 => Move::West,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                #[derive(Clone)]
                pub struct Input {
                    pub lantern_fish: Position,
                    pub tiles: _rt::Vec<Tile>,
                    pub moves: _rt::Vec<Move>,
                }
                impl ::core::fmt::Debug for Input {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Input")
                            .field("lantern-fish", &self.lantern_fish)
                            .field("tiles", &self.tiles)
                            .field("moves", &self.moves)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_a_cabi<T: Guest>(
                    arg0: i64,
                    arg1: i64,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: *mut u8,
                    arg5: usize,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let base3 = arg2;
                    let len3 = arg3;
                    let mut result3 = _rt::Vec::with_capacity(len3);
                    for i in 0..len3 {
                        let base = base3.add(i * 24);
                        let e3 = {
                            let l0 = *base.add(0).cast::<i64>();
                            let l1 = *base.add(8).cast::<i64>();
                            let l2 = i32::from(*base.add(16).cast::<u8>());
                            Tile {
                                position: Position { x: l0, y: l1 },
                                type_: TileType::_lift(l2 as u8),
                            }
                        };
                        result3.push(e3);
                    }
                    _rt::cabi_dealloc(base3, len3 * 24, 8);
                    let base5 = arg4;
                    let len5 = arg5;
                    let mut result5 = _rt::Vec::with_capacity(len5);
                    for i in 0..len5 {
                        let base = base5.add(i * 1);
                        let e5 = {
                            let l4 = i32::from(*base.add(0).cast::<u8>());
                            Move::_lift(l4 as u8)
                        };
                        result5.push(e5);
                    }
                    _rt::cabi_dealloc(base5, len5 * 1, 1);
                    let result6 = T::solve_a(Input {
                        lantern_fish: Position { x: arg0, y: arg1 },
                        tiles: result3,
                        moves: result5,
                    });
                    _rt::as_i64(result6)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_solve_b_cabi<T: Guest>(
                    arg0: i64,
                    arg1: i64,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: *mut u8,
                    arg5: usize,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let base3 = arg2;
                    let len3 = arg3;
                    let mut result3 = _rt::Vec::with_capacity(len3);
                    for i in 0..len3 {
                        let base = base3.add(i * 24);
                        let e3 = {
                            let l0 = *base.add(0).cast::<i64>();
                            let l1 = *base.add(8).cast::<i64>();
                            let l2 = i32::from(*base.add(16).cast::<u8>());
                            Tile {
                                position: Position { x: l0, y: l1 },
                                type_: TileType::_lift(l2 as u8),
                            }
                        };
                        result3.push(e3);
                    }
                    _rt::cabi_dealloc(base3, len3 * 24, 8);
                    let base5 = arg4;
                    let len5 = arg5;
                    let mut result5 = _rt::Vec::with_capacity(len5);
                    for i in 0..len5 {
                        let base = base5.add(i * 1);
                        let e5 = {
                            let l4 = i32::from(*base.add(0).cast::<u8>());
                            Move::_lift(l4 as u8)
                        };
                        result5.push(e5);
                    }
                    _rt::cabi_dealloc(base5, len5 * 1, 1);
                    let result6 = T::solve_b(Input {
                        lantern_fish: Position { x: arg0, y: arg1 },
                        tiles: result3,
                        moves: result5,
                    });
                    _rt::as_i64(result6)
                }
                pub trait Guest {
                    fn solve_a(input: Input) -> i64;
                    fn solve_b(input: Input) -> i64;
                }
                #[doc(hidden)]
                macro_rules! __export_aoc2024_day15_solver_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "aoc2024:day15/solver#solve-a"]
                        unsafe extern "C" fn export_solve_a(arg0 : i64, arg1 : i64, arg2
                        : * mut u8, arg3 : usize, arg4 : * mut u8, arg5 : usize,) -> i64
                        { $($path_to_types)*:: _export_solve_a_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4, arg5) } #[export_name =
                        "aoc2024:day15/solver#solve-b"] unsafe extern "C" fn
                        export_solve_b(arg0 : i64, arg1 : i64, arg2 : * mut u8, arg3 :
                        usize, arg4 : * mut u8, arg5 : usize,) -> i64 {
                        $($path_to_types)*:: _export_solve_b_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4, arg5) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_aoc2024_day15_solver_cabi;
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
        exports::aoc2024::day15::solver::__export_aoc2024_day15_solver_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::aoc2024::day15::solver);
    };
}
#[doc(inline)]
pub(crate) use __export_solver_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:aoc2024:day15-solver:solver:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 432] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xb3\x02\x01A\x02\x01\
A\x04\x01B\x02\x01@\x01\x05inputs\x01\0\x04\0\x04info\x01\0\x03\0\x0eaoc:base/de\
bug\x05\0\x01B\x0f\x01r\x02\x01xx\x01yx\x04\0\x08position\x03\0\0\x01m\x02\x04wa\
ll\x03box\x04\0\x09tile-type\x03\0\x02\x01r\x02\x08position\x01\x04type\x03\x04\0\
\x04tile\x03\0\x04\x01m\x04\x05north\x05south\x04east\x04west\x04\0\x04move\x03\0\
\x06\x01p\x05\x01p\x07\x01r\x03\x0clantern-fish\x01\x05tiles\x08\x05moves\x09\x04\
\0\x05input\x03\0\x0a\x01@\x01\x05input\x0b\0x\x04\0\x07solve-a\x01\x0c\x04\0\x07\
solve-b\x01\x0c\x04\0\x14aoc2024:day15/solver\x05\x01\x04\0\x1baoc2024:day15-sol\
ver/solver\x04\0\x0b\x0c\x01\0\x06solver\x03\0\0\0G\x09producers\x01\x0cprocesse\
d-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
