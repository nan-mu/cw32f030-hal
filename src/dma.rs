#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt status register"]
    pub isr: ISR,
    #[doc = "0x04 - Interrupt flag clear register"]
    pub icr: ICR,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - Channel1 control and status register"]
    pub csr1: CSR1,
    #[doc = "0x24 - Channel1 counter register"]
    pub cnt1: CNT1,
    #[doc = "0x28 - Channel1 source address register"]
    pub srcaddr1: SRCADDR1,
    #[doc = "0x2c - Channel1 destination address register"]
    pub dstaddr1: DSTADDR1,
    #[doc = "0x30 - Channel1 trigger register"]
    pub trig1: TRIG1,
    _reserved7: [u8; 0x0c],
    #[doc = "0x40 - Channel2 control and status register"]
    pub csr2: CSR2,
    #[doc = "0x44 - Channel2 counter register"]
    pub cnt2: CNT2,
    #[doc = "0x48 - Channel2 source address register"]
    pub srcaddr2: SRCADDR2,
    #[doc = "0x4c - Channel2 destination address register"]
    pub dstaddr2: DSTADDR2,
    #[doc = "0x50 - Channel2 trigger register"]
    pub trig2: TRIG2,
    _reserved12: [u8; 0x0c],
    #[doc = "0x60 - Channel3 control and status register"]
    pub csr3: CSR3,
    #[doc = "0x64 - Channel3 counter register"]
    pub cnt3: CNT3,
    #[doc = "0x68 - Channel3 source address register"]
    pub srcaddr3: SRCADDR3,
    #[doc = "0x6c - Channel3 destination address register"]
    pub dstaddr3: DSTADDR3,
    #[doc = "0x70 - Channel3 trigger register"]
    pub trig3: TRIG3,
    _reserved17: [u8; 0x0c],
    #[doc = "0x80 - Channel4 control and status register"]
    pub csr4: CSR4,
    #[doc = "0x84 - Channel4 counter register"]
    pub cnt4: CNT4,
    #[doc = "0x88 - Channel4 source address register"]
    pub srcaddr4: SRCADDR4,
    #[doc = "0x8c - Channel4 destination address register"]
    pub dstaddr4: DSTADDR4,
    #[doc = "0x90 - Channel4 trigger register"]
    pub trig4: TRIG4,
    _reserved22: [u8; 0x0c],
    #[doc = "0xa0 - Channel5 control and status register"]
    pub csr5: CSR5,
    #[doc = "0xa4 - Channel5 counter register"]
    pub cnt5: CNT5,
    #[doc = "0xa8 - Channel5 source address register"]
    pub srcaddr5: SRCADDR5,
    #[doc = "0xac - Channel5 destination address register"]
    pub dstaddr5: DSTADDR5,
    #[doc = "0xb0 - Channel5 trigger register"]
    pub trig5: TRIG5,
}
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
#[doc = "CSR1 (rw) register accessor: Channel1 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`]
module"]
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
#[doc = "Channel1 control and status register"]
pub mod csr1;
#[doc = "CNT1 (rw) register accessor: Channel1 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt1`]
module"]
pub type CNT1 = crate::Reg<cnt1::CNT1_SPEC>;
#[doc = "Channel1 counter register"]
pub mod cnt1;
#[doc = "SRCADDR1 (rw) register accessor: Channel1 source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr1`]
module"]
pub type SRCADDR1 = crate::Reg<srcaddr1::SRCADDR1_SPEC>;
#[doc = "Channel1 source address register"]
pub mod srcaddr1;
#[doc = "DSTADDR1 (rw) register accessor: Channel1 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr1`]
module"]
pub type DSTADDR1 = crate::Reg<dstaddr1::DSTADDR1_SPEC>;
#[doc = "Channel1 destination address register"]
pub mod dstaddr1;
#[doc = "TRIG1 (rw) register accessor: Channel1 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig1`]
module"]
pub type TRIG1 = crate::Reg<trig1::TRIG1_SPEC>;
#[doc = "Channel1 trigger register"]
pub mod trig1;
#[doc = "CSR2 (rw) register accessor: Channel2 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr2`]
module"]
pub type CSR2 = crate::Reg<csr2::CSR2_SPEC>;
#[doc = "Channel2 control and status register"]
pub mod csr2;
#[doc = "CNT2 (rw) register accessor: Channel2 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt2`]
module"]
pub type CNT2 = crate::Reg<cnt2::CNT2_SPEC>;
#[doc = "Channel2 counter register"]
pub mod cnt2;
#[doc = "SRCADDR2 (rw) register accessor: Channel2 source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr2`]
module"]
pub type SRCADDR2 = crate::Reg<srcaddr2::SRCADDR2_SPEC>;
#[doc = "Channel2 source address register"]
pub mod srcaddr2;
#[doc = "DSTADDR2 (rw) register accessor: Channel2 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr2`]
module"]
pub type DSTADDR2 = crate::Reg<dstaddr2::DSTADDR2_SPEC>;
#[doc = "Channel2 destination address register"]
pub mod dstaddr2;
#[doc = "TRIG2 (rw) register accessor: Channel2 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig2`]
module"]
pub type TRIG2 = crate::Reg<trig2::TRIG2_SPEC>;
#[doc = "Channel2 trigger register"]
pub mod trig2;
#[doc = "CSR3 (rw) register accessor: Channel3 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr3`]
module"]
pub type CSR3 = crate::Reg<csr3::CSR3_SPEC>;
#[doc = "Channel3 control and status register"]
pub mod csr3;
#[doc = "CNT3 (rw) register accessor: Channel3 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt3`]
module"]
pub type CNT3 = crate::Reg<cnt3::CNT3_SPEC>;
#[doc = "Channel3 counter register"]
pub mod cnt3;
#[doc = "SRCADDR3 (rw) register accessor: Channel3 source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcaddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcaddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr3`]
module"]
pub type SRCADDR3 = crate::Reg<srcaddr3::SRCADDR3_SPEC>;
#[doc = "Channel3 source address register"]
pub mod srcaddr3;
#[doc = "DSTADDR3 (rw) register accessor: Channel3 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstaddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstaddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr3`]
module"]
pub type DSTADDR3 = crate::Reg<dstaddr3::DSTADDR3_SPEC>;
#[doc = "Channel3 destination address register"]
pub mod dstaddr3;
#[doc = "TRIG3 (rw) register accessor: Channel3 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig3`]
module"]
pub type TRIG3 = crate::Reg<trig3::TRIG3_SPEC>;
#[doc = "Channel3 trigger register"]
pub mod trig3;
#[doc = "CSR4 (rw) register accessor: Channel4 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr4`]
module"]
pub type CSR4 = crate::Reg<csr4::CSR4_SPEC>;
#[doc = "Channel4 control and status register"]
pub mod csr4;
#[doc = "CNT4 (rw) register accessor: Channel4 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt4`]
module"]
pub type CNT4 = crate::Reg<cnt4::CNT4_SPEC>;
#[doc = "Channel4 counter register"]
pub mod cnt4;
#[doc = "SRCADDR4 (rw) register accessor: Channel4 source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcaddr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcaddr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr4`]
module"]
pub type SRCADDR4 = crate::Reg<srcaddr4::SRCADDR4_SPEC>;
#[doc = "Channel4 source address register"]
pub mod srcaddr4;
#[doc = "DSTADDR4 (rw) register accessor: Channel4 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstaddr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstaddr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr4`]
module"]
pub type DSTADDR4 = crate::Reg<dstaddr4::DSTADDR4_SPEC>;
#[doc = "Channel4 destination address register"]
pub mod dstaddr4;
#[doc = "TRIG4 (rw) register accessor: Channel4 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig4`]
module"]
pub type TRIG4 = crate::Reg<trig4::TRIG4_SPEC>;
#[doc = "Channel4 trigger register"]
pub mod trig4;
#[doc = "CSR5 (rw) register accessor: Channel5 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr5`]
module"]
pub type CSR5 = crate::Reg<csr5::CSR5_SPEC>;
#[doc = "Channel5 control and status register"]
pub mod csr5;
#[doc = "CNT5 (rw) register accessor: Channel5 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt5`]
module"]
pub type CNT5 = crate::Reg<cnt5::CNT5_SPEC>;
#[doc = "Channel5 counter register"]
pub mod cnt5;
#[doc = "SRCADDR5 (rw) register accessor: Channel5 source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcaddr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcaddr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr5`]
module"]
pub type SRCADDR5 = crate::Reg<srcaddr5::SRCADDR5_SPEC>;
#[doc = "Channel5 source address register"]
pub mod srcaddr5;
#[doc = "DSTADDR5 (rw) register accessor: Channel5 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstaddr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstaddr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr5`]
module"]
pub type DSTADDR5 = crate::Reg<dstaddr5::DSTADDR5_SPEC>;
#[doc = "Channel5 destination address register"]
pub mod dstaddr5;
#[doc = "TRIG5 (rw) register accessor: Channel5 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trig5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trig5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig5`]
module"]
pub type TRIG5 = crate::Reg<trig5::TRIG5_SPEC>;
#[doc = "Channel5 trigger register"]
pub mod trig5;
