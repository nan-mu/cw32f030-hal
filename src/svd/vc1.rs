#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc DIV"]
    pub div: DIV,
    #[doc = "0x04 - Control register0"]
    pub cr0: CR0,
    #[doc = "0x08 - Control register1"]
    pub cr1: CR1,
    #[doc = "0x0c - Status register"]
    pub sr: SR,
}
#[doc = "DIV (rw) register accessor: desc DIV\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
pub type DIV = crate::svd::Reg<div::DIV_SPEC>;
#[doc = "desc DIV"]
pub mod div;
#[doc = "CR0 (rw) register accessor: Control register0\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr0::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`]
module"]
pub type CR0 = crate::svd::Reg<cr0::CR0_SPEC>;
#[doc = "Control register0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: Control register1\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::svd::Reg<cr1::CR1_SPEC>;
#[doc = "Control register1"]
pub mod cr1;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::svd::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
