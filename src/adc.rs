#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register0"]
    pub cr0: CR0,
    #[doc = "0x04 - Control register1"]
    pub cr1: CR1,
    #[doc = "0x08 - desc START"]
    pub start: START,
    #[doc = "0x0c - desc SQR"]
    pub sqr: SQR,
    #[doc = "0x10 - Control register2"]
    pub cr2: CR2,
    #[doc = "0x14 - desc VTH"]
    pub vth: VTH,
    #[doc = "0x18 - desc VTL"]
    pub vtl: VTL,
    #[doc = "0x1c - desc TRIGGER"]
    pub trigger: TRIGGER,
    #[doc = "0x20 - desc RESULT0"]
    pub result0: RESULT0,
    #[doc = "0x24 - desc RESULT1"]
    pub result1: RESULT1,
    #[doc = "0x28 - desc RESULT2"]
    pub result2: RESULT2,
    #[doc = "0x2c - desc RESULT3"]
    pub result3: RESULT3,
    #[doc = "0x30 - desc RESULTACC"]
    pub resultacc: RESULTACC,
    #[doc = "0x34 - Interrupt enable register"]
    pub ier: IER,
    #[doc = "0x38 - Interrupt flag clear register"]
    pub icr: ICR,
    #[doc = "0x3c - Interrupt status register"]
    pub isr: ISR,
    #[doc = "0x40 - desc RESULT140"]
    pub result140: RESULT140,
    #[doc = "0x44 - desc RESULT141"]
    pub result141: RESULT141,
    #[doc = "0x48 - desc RESULT142"]
    pub result142: RESULT142,
    #[doc = "0x4c - desc RESULT143"]
    pub result143: RESULT143,
    _reserved20: [u8; 0x38],
    #[doc = "0x88 - desc ENABLE14"]
    pub enable14: ENABLE14,
}
#[doc = "CR0 (rw) register accessor: Control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`]
module"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "Control register0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: Control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control register1"]
pub mod cr1;
#[doc = "START (rw) register accessor: desc START\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "desc START"]
pub mod start;
#[doc = "SQR (rw) register accessor: desc SQR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr`]
module"]
pub type SQR = crate::Reg<sqr::SQR_SPEC>;
#[doc = "desc SQR"]
pub mod sqr;
#[doc = "CR2 (rw) register accessor: Control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control register2"]
pub mod cr2;
#[doc = "VTH (rw) register accessor: desc VTH\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vth`]
module"]
pub type VTH = crate::Reg<vth::VTH_SPEC>;
#[doc = "desc VTH"]
pub mod vth;
#[doc = "VTL (rw) register accessor: desc VTL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtl`]
module"]
pub type VTL = crate::Reg<vtl::VTL_SPEC>;
#[doc = "desc VTL"]
pub mod vtl;
#[doc = "TRIGGER (rw) register accessor: desc TRIGGER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigger::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`]
module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "desc TRIGGER"]
pub mod trigger;
#[doc = "RESULT0 (r) register accessor: desc RESULT0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result0`]
module"]
pub type RESULT0 = crate::Reg<result0::RESULT0_SPEC>;
#[doc = "desc RESULT0"]
pub mod result0;
#[doc = "RESULT1 (r) register accessor: desc RESULT1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result1`]
module"]
pub type RESULT1 = crate::Reg<result1::RESULT1_SPEC>;
#[doc = "desc RESULT1"]
pub mod result1;
#[doc = "RESULT2 (r) register accessor: desc RESULT2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result2`]
module"]
pub type RESULT2 = crate::Reg<result2::RESULT2_SPEC>;
#[doc = "desc RESULT2"]
pub mod result2;
#[doc = "RESULT3 (r) register accessor: desc RESULT3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result3`]
module"]
pub type RESULT3 = crate::Reg<result3::RESULT3_SPEC>;
#[doc = "desc RESULT3"]
pub mod result3;
#[doc = "RESULTACC (r) register accessor: desc RESULTACC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resultacc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resultacc`]
module"]
pub type RESULTACC = crate::Reg<resultacc::RESULTACC_SPEC>;
#[doc = "desc RESULTACC"]
pub mod resultacc;
#[doc = "IER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "ISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod icr;
#[doc = "RESULT140 (r) register accessor: desc RESULT140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result140::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result140`]
module"]
pub type RESULT140 = crate::Reg<result140::RESULT140_SPEC>;
#[doc = "desc RESULT140"]
pub mod result140;
#[doc = "RESULT141 (r) register accessor: desc RESULT141\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result141::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result141`]
module"]
pub type RESULT141 = crate::Reg<result141::RESULT141_SPEC>;
#[doc = "desc RESULT141"]
pub mod result141;
#[doc = "RESULT142 (r) register accessor: desc RESULT142\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result142::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result142`]
module"]
pub type RESULT142 = crate::Reg<result142::RESULT142_SPEC>;
#[doc = "desc RESULT142"]
pub mod result142;
#[doc = "RESULT143 (r) register accessor: desc RESULT143\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result143::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result143`]
module"]
pub type RESULT143 = crate::Reg<result143::RESULT143_SPEC>;
#[doc = "desc RESULT143"]
pub mod result143;
#[doc = "ENABLE14 (w) register accessor: desc ENABLE14\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable14::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable14`]
module"]
pub type ENABLE14 = crate::Reg<enable14::ENABLE14_SPEC>;
#[doc = "desc ENABLE14"]
pub mod enable14;
