#![no_std]

pub mod svd;

#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;

impl svd::Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { svd::Peripherals::steal() })
            }
        })
    }
}
