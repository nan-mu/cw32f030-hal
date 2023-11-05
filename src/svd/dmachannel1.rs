#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel.y control and status register"]
    pub csr: CSR,
    #[doc = "0x04 - Channel.y counter register"]
    pub cnt: CNT,
    #[doc = "0x08 - Channel.y source address register"]
    pub srcaddr: SRCADDR,
    #[doc = "0x0c - Channel.y destination address register"]
    pub dstaddr: DSTADDR,
    #[doc = "0x10 - Channel.y trigger register"]
    pub trig: TRIG,
}
#[doc = "CSR (rw) register accessor: Channel.y control and status register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::svd::Reg<csr::CSR_SPEC>;
#[doc = "Channel.y control and status register"]
pub mod csr;
#[doc = "CNT (rw) register accessor: Channel.y counter register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::svd::Reg<cnt::CNT_SPEC>;
#[doc = "Channel.y counter register"]
pub mod cnt;
#[doc = "SRCADDR (rw) register accessor: Channel.y source address register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`srcaddr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`srcaddr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr`]
module"]
pub type SRCADDR = crate::svd::Reg<srcaddr::SRCADDR_SPEC>;
#[doc = "Channel.y source address register"]
pub mod srcaddr;
#[doc = "DSTADDR (rw) register accessor: Channel.y destination address register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dstaddr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dstaddr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr`]
module"]
pub type DSTADDR = crate::svd::Reg<dstaddr::DSTADDR_SPEC>;
#[doc = "Channel.y destination address register"]
pub mod dstaddr;
#[doc = "TRIG (rw) register accessor: Channel.y trigger register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`trig::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`trig::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig`]
module"]
pub type TRIG = crate::svd::Reg<trig::TRIG_SPEC>;
#[doc = "Channel.y trigger register"]
pub mod trig;
