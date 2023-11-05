#![no_std]

pub mod svd;

impl svd::Peripherals {
    #[doc = r"Returns all the peripherals *once*<br />返回外设操作对象，相当于初始化外设。在高级抽象中应当隐式地调用"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if false {
                None
            } else {
                Some(unsafe { svd::Peripherals::steal() })
            }
        })
    }
    // j8 cw32的RCC叫sysctrl，md
}

#[test]
fn int_peripheral() {
    // j8 cw32的RCC叫sysctrl，md
    let dp = svd::Peripherals::take().unwrap();
    // dp
    // Enable the GPIOA peripheral clock as above.
    dp.SYSCTRL.ahben.modify(|_, w| w.gpioa().set_bit());
    dp.GPIOC.analog.modify(|_, w| w.pin13().bit(true));
    loop {}
}
