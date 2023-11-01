#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: CR,
    _reserved1: [u8; 0x04],
    _reserved_1_dr8: [u8; 0x04],
    _reserved_2_result16: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x08 - Data register"]
    #[inline(always)]
    pub const fn dr8(&self) -> &DR8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Data register"]
    #[inline(always)]
    pub const fn dr16(&self) -> &DR16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Data register"]
    #[inline(always)]
    pub const fn dr32(&self) -> &DR32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x0c - Result register"]
    #[inline(always)]
    pub const fn result16(&self) -> &RESULT16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - Result register"]
    #[inline(always)]
    pub const fn result32(&self) -> &RESULT32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
}
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "DR32 (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr32`]
module"]
pub type DR32 = crate::Reg<dr32::DR32_SPEC>;
#[doc = "Data register"]
pub mod dr32;
#[doc = "DR16 (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr16`]
module"]
pub type DR16 = crate::Reg<dr16::DR16_SPEC>;
#[doc = "Data register"]
pub mod dr16;
#[doc = "DR8 (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`]
module"]
pub type DR8 = crate::Reg<dr8::DR8_SPEC>;
#[doc = "Data register"]
pub mod dr8;
#[doc = "RESULT32 (r) register accessor: Result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result32::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result32`]
module"]
pub type RESULT32 = crate::Reg<result32::RESULT32_SPEC>;
#[doc = "Result register"]
pub mod result32;
#[doc = "RESULT16 (r) register accessor: Result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result16`]
module"]
pub type RESULT16 = crate::Reg<result16::RESULT16_SPEC>;
#[doc = "Result register"]
pub mod result16;
