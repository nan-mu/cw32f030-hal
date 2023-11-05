#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc BRREN"]
    pub brren: BRREN,
    #[doc = "0x04 - desc BRR"]
    pub brr: BRR,
    #[doc = "0x08 - Control register"]
    pub cr: CR,
    #[doc = "0x0c - Data register"]
    pub dr: DR,
    #[doc = "0x10 - Slave Addrress0"]
    pub addr0: ADDR0,
    #[doc = "0x14 - Status register"]
    pub stat: STAT,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Slave Addrress1"]
    pub addr1: ADDR1,
    #[doc = "0x24 - Slave Addrress2"]
    pub addr2: ADDR2,
    #[doc = "0x28 - Slave Addrress match flag"]
    pub match_: MATCH,
}
#[doc = "BRREN (rw) register accessor: desc BRREN\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`brren::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`brren::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brren`]
module"]
pub type BRREN = crate::svd::Reg<brren::BRREN_SPEC>;
#[doc = "desc BRREN"]
pub mod brren;
#[doc = "BRR (rw) register accessor: desc BRR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`brr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
pub type BRR = crate::svd::Reg<brr::BRR_SPEC>;
#[doc = "desc BRR"]
pub mod brr;
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::svd::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "DR (rw) register accessor: Data register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::svd::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
#[doc = "ADDR0 (rw) register accessor: Slave Addrress0\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`addr0::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`addr0::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr0`]
module"]
pub type ADDR0 = crate::svd::Reg<addr0::ADDR0_SPEC>;
#[doc = "Slave Addrress0"]
pub mod addr0;
#[doc = "STAT (r) register accessor: Status register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::svd::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "ADDR1 (rw) register accessor: Slave Addrress1\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`addr1::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`addr1::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr1`]
module"]
pub type ADDR1 = crate::svd::Reg<addr1::ADDR1_SPEC>;
#[doc = "Slave Addrress1"]
pub mod addr1;
#[doc = "ADDR2 (rw) register accessor: Slave Addrress2\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`addr2::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`addr2::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr2`]
module"]
pub type ADDR2 = crate::svd::Reg<addr2::ADDR2_SPEC>;
#[doc = "Slave Addrress2"]
pub mod addr2;
#[doc = "MATCH (r) register accessor: Slave Addrress match flag\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`match_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match_`]
module"]
pub type MATCH = crate::svd::Reg<match_::MATCH_SPEC>;
#[doc = "Slave Addrress match flag"]
pub mod match_;
