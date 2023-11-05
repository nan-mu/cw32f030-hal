#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register1"]
    pub cr1: CR1,
    #[doc = "0x04 - Interrupt enable register"]
    pub ier: IER,
    #[doc = "0x08 - Control register2"]
    pub cr2: CR2,
    #[doc = "0x0c - Slave slect register"]
    pub ssi: SSI,
    #[doc = "0x10 - Interrupt status register"]
    pub isr: ISR,
    #[doc = "0x14 - Interrupt flag clear register"]
    pub icr: ICR,
    #[doc = "0x18 - Data register"]
    pub dr: DR,
}
#[doc = "CR1 (rw) register accessor: Control register1\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::svd::Reg<cr1::CR1_SPEC>;
#[doc = "Control register1"]
pub mod cr1;
#[doc = "IER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::svd::Reg<ier::IER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "CR2 (rw) register accessor: Control register2\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::svd::Reg<cr2::CR2_SPEC>;
#[doc = "Control register2"]
pub mod cr2;
#[doc = "SSI (rw) register accessor: Slave slect register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ssi::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ssi::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssi`]
module"]
pub type SSI = crate::svd::Reg<ssi::SSI_SPEC>;
#[doc = "Slave slect register"]
pub mod ssi;
#[doc = "ISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::svd::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interrupt flag clear register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::svd::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod icr;
#[doc = "DR (rw) register accessor: Data register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::svd::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
