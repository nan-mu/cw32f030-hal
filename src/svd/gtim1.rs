#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0300],
    #[doc = "0x300 - Auto Reload Register"]
    pub arr: ARR,
    #[doc = "0x304 - Counter Register"]
    pub cnt: CNT,
    #[doc = "0x308 - Capture compare control Register"]
    pub cmmr: CMMR,
    #[doc = "0x30c - ETR Control register"]
    pub etr: ETR,
    #[doc = "0x310 - Control register0"]
    pub cr0: CR0,
    #[doc = "0x314 - Interrupt enable register"]
    pub ier: IER,
    #[doc = "0x318 - Interrupt status register"]
    pub isr: ISR,
    #[doc = "0x31c - Interrupt flag clear register"]
    pub icr: ICR,
    #[doc = "0x320 - capture compare register"]
    pub ccr1: CCR1,
    #[doc = "0x324 - capture compare register"]
    pub ccr2: CCR2,
    #[doc = "0x328 - capture compare register"]
    pub ccr3: CCR3,
    #[doc = "0x32c - capture compare register"]
    pub ccr4: CCR4,
    #[doc = "0x330 - Control register1"]
    pub cr1: CR1,
    _reserved13: [u8; 0x0c],
    #[doc = "0x340 - DMA Control register"]
    pub dma: DMA,
}
#[doc = "ARR (rw) register accessor: Auto Reload Register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`arr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`]
module"]
pub type ARR = crate::svd::Reg<arr::ARR_SPEC>;
#[doc = "Auto Reload Register"]
pub mod arr;
#[doc = "CNT (rw) register accessor: Counter Register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::svd::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Register"]
pub mod cnt;
#[doc = "CMMR (rw) register accessor: Capture compare control Register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cmmr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cmmr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmmr`]
module"]
pub type CMMR = crate::svd::Reg<cmmr::CMMR_SPEC>;
#[doc = "Capture compare control Register"]
pub mod cmmr;
#[doc = "ETR (rw) register accessor: ETR Control register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`etr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`etr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etr`]
module"]
pub type ETR = crate::svd::Reg<etr::ETR_SPEC>;
#[doc = "ETR Control register"]
pub mod etr;
#[doc = "CR0 (rw) register accessor: Control register0\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr0::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`]
module"]
pub type CR0 = crate::svd::Reg<cr0::CR0_SPEC>;
#[doc = "Control register0"]
pub mod cr0;
#[doc = "IER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::svd::Reg<ier::IER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "ISR (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::svd::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interrupt flag clear register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::svd::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod icr;
#[doc = "CCR1 (rw) register accessor: capture compare register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ccr1::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
pub type CCR1 = crate::svd::Reg<ccr1::CCR1_SPEC>;
#[doc = "capture compare register"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: capture compare register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ccr2::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`]
module"]
pub type CCR2 = crate::svd::Reg<ccr2::CCR2_SPEC>;
#[doc = "capture compare register"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: capture compare register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ccr3::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`]
module"]
pub type CCR3 = crate::svd::Reg<ccr3::CCR3_SPEC>;
#[doc = "capture compare register"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: capture compare register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ccr4::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4`]
module"]
pub type CCR4 = crate::svd::Reg<ccr4::CCR4_SPEC>;
#[doc = "capture compare register"]
pub mod ccr4;
#[doc = "CR1 (rw) register accessor: Control register1\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::svd::Reg<cr1::CR1_SPEC>;
#[doc = "Control register1"]
pub mod cr1;
#[doc = "DMA (rw) register accessor: DMA Control register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
pub type DMA = crate::svd::Reg<dma::DMA_SPEC>;
#[doc = "DMA Control register"]
pub mod dma;
