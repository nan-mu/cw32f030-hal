#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Reg0"]
    pub cr0: CR0,
    #[doc = "0x04 - Control Reg1"]
    pub cr1: CR1,
    #[doc = "0x08 - Control Reg2"]
    pub cr2: CR2,
    #[doc = "0x0c - Interupt Enable Reg"]
    pub ier: IER,
    #[doc = "0x10 - Interupt Status Reg"]
    pub isr: ISR,
    #[doc = "0x14 - Interupt Clear Reg"]
    pub icr: ICR,
    #[doc = "0x18 - HSI Control Reg"]
    pub hsi: HSI,
    #[doc = "0x1c - HSE Control Reg"]
    pub hse: HSE,
    #[doc = "0x20 - LSI Control Reg"]
    pub lsi: LSI,
    #[doc = "0x24 - LSE Control Reg"]
    pub lse: LSE,
    #[doc = "0x28 - PLL Control Reg"]
    pub pll: PLL,
    #[doc = "0x2c - Debug Control Reg"]
    pub debug: DEBUG,
    #[doc = "0x30 - AHB Clock Control Reg"]
    pub ahben: AHBEN,
    #[doc = "0x34 - APB Clock Control Reg2"]
    pub apben2: APBEN2,
    #[doc = "0x38 - APB Clock Control Reg1"]
    pub apben1: APBEN1,
    _reserved15: [u8; 0x04],
    #[doc = "0x40 - AHB Reset Control Reg"]
    pub ahbrst: AHBRST,
    #[doc = "0x44 - APB Reset Control Reg2"]
    pub apbrst2: APBRST2,
    #[doc = "0x48 - APB Reset Control Reg1"]
    pub apbrst1: APBRST1,
    #[doc = "0x4c - Reset Status Reg"]
    pub resetflag: RESETFLAG,
    #[doc = "0x50 - GTIM1 CAP Control Reg"]
    pub gtim1cap: GTIM1CAP,
    #[doc = "0x54 - GTIM2 CAP Control Reg"]
    pub gtim2cap: GTIM2CAP,
    #[doc = "0x58 - GTIM3 CAP Control Reg"]
    pub gtim3cap: GTIM3CAP,
    #[doc = "0x5c - GTIM4 CAP Control Reg"]
    pub gtim4cap: GTIM4CAP,
    #[doc = "0x60 - ATIM ETR Control Reg"]
    pub atimetr: ATIMETR,
    #[doc = "0x64 - GTIM1-4 ETR Control Reg"]
    pub gtimetr: GTIMETR,
    _reserved25: [u8; 0x04],
    #[doc = "0x6c - BTIMx GTIMx ATIM ITR Control Reg"]
    pub timitr: TIMITR,
    #[doc = "0x70 - Master Clock Output Control Reg"]
    pub mco: MCO,
    #[doc = "0x74 - IR MOD Control Reg"]
    pub irmod: IRMOD,
}
#[doc = "CR0 (rw) register accessor: Control Reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`]
module"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "Control Reg0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: Control Reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control Reg1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control Reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control Reg2"]
pub mod cr2;
#[doc = "IER (rw) register accessor: Interupt Enable Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interupt Enable Reg"]
pub mod ier;
#[doc = "ISR (r) register accessor: Interupt Status Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interupt Status Reg"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interupt Clear Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interupt Clear Reg"]
pub mod icr;
#[doc = "HSI (rw) register accessor: HSI Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsi`]
module"]
pub type HSI = crate::Reg<hsi::HSI_SPEC>;
#[doc = "HSI Control Reg"]
pub mod hsi;
#[doc = "HSE (rw) register accessor: HSE Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hse`]
module"]
pub type HSE = crate::Reg<hse::HSE_SPEC>;
#[doc = "HSE Control Reg"]
pub mod hse;
#[doc = "LSI (rw) register accessor: LSI Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsi`]
module"]
pub type LSI = crate::Reg<lsi::LSI_SPEC>;
#[doc = "LSI Control Reg"]
pub mod lsi;
#[doc = "LSE (rw) register accessor: LSE Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lse`]
module"]
pub type LSE = crate::Reg<lse::LSE_SPEC>;
#[doc = "LSE Control Reg"]
pub mod lse;
#[doc = "PLL (rw) register accessor: PLL Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll`]
module"]
pub type PLL = crate::Reg<pll::PLL_SPEC>;
#[doc = "PLL Control Reg"]
pub mod pll;
#[doc = "DEBUG (rw) register accessor: Debug Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`]
module"]
pub type DEBUG = crate::Reg<debug::DEBUG_SPEC>;
#[doc = "Debug Control Reg"]
pub mod debug;
#[doc = "AHBEN (rw) register accessor: AHB Clock Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben`]
module"]
pub type AHBEN = crate::Reg<ahben::AHBEN_SPEC>;
#[doc = "AHB Clock Control Reg"]
pub mod ahben;
#[doc = "APBEN2 (rw) register accessor: APB Clock Control Reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apben2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apben2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apben2`]
module"]
pub type APBEN2 = crate::Reg<apben2::APBEN2_SPEC>;
#[doc = "APB Clock Control Reg2"]
pub mod apben2;
#[doc = "APBEN1 (rw) register accessor: APB Clock Control Reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apben1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apben1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apben1`]
module"]
pub type APBEN1 = crate::Reg<apben1::APBEN1_SPEC>;
#[doc = "APB Clock Control Reg1"]
pub mod apben1;
#[doc = "AHBRST (rw) register accessor: AHB Reset Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst`]
module"]
pub type AHBRST = crate::Reg<ahbrst::AHBRST_SPEC>;
#[doc = "AHB Reset Control Reg"]
pub mod ahbrst;
#[doc = "APBRST2 (rw) register accessor: APB Reset Control Reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbrst2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbrst2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrst2`]
module"]
pub type APBRST2 = crate::Reg<apbrst2::APBRST2_SPEC>;
#[doc = "APB Reset Control Reg2"]
pub mod apbrst2;
#[doc = "APBRST1 (rw) register accessor: APB Reset Control Reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbrst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbrst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrst1`]
module"]
pub type APBRST1 = crate::Reg<apbrst1::APBRST1_SPEC>;
#[doc = "APB Reset Control Reg1"]
pub mod apbrst1;
#[doc = "RESETFLAG (rw) register accessor: Reset Status Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetflag`]
module"]
pub type RESETFLAG = crate::Reg<resetflag::RESETFLAG_SPEC>;
#[doc = "Reset Status Reg"]
pub mod resetflag;
#[doc = "GTIM1CAP (rw) register accessor: GTIM1 CAP Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtim1cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtim1cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtim1cap`]
module"]
pub type GTIM1CAP = crate::Reg<gtim1cap::GTIM1CAP_SPEC>;
#[doc = "GTIM1 CAP Control Reg"]
pub mod gtim1cap;
#[doc = "GTIM2CAP (rw) register accessor: GTIM2 CAP Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtim2cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtim2cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtim2cap`]
module"]
pub type GTIM2CAP = crate::Reg<gtim2cap::GTIM2CAP_SPEC>;
#[doc = "GTIM2 CAP Control Reg"]
pub mod gtim2cap;
#[doc = "GTIM3CAP (rw) register accessor: GTIM3 CAP Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtim3cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtim3cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtim3cap`]
module"]
pub type GTIM3CAP = crate::Reg<gtim3cap::GTIM3CAP_SPEC>;
#[doc = "GTIM3 CAP Control Reg"]
pub mod gtim3cap;
#[doc = "GTIM4CAP (rw) register accessor: GTIM4 CAP Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtim4cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtim4cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtim4cap`]
module"]
pub type GTIM4CAP = crate::Reg<gtim4cap::GTIM4CAP_SPEC>;
#[doc = "GTIM4 CAP Control Reg"]
pub mod gtim4cap;
#[doc = "ATIMETR (rw) register accessor: ATIM ETR Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atimetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atimetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atimetr`]
module"]
pub type ATIMETR = crate::Reg<atimetr::ATIMETR_SPEC>;
#[doc = "ATIM ETR Control Reg"]
pub mod atimetr;
#[doc = "GTIMETR (rw) register accessor: GTIM1-4 ETR Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtimetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtimetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtimetr`]
module"]
pub type GTIMETR = crate::Reg<gtimetr::GTIMETR_SPEC>;
#[doc = "GTIM1-4 ETR Control Reg"]
pub mod gtimetr;
#[doc = "TIMITR (rw) register accessor: BTIMx GTIMx ATIM ITR Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timitr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timitr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timitr`]
module"]
pub type TIMITR = crate::Reg<timitr::TIMITR_SPEC>;
#[doc = "BTIMx GTIMx ATIM ITR Control Reg"]
pub mod timitr;
#[doc = "MCO (rw) register accessor: Master Clock Output Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mco::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mco::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mco`]
module"]
pub type MCO = crate::Reg<mco::MCO_SPEC>;
#[doc = "Master Clock Output Control Reg"]
pub mod mco;
#[doc = "IRMOD (rw) register accessor: IR MOD Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irmod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irmod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irmod`]
module"]
pub type IRMOD = crate::Reg<irmod::IRMOD_SPEC>;
#[doc = "IR MOD Control Reg"]
pub mod irmod;
