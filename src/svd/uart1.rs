#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register1"]
    pub cr1: CR1,
    #[doc = "0x04 - Control register2"]
    pub cr2: CR2,
    #[doc = "0x08 - Interrupt enable register"]
    pub ier: IER,
    #[doc = "0x0c - desc BRRI"]
    pub brri: BRRI,
    #[doc = "0x10 - desc BRRF"]
    pub brrf: BRRF,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - Interrupt status register"]
    pub isr: ISR,
    #[doc = "0x20 - Interrupt flag clear register"]
    pub icr: ICR,
    #[doc = "0x24 - Data reg for read"]
    pub rdr: RDR,
    #[doc = "0x28 - Data reg for write"]
    pub tdr: TDR,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - Slave addr"]
    pub addr: ADDR,
    #[doc = "0x34 - Slave addr mask"]
    pub mask: MASK,
}
#[doc = "CR1 (rw) register accessor: Control register1\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::svd::Reg<cr1::CR1_SPEC>;
#[doc = "Control register1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control register2\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::svd::Reg<cr2::CR2_SPEC>;
#[doc = "Control register2"]
pub mod cr2;
#[doc = "IER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::svd::Reg<ier::IER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ier;
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
#[doc = "BRRI (rw) register accessor: desc BRRI\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`brri::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`brri::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brri`]
module"]
pub type BRRI = crate::svd::Reg<brri::BRRI_SPEC>;
#[doc = "desc BRRI"]
pub mod brri;
#[doc = "BRRF (rw) register accessor: desc BRRF\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`brrf::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`brrf::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brrf`]
module"]
pub type BRRF = crate::svd::Reg<brrf::BRRF_SPEC>;
#[doc = "desc BRRF"]
pub mod brrf;
#[doc = "RDR (r) register accessor: Data reg for read\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
pub type RDR = crate::svd::Reg<rdr::RDR_SPEC>;
#[doc = "Data reg for read"]
pub mod rdr;
#[doc = "TDR (w) register accessor: Data reg for write\n\nYou can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`]
module"]
pub type TDR = crate::svd::Reg<tdr::TDR_SPEC>;
#[doc = "Data reg for write"]
pub mod tdr;
#[doc = "ADDR (rw) register accessor: Slave addr\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::svd::Reg<addr::ADDR_SPEC>;
#[doc = "Slave addr"]
pub mod addr;
#[doc = "MASK (rw) register accessor: Slave addr mask\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`mask::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
pub type MASK = crate::svd::Reg<mask::MASK_SPEC>;
#[doc = "Slave addr mask"]
pub mod mask;
