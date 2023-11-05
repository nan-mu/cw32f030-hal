#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt enable register"]
    pub ier: IER,
    #[doc = "0x04 - Parity check error addr register"]
    pub addr: ADDR,
    #[doc = "0x08 - Interrupt flag register"]
    pub isr: ISR,
    #[doc = "0x0c - Interrupt flag clear register"]
    pub icr: ICR,
}
#[doc = "IER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::svd::Reg<ier::IER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "ADDR (r) register accessor: Parity check error addr register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::svd::Reg<addr::ADDR_SPEC>;
#[doc = "Parity check error addr register"]
pub mod addr;
#[doc = "ISR (r) register accessor: Interrupt flag register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::svd::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt flag register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interrupt flag clear register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::svd::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod icr;
