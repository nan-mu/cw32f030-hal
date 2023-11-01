#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc KEY"]
    pub key: KEY,
    #[doc = "0x04 - Control register0"]
    pub cr0: CR0,
    #[doc = "0x08 - Control register1"]
    pub cr1: CR1,
    #[doc = "0x0c - Control register2"]
    pub cr2: CR2,
    #[doc = "0x10 - Compen register"]
    pub compen: COMPEN,
    #[doc = "0x14 - Time.Second register"]
    pub date: DATE,
    #[doc = "0x18 - Time.Second register"]
    pub time: TIME,
    #[doc = "0x1c - Alarm - A"]
    pub alarma: ALARMA,
    #[doc = "0x20 - Alarm - B"]
    pub alarmb: ALARMB,
    #[doc = "0x24 - desc TAMPDATE"]
    pub tampdate: TAMPDATE,
    #[doc = "0x28 - desc TAMPTIME"]
    pub tamptime: TAMPTIME,
    #[doc = "0x2c - Auto Wakeup Timer Auto Reload Register"]
    pub awtarr: AWTARR,
    #[doc = "0x30 - Interrupt enable register"]
    pub ier: IER,
    #[doc = "0x34 - Interrupt status register"]
    pub isr: ISR,
    #[doc = "0x38 - Interrupt flag clear register"]
    pub icr: ICR,
}
#[doc = "KEY (w) register accessor: desc KEY\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`]
module"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "desc KEY"]
pub mod key;
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
#[doc = "CR2 (rw) register accessor: Control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control register2"]
pub mod cr2;
#[doc = "COMPEN (rw) register accessor: Compen register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compen`]
module"]
pub type COMPEN = crate::Reg<compen::COMPEN_SPEC>;
#[doc = "Compen register"]
pub mod compen;
#[doc = "DATE (rw) register accessor: Time.Second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`]
module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Time.Second register"]
pub mod date;
#[doc = "TIME (rw) register accessor: Time.Second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`]
module"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "Time.Second register"]
pub mod time;
#[doc = "ALARMA (rw) register accessor: Alarm - A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarma`]
module"]
pub type ALARMA = crate::Reg<alarma::ALARMA_SPEC>;
#[doc = "Alarm - A"]
pub mod alarma;
#[doc = "ALARMB (rw) register accessor: Alarm - B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarmb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarmb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarmb`]
module"]
pub type ALARMB = crate::Reg<alarmb::ALARMB_SPEC>;
#[doc = "Alarm - B"]
pub mod alarmb;
#[doc = "TAMPDATE (rw) register accessor: desc TAMPDATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tampdate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tampdate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampdate`]
module"]
pub type TAMPDATE = crate::Reg<tampdate::TAMPDATE_SPEC>;
#[doc = "desc TAMPDATE"]
pub mod tampdate;
#[doc = "TAMPTIME (rw) register accessor: desc TAMPTIME\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamptime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamptime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamptime`]
module"]
pub type TAMPTIME = crate::Reg<tamptime::TAMPTIME_SPEC>;
#[doc = "desc TAMPTIME"]
pub mod tamptime;
#[doc = "AWTARR (rw) register accessor: Auto Wakeup Timer Auto Reload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awtarr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awtarr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awtarr`]
module"]
pub type AWTARR = crate::Reg<awtarr::AWTARR_SPEC>;
#[doc = "Auto Wakeup Timer Auto Reload Register"]
pub mod awtarr;
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
