pub type ByteArray = _rt::Vec<u8>;
#[derive(Clone)]
pub struct Event {
    pub id: _rt::String,
    pub content: ByteArray,
}
impl ::core::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Event")
            .field("id", &self.id)
            .field("content", &self.content)
            .finish()
    }
}
#[allow(unused_unsafe, clippy::all)]
/// Host-provided functionality
pub fn persist(key: &str, req: &ByteArray) {
    unsafe {
        let vec0 = key;
        let ptr0 = vec0.as_ptr().cast::<u8>();
        let len0 = vec0.len();
        let vec1 = req;
        let ptr1 = vec1.as_ptr().cast::<u8>();
        let len1 = vec1.len();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "persist"]
            fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize) {
            unreachable!()
        }
        wit_import(ptr0.cast_mut(), len0, ptr1.cast_mut(), len1);
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn retrieve(key: &str) -> Option<ByteArray> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
        let vec0 = key;
        let ptr0 = vec0.as_ptr().cast::<u8>();
        let len0 = vec0.len();
        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "retrieve"]
            fn wit_import(_: *mut u8, _: usize, _: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0.cast_mut(), len0, ptr1);
        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
        match l2 {
            0 => None,
            1 => {
                let e = {
                    let l3 = *ptr1.add(4).cast::<*mut u8>();
                    let l4 = *ptr1.add(8).cast::<usize>();
                    let len5 = l4;
                    _rt::Vec::from_raw_parts(l3.cast(), len5, len5)
                };
                Some(e)
            }
            _ => _rt::invalid_enum_discriminant(),
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn retrieve_range(start: &str, end: &str) -> _rt::Vec<ByteArray> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
        let vec0 = start;
        let ptr0 = vec0.as_ptr().cast::<u8>();
        let len0 = vec0.len();
        let vec1 = end;
        let ptr1 = vec1.as_ptr().cast::<u8>();
        let len1 = vec1.len();
        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "retrieve-range"]
            fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize, _: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize, _: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0.cast_mut(), len0, ptr1.cast_mut(), len1, ptr2);
        let l3 = *ptr2.add(0).cast::<*mut u8>();
        let l4 = *ptr2.add(4).cast::<usize>();
        let base8 = l3;
        let len8 = l4;
        let mut result8 = _rt::Vec::with_capacity(len8);
        for i in 0..len8 {
            let base = base8.add(i * 8);
            let e8 = {
                let l5 = *base.add(0).cast::<*mut u8>();
                let l6 = *base.add(4).cast::<usize>();
                let len7 = l6;
                _rt::Vec::from_raw_parts(l5.cast(), len7, len7)
            };
            result8.push(e8);
        }
        _rt::cabi_dealloc(base8, len8 * 8, 4);
        result8
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_execute_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let result1 = T::execute(_rt::Vec::from_raw_parts(arg0.cast(), len0, len0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result1 {
        Ok(e) => {
            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
            let Event { id: id3, content: content3 } = e;
            let vec4 = (id3.into_bytes()).into_boxed_slice();
            let ptr4 = vec4.as_ptr().cast::<u8>();
            let len4 = vec4.len();
            ::core::mem::forget(vec4);
            *ptr2.add(8).cast::<usize>() = len4;
            *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
            let vec5 = (content3).into_boxed_slice();
            let ptr5 = vec5.as_ptr().cast::<u8>();
            let len5 = vec5.len();
            ::core::mem::forget(vec5);
            *ptr2.add(16).cast::<usize>() = len5;
            *ptr2.add(12).cast::<*mut u8>() = ptr5.cast_mut();
        }
        Err(e) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
            let vec6 = (e).into_boxed_slice();
            let ptr6 = vec6.as_ptr().cast::<u8>();
            let len6 = vec6.len();
            ::core::mem::forget(vec6);
            *ptr2.add(8).cast::<usize>() = len6;
            *ptr2.add(4).cast::<*mut u8>() = ptr6.cast_mut();
        }
    };
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_execute<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = *arg0.add(4).cast::<*mut u8>();
            let l2 = *arg0.add(8).cast::<usize>();
            _rt::cabi_dealloc(l1, l2, 1);
            let l3 = *arg0.add(12).cast::<*mut u8>();
            let l4 = *arg0.add(16).cast::<usize>();
            let base5 = l3;
            let len5 = l4;
            _rt::cabi_dealloc(base5, len5 * 1, 1);
        }
        _ => {
            let l6 = *arg0.add(4).cast::<*mut u8>();
            let l7 = *arg0.add(8).cast::<usize>();
            let base8 = l6;
            let len8 = l7;
            _rt::cabi_dealloc(base8, len8 * 1, 1);
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_query_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let result1 = T::query(_rt::Vec::from_raw_parts(arg0.cast(), len0, len0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result1 {
        Ok(e) => {
            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
            let vec3 = (e).into_boxed_slice();
            let ptr3 = vec3.as_ptr().cast::<u8>();
            let len3 = vec3.len();
            ::core::mem::forget(vec3);
            *ptr2.add(8).cast::<usize>() = len3;
            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
        }
        Err(e) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
            let vec4 = (e).into_boxed_slice();
            let ptr4 = vec4.as_ptr().cast::<u8>();
            let len4 = vec4.len();
            ::core::mem::forget(vec4);
            *ptr2.add(8).cast::<usize>() = len4;
            *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
        }
    };
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_query<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = *arg0.add(4).cast::<*mut u8>();
            let l2 = *arg0.add(8).cast::<usize>();
            let base3 = l1;
            let len3 = l2;
            _rt::cabi_dealloc(base3, len3 * 1, 1);
        }
        _ => {
            let l4 = *arg0.add(4).cast::<*mut u8>();
            let l5 = *arg0.add(8).cast::<usize>();
            let base6 = l4;
            let len6 = l5;
            _rt::cabi_dealloc(base6, len6 * 1, 1);
        }
    }
}
pub trait Guest {
    /// Component API
    fn execute(cmd: ByteArray) -> Result<Event, ByteArray>;
    fn query(req: ByteArray) -> Result<ByteArray, ByteArray>;
}
#[doc(hidden)]
macro_rules! __export_world_example_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "execute"] unsafe extern "C" fn
        export_execute(arg0 : * mut u8, arg1 : usize,) -> * mut u8 { $($path_to_types)*::
        _export_execute_cabi::<$ty > (arg0, arg1) } #[export_name = "cabi_post_execute"]
        unsafe extern "C" fn _post_return_execute(arg0 : * mut u8,) {
        $($path_to_types)*:: __post_return_execute::<$ty > (arg0) } #[export_name =
        "query"] unsafe extern "C" fn export_query(arg0 : * mut u8, arg1 : usize,) -> *
        mut u8 { $($path_to_types)*:: _export_query_cabi::<$ty > (arg0, arg1) }
        #[export_name = "cabi_post_query"] unsafe extern "C" fn _post_return_query(arg0 :
        * mut u8,) { $($path_to_types)*:: __post_return_query::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_example_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 20]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 20]);
mod _rt {
    pub use alloc_crate::vec::Vec;
    pub use alloc_crate::string::String;
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
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
macro_rules! __export_example_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_example_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:dnevest:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 354] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xe4\x01\x01A\x02\x01\
A\x12\x01p}\x03\0\x0abyte-array\x03\0\0\x01r\x02\x02ids\x07content\x01\x03\0\x05\
event\x03\0\x02\x01@\x02\x03keys\x03req\x01\x01\0\x03\0\x07persist\x01\x04\x01k\x01\
\x01@\x01\x03keys\0\x05\x03\0\x08retrieve\x01\x06\x01p\x01\x01@\x02\x05starts\x03\
ends\0\x07\x03\0\x0eretrieve-range\x01\x08\x01j\x01\x03\x01\x01\x01@\x01\x03cmd\x01\
\0\x09\x04\0\x07execute\x01\x0a\x01j\x01\x01\x01\x01\x01@\x01\x03req\x01\0\x0b\x04\
\0\x05query\x01\x0c\x04\x01\x19component:dnevest/example\x04\0\x0b\x0d\x01\0\x07\
example\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.21\
6.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
