pub type ByteArray = _rt::Vec<u8>;
pub type Event = (_rt::String, _rt::Vec<ByteArray>);
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
            let vec7 = e;
            let len7 = vec7.len();
            let layout7 = _rt::alloc::Layout::from_size_align_unchecked(
                vec7.len() * 16,
                4,
            );
            let result7 = if layout7.size() != 0 {
                let ptr = _rt::alloc::alloc(layout7).cast::<u8>();
                if ptr.is_null() {
                    _rt::alloc::handle_alloc_error(layout7);
                }
                ptr
            } else {
                { ::core::ptr::null_mut() }
            };
            for (i, e) in vec7.into_iter().enumerate() {
                let base = result7.add(i * 16);
                {
                    let (t3_0, t3_1) = e;
                    let vec4 = (t3_0.into_bytes()).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *base.add(4).cast::<usize>() = len4;
                    *base.add(0).cast::<*mut u8>() = ptr4.cast_mut();
                    let vec6 = t3_1;
                    let len6 = vec6.len();
                    let layout6 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec6.len() * 8,
                        4,
                    );
                    let result6 = if layout6.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout6).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout6);
                        }
                        ptr
                    } else {
                        { ::core::ptr::null_mut() }
                    };
                    for (i, e) in vec6.into_iter().enumerate() {
                        let base = result6.add(i * 8);
                        {
                            let vec5 = (e).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *base.add(4).cast::<usize>() = len5;
                            *base.add(0).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    }
                    *base.add(12).cast::<usize>() = len6;
                    *base.add(8).cast::<*mut u8>() = result6;
                }
            }
            *ptr2.add(8).cast::<usize>() = len7;
            *ptr2.add(4).cast::<*mut u8>() = result7;
        }
        Err(_) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
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
            let base11 = l1;
            let len11 = l2;
            for i in 0..len11 {
                let base = base11.add(i * 16);
                {
                    let l3 = *base.add(0).cast::<*mut u8>();
                    let l4 = *base.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l3, l4, 1);
                    let l5 = *base.add(8).cast::<*mut u8>();
                    let l6 = *base.add(12).cast::<usize>();
                    let base10 = l5;
                    let len10 = l6;
                    for i in 0..len10 {
                        let base = base10.add(i * 8);
                        {
                            let l7 = *base.add(0).cast::<*mut u8>();
                            let l8 = *base.add(4).cast::<usize>();
                            let base9 = l7;
                            let len9 = l8;
                            _rt::cabi_dealloc(base9, len9 * 1, 1);
                        }
                    }
                    _rt::cabi_dealloc(base10, len10 * 8, 4);
                }
            }
            _rt::cabi_dealloc(base11, len11 * 16, 4);
        }
        _ => {}
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
            let vec4 = e;
            let len4 = vec4.len();
            let layout4 = _rt::alloc::Layout::from_size_align_unchecked(
                vec4.len() * 8,
                4,
            );
            let result4 = if layout4.size() != 0 {
                let ptr = _rt::alloc::alloc(layout4).cast::<u8>();
                if ptr.is_null() {
                    _rt::alloc::handle_alloc_error(layout4);
                }
                ptr
            } else {
                { ::core::ptr::null_mut() }
            };
            for (i, e) in vec4.into_iter().enumerate() {
                let base = result4.add(i * 8);
                {
                    let vec3 = (e).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *base.add(4).cast::<usize>() = len3;
                    *base.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                }
            }
            *ptr2.add(8).cast::<usize>() = len4;
            *ptr2.add(4).cast::<*mut u8>() = result4;
        }
        Err(_) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
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
            let base6 = l1;
            let len6 = l2;
            for i in 0..len6 {
                let base = base6.add(i * 8);
                {
                    let l3 = *base.add(0).cast::<*mut u8>();
                    let l4 = *base.add(4).cast::<usize>();
                    let base5 = l3;
                    let len5 = l4;
                    _rt::cabi_dealloc(base5, len5 * 1, 1);
                }
            }
            _rt::cabi_dealloc(base6, len6 * 8, 4);
        }
        _ => {}
    }
}
pub trait Guest {
    /// Component API
    fn execute(cmd: ByteArray) -> Result<_rt::Vec<Event>, ()>;
    fn query(req: ByteArray) -> Result<_rt::Vec<ByteArray>, ()>;
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
struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 12]);
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
    pub use alloc_crate::alloc;
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
#[link_section = "component-type:wit-bindgen:0.30.0:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 344] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xda\x01\x01A\x02\x01\
A\x13\x01p}\x03\0\x0abyte-array\x03\0\0\x01p\x01\x01o\x02s\x02\x03\0\x05event\x03\
\0\x03\x01@\x02\x03keys\x03req\x01\x01\0\x03\0\x07persist\x01\x05\x01k\x01\x01@\x01\
\x03keys\0\x06\x03\0\x08retrieve\x01\x07\x01@\x02\x05starts\x03ends\0\x02\x03\0\x0e\
retrieve-range\x01\x08\x01p\x04\x01j\x01\x09\0\x01@\x01\x03cmd\x01\0\x0a\x04\0\x07\
execute\x01\x0b\x01j\x01\x02\0\x01@\x01\x03req\x01\0\x0c\x04\0\x05query\x01\x0d\x04\
\x01\x19component:dnevest/example\x04\0\x0b\x0d\x01\0\x07example\x03\0\0\0G\x09p\
roducers\x01\x0cprocessed-by\x02\x0dwit-component\x070.215.0\x10wit-bindgen-rust\
\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
