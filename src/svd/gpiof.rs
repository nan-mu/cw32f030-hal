#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc DIR"]
    pub dir: DIR,
    #[doc = "0x04 - desc OPENDRAIN"]
    pub opendrain: OPENDRAIN,
    #[doc = "0x08 - desc SPEED"]
    pub speed: SPEED,
    #[doc = "0x0c - desc PDR"]
    pub pdr: PDR,
    #[doc = "0x10 - desc PUR"]
    pub pur: PUR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - desc AFRL"]
    pub afrl: AFRL,
    #[doc = "0x1c - desc ANALOG"]
    pub analog: ANALOG,
    #[doc = "0x20 - desc DRIVER"]
    pub driver: DRIVER,
    #[doc = "0x24 - Interrupt enable register"]
    pub riseie: RISEIE,
    #[doc = "0x28 - Interrupt enable register"]
    pub fallie: FALLIE,
    #[doc = "0x2c - Interrupt enable register"]
    pub highie: HIGHIE,
    #[doc = "0x30 - Interrupt enable register"]
    pub lowie: LOWIE,
    #[doc = "0x34 - Interrupt status register"]
    pub isr: ISR,
    #[doc = "0x38 - Interrupt flag clear register"]
    pub icr: ICR,
    #[doc = "0x3c - desc LOCK"]
    pub lock: LOCK,
    #[doc = "0x40 - desc FILTER"]
    pub filter: FILTER,
    _reserved16: [u8; 0x0c],
    #[doc = "0x50 - desc IDR"]
    pub idr: IDR,
    _reserved_17_odr: [u8; 0x04],
    #[doc = "0x58 - desc BRR"]
    pub brr: BRR,
    #[doc = "0x5c - desc BSRR"]
    pub bsrr: BSRR,
    #[doc = "0x60 - desc TOG"]
    pub tog: TOG,
}
impl RegisterBlock {
    #[doc = "0x54 - desc ODRLOWBYTE"]
    #[inline(always)]
    pub const fn odrlowbyte(&self) -> &ODRLOWBYTE {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
    #[doc = "0x54 - desc ODR"]
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
}
#[doc = "DIR (rw) register accessor: desc DIR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dir::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
pub type DIR = crate::svd::Reg<dir::DIR_SPEC>;
#[doc = "desc DIR"]
pub mod dir;
#[doc = "OPENDRAIN (rw) register accessor: desc OPENDRAIN\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`opendrain::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`opendrain::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opendrain`]
module"]
pub type OPENDRAIN = crate::svd::Reg<opendrain::OPENDRAIN_SPEC>;
#[doc = "desc OPENDRAIN"]
pub mod opendrain;
#[doc = "SPEED (rw) register accessor: desc SPEED\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`speed::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`speed::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@speed`]
module"]
pub type SPEED = crate::svd::Reg<speed::SPEED_SPEC>;
#[doc = "desc SPEED"]
pub mod speed;
#[doc = "PDR (rw) register accessor: desc PDR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`pdr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`pdr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdr`]
module"]
pub type PDR = crate::svd::Reg<pdr::PDR_SPEC>;
#[doc = "desc PDR"]
pub mod pdr;
#[doc = "PUR (rw) register accessor: desc PUR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`pur::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`pur::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pur`]
module"]
pub type PUR = crate::svd::Reg<pur::PUR_SPEC>;
#[doc = "desc PUR"]
pub mod pur;
#[doc = "AFRL (rw) register accessor: desc AFRL\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`afrl::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`afrl::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrl`]
module"]
pub type AFRL = crate::svd::Reg<afrl::AFRL_SPEC>;
#[doc = "desc AFRL"]
pub mod afrl;
#[doc = "ANALOG (rw) register accessor: desc ANALOG\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`analog::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`analog::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog`]
module"]
pub type ANALOG = crate::svd::Reg<analog::ANALOG_SPEC>;
#[doc = "desc ANALOG"]
pub mod analog;
#[doc = "DRIVER (rw) register accessor: desc DRIVER\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`driver::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`driver::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@driver`]
module"]
pub type DRIVER = crate::svd::Reg<driver::DRIVER_SPEC>;
#[doc = "desc DRIVER"]
pub mod driver;
#[doc = "RISEIE (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`riseie::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`riseie::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riseie`]
module"]
pub type RISEIE = crate::svd::Reg<riseie::RISEIE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod riseie;
#[doc = "FALLIE (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`fallie::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`fallie::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fallie`]
module"]
pub type FALLIE = crate::svd::Reg<fallie::FALLIE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod fallie;
#[doc = "HIGHIE (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`highie::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`highie::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@highie`]
module"]
pub type HIGHIE = crate::svd::Reg<highie::HIGHIE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod highie;
#[doc = "LOWIE (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`lowie::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`lowie::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowie`]
module"]
pub type LOWIE = crate::svd::Reg<lowie::LOWIE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod lowie;
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
#[doc = "LOCK (rw) register accessor: desc LOCK\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::svd::Reg<lock::LOCK_SPEC>;
#[doc = "desc LOCK"]
pub mod lock;
#[doc = "FILTER (rw) register accessor: desc FILTER\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`filter::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`filter::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter`]
module"]
pub type FILTER = crate::svd::Reg<filter::FILTER_SPEC>;
#[doc = "desc FILTER"]
pub mod filter;
#[doc = "IDR (rw) register accessor: desc IDR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`idr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`idr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::svd::Reg<idr::IDR_SPEC>;
#[doc = "desc IDR"]
pub mod idr;
#[doc = "ODR (rw) register accessor: desc ODR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`odr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`]
module"]
pub type ODR = crate::svd::Reg<odr::ODR_SPEC>;
#[doc = "desc ODR"]
pub mod odr;
#[doc = "ODRLOWBYTE (rw) register accessor: desc ODRLOWBYTE\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`odrlowbyte::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`odrlowbyte::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odrlowbyte`]
module"]
pub type ODRLOWBYTE = crate::svd::Reg<odrlowbyte::ODRLOWBYTE_SPEC>;
#[doc = "desc ODRLOWBYTE"]
pub mod odrlowbyte;
#[doc = "BRR (rw) register accessor: desc BRR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`brr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
pub type BRR = crate::svd::Reg<brr::BRR_SPEC>;
#[doc = "desc BRR"]
pub mod brr;
#[doc = "BSRR (rw) register accessor: desc BSRR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`bsrr::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`bsrr::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`]
module"]
pub type BSRR = crate::svd::Reg<bsrr::BSRR_SPEC>;
#[doc = "desc BSRR"]
pub mod bsrr;
#[doc = "TOG (rw) register accessor: desc TOG\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`tog::R`].  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`tog::W`]. You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tog`]
module"]
pub type TOG = crate::svd::Reg<tog::TOG_SPEC>;
#[doc = "desc TOG"]
pub mod tog;
