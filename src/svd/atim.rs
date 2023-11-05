#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc ARR"]
    pub arr: ARR,
    #[doc = "0x04 - desc CNT"]
    pub cnt: CNT,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - desc CR"]
    pub cr: CR,
    #[doc = "0x10 - desc ISR"]
    pub isr: ISR,
    #[doc = "0x14 - desc ICR"]
    pub icr: ICR,
    #[doc = "0x18 - desc MSCR"]
    pub mscr: MSCR,
    #[doc = "0x1c - desc FLTR"]
    pub fltr: FLTR,
    #[doc = "0x20 - desc TRIG"]
    pub trig: TRIG,
    #[doc = "0x24 - desc CH1CR"]
    pub ch1cr: CH1CR,
    #[doc = "0x28 - desc CH2CR"]
    pub ch2cr: CH2CR,
    #[doc = "0x2c - desc CH3CR"]
    pub ch3cr: CH3CR,
    #[doc = "0x30 - desc DTR"]
    pub dtr: DTR,
    #[doc = "0x34 - desc RCR"]
    pub rcr: RCR,
    _reserved13: [u8; 0x04],
    #[doc = "0x3c - desc CH1CCRA"]
    pub ch1ccra: CH1CCRA,
    #[doc = "0x40 - desc CH1CCRB"]
    pub ch1ccrb: CH1CCRB,
    #[doc = "0x44 - desc CH2CCRA"]
    pub ch2ccra: CH2CCRA,
    #[doc = "0x48 - desc CH2CCRB"]
    pub ch2ccrb: CH2CCRB,
    #[doc = "0x4c - desc CH3CCRA"]
    pub ch3ccra: CH3CCRA,
    #[doc = "0x50 - desc CH3CCRB"]
    pub ch3ccrb: CH3CCRB,
    #[doc = "0x54 - desc CH4CCR"]
    pub ch4ccr: CH4CCR,
    #[doc = "0x58 - desc CH4CR"]
    pub ch4cr: CH4CR,
}
#[doc = "ARR (rw) register accessor: desc ARR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`arr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`]
module"]
pub type ARR = crate::svd::Reg<arr::ARR_SPEC>;
#[doc = "desc ARR"]
pub mod arr;
#[doc = "CNT (rw) register accessor: desc CNT\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::svd::Reg<cnt::CNT_SPEC>;
#[doc = "desc CNT"]
pub mod cnt;
#[doc = "CR (rw) register accessor: desc CR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::svd::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "ISR (r) register accessor: desc ISR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::svd::Reg<isr::ISR_SPEC>;
#[doc = "desc ISR"]
pub mod isr;
#[doc = "ICR (rw) register accessor: desc ICR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::svd::Reg<icr::ICR_SPEC>;
#[doc = "desc ICR"]
pub mod icr;
#[doc = "MSCR (rw) register accessor: desc MSCR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`mscr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`mscr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mscr`]
module"]
pub type MSCR = crate::svd::Reg<mscr::MSCR_SPEC>;
#[doc = "desc MSCR"]
pub mod mscr;
#[doc = "FLTR (rw) register accessor: desc FLTR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`fltr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`fltr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltr`]
module"]
pub type FLTR = crate::svd::Reg<fltr::FLTR_SPEC>;
#[doc = "desc FLTR"]
pub mod fltr;
#[doc = "TRIG (rw) register accessor: desc TRIG\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`trig::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`trig::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig`]
module"]
pub type TRIG = crate::svd::Reg<trig::TRIG_SPEC>;
#[doc = "desc TRIG"]
pub mod trig;
#[doc = "CH1CR (rw) register accessor: desc CH1CR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch1cr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch1cr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cr`]
module"]
pub type CH1CR = crate::svd::Reg<ch1cr::CH1CR_SPEC>;
#[doc = "desc CH1CR"]
pub mod ch1cr;
#[doc = "CH2CR (rw) register accessor: desc CH2CR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch2cr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch2cr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cr`]
module"]
pub type CH2CR = crate::svd::Reg<ch2cr::CH2CR_SPEC>;
#[doc = "desc CH2CR"]
pub mod ch2cr;
#[doc = "CH3CR (rw) register accessor: desc CH3CR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch3cr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch3cr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cr`]
module"]
pub type CH3CR = crate::svd::Reg<ch3cr::CH3CR_SPEC>;
#[doc = "desc CH3CR"]
pub mod ch3cr;
#[doc = "CH4CR (rw) register accessor: desc CH4CR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch4cr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch4cr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cr`]
module"]
pub type CH4CR = crate::svd::Reg<ch4cr::CH4CR_SPEC>;
#[doc = "desc CH4CR"]
pub mod ch4cr;
#[doc = "DTR (rw) register accessor: desc DTR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dtr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dtr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtr`]
module"]
pub type DTR = crate::svd::Reg<dtr::DTR_SPEC>;
#[doc = "desc DTR"]
pub mod dtr;
#[doc = "RCR (rw) register accessor: desc RCR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
pub type RCR = crate::svd::Reg<rcr::RCR_SPEC>;
#[doc = "desc RCR"]
pub mod rcr;
#[doc = "CH1CCRA (rw) register accessor: desc CH1CCRA\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch1ccra::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch1ccra::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ccra`]
module"]
pub type CH1CCRA = crate::svd::Reg<ch1ccra::CH1CCRA_SPEC>;
#[doc = "desc CH1CCRA"]
pub mod ch1ccra;
#[doc = "CH1CCRB (rw) register accessor: desc CH1CCRB\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch1ccrb::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch1ccrb::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ccrb`]
module"]
pub type CH1CCRB = crate::svd::Reg<ch1ccrb::CH1CCRB_SPEC>;
#[doc = "desc CH1CCRB"]
pub mod ch1ccrb;
#[doc = "CH2CCRA (rw) register accessor: desc CH2CCRA\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch2ccra::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch2ccra::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ccra`]
module"]
pub type CH2CCRA = crate::svd::Reg<ch2ccra::CH2CCRA_SPEC>;
#[doc = "desc CH2CCRA"]
pub mod ch2ccra;
#[doc = "CH2CCRB (rw) register accessor: desc CH2CCRB\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch2ccrb::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch2ccrb::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ccrb`]
module"]
pub type CH2CCRB = crate::svd::Reg<ch2ccrb::CH2CCRB_SPEC>;
#[doc = "desc CH2CCRB"]
pub mod ch2ccrb;
#[doc = "CH3CCRA (rw) register accessor: desc CH3CCRA\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch3ccra::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch3ccra::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ccra`]
module"]
pub type CH3CCRA = crate::svd::Reg<ch3ccra::CH3CCRA_SPEC>;
#[doc = "desc CH3CCRA"]
pub mod ch3ccra;
#[doc = "CH3CCRB (rw) register accessor: desc CH3CCRB\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch3ccrb::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch3ccrb::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ccrb`]
module"]
pub type CH3CCRB = crate::svd::Reg<ch3ccrb::CH3CCRB_SPEC>;
#[doc = "desc CH3CCRB"]
pub mod ch3ccrb;
#[doc = "CH4CCR (rw) register accessor: desc CH4CCR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch4ccr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch4ccr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4ccr`]
module"]
pub type CH4CCR = crate::svd::Reg<ch4ccr::CH4CCR_SPEC>;
#[doc = "desc CH4CCR"]
pub mod ch4ccr;
