use core::{mem::{size_of, transmute}, ptr::with_exposed_provenance};

/// Define a constructor that runs before `main`.
#[macro_export]
macro_rules! ctor {
    (fn $name:ident() { $($body:tt)* }) => {
        #[used]
        #[unsafe(link_section = ".ctors")]
        #[allow(non_upper_case_globals)]
        static $name: fn() = || {
            $($body)*
        };
    };
}

/// Define a destructor that runs after `main`.
///
/// Since the runtime `panic!`s after `main` returns, this is mostly useful with
/// the `loadable_exe` feature.
#[macro_export]
macro_rules! dtor {
    (fn $name:ident() { $($body:tt)* }) => {
        #[used]
        #[unsafe(link_section = ".dtors")]
        #[allow(non_upper_case_globals)]
        static $name: fn() = || {
            $($body)*
        };
    };
}

#[cfg(feature = "loadable_exe")]
type RtReturn = ();
#[cfg(not(feature = "loadable_exe"))]
type RtReturn = !;

/// The runtime used by the default linker scripts.
#[unsafe(no_mangle)]
extern "C" fn __start() -> RtReturn {
    // SAFETY: If there is no unmangled function named `main` this causes an error
    // at link-time.
    unsafe {
        #[cfg(not(test))]
        unsafe extern "Rust" {
            fn main();
        }
        unsafe extern "C" {
            static __ctors_start: usize;
            static __ctors_end: usize;
            static __dtors_start: usize;
            static __dtors_end: usize;
        }
        let start = (&raw const __ctors_start).expose_provenance();
        let end = (&raw const __ctors_end).expose_provenance();
        let ctors_range = end - start;
        let num_ctors = ctors_range / size_of::<usize>();
        for n in 0..num_ctors {
            let ctor_ptr = start + (n * size_of::<usize>());
            let fn_addr = *with_exposed_provenance(ctor_ptr);
            let ctor = transmute::<usize, fn()>(fn_addr);
            ctor();
        }
        #[cfg(not(test))]
        main();

        #[cfg(test)]
        crate::main();

        let start = (&raw const __dtors_start).expose_provenance();
        let end = (&raw const __dtors_end).expose_provenance();
        let dtors_range = end - start;
        let num_dtors = dtors_range / size_of::<usize>();
        for n in 0..num_dtors {
            let dtor_ptr = start + (n * size_of::<usize>());
            let fn_addr = *with_exposed_provenance(dtor_ptr);
            let dtor = transmute::<usize, fn()>(fn_addr);
            dtor();
        }
    }
    #[cfg(not(feature = "loadable_exe"))]
    panic!("`main` should not return")
}

// Define string-literals to embed in PSEXE header
// Using the same identifier for all regions conveniently makes the crate
// features mutually exclusive

#[cfg(any(feature = "NA_region", test))]
#[used]
#[unsafe(no_mangle)]
#[doc(hidden)]
#[unsafe(link_section = ".region")]
pub static _REGION: [u8; 55] = *b"Sony Computer Entertainment Inc. for North America area";

#[cfg(feature = "EU_region")]
#[used]
#[unsafe(no_mangle)]
#[doc(hidden)]
#[unsafe(link_section = ".region")]
pub static _REGION: [u8; 48] = *b"Sony Computer Entertainment Inc. for Europe area";

#[cfg(feature = "J_region")]
#[used]
#[unsafe(no_mangle)]
#[doc(hidden)]
#[unsafe(link_section = ".region")]
pub static _REGION: [u8; 47] = *b"Sony Computer Entertainment Inc. for Japan area";
