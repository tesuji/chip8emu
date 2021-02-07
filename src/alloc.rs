use std::alloc::{alloc_zeroed, handle_alloc_error, Layout};

// FIXME: Use `Box::new_zeroed` when it stabilized.
// <https://github.com/rust-lang/rust/issues/63291>.
macro_rules! gen_boxed_array {
    ($name:ident, $TYPE:ty, $SIZE:expr) => {
        pub(crate) fn $name() -> Box<[$TYPE; $SIZE]> {
            type T = [$TYPE; $SIZE];
            const LAYOUT: Layout = Layout::new::<T>();
            unsafe {
                let ptr = alloc_zeroed(LAYOUT);
                if ptr.is_null() {
                    handle_alloc_error(LAYOUT);
                }
                Box::from_raw(ptr.cast::<T>())
            }
        }
    };
}

gen_boxed_array!(boxed_zeroed_memory, u8, crate::memory::RAM_SIZE as usize);
gen_boxed_array!(boxed_zeroed_display, bool, crate::display::DISPLAY_SIZE);
