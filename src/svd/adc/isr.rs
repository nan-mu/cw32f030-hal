#[doc = "Register `ISR` reader"]
pub type R = crate::svd::R<ISR_SPEC>;
#[doc = "Field `EOC` reader - desc EOC"]
pub type EOC_R = crate::svd::BitReader;
#[doc = "Field `EOS` reader - desc EOS"]
pub type EOS_R = crate::svd::BitReader;
#[doc = "Field `EOA` reader - desc EOA"]
pub type EOA_R = crate::svd::BitReader;
#[doc = "Field `WDTL` reader - desc WDTL"]
pub type WDTL_R = crate::svd::BitReader;
#[doc = "Field `WDTH` reader - desc WDTH"]
pub type WDTH_R = crate::svd::BitReader;
#[doc = "Field `WDTR` reader - desc WDTR"]
pub type WDTR_R = crate::svd::BitReader;
#[doc = "Field `OVW` reader - desc OVW"]
pub type OVW_R = crate::svd::BitReader;
#[doc = "Field `READY` reader - desc READY"]
pub type READY_R = crate::svd::BitReader;
impl R {
    #[doc = "Bit 0 - desc EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EOA"]
    #[inline(always)]
    pub fn eoa(&self) -> EOA_R {
        EOA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc WDTL"]
    #[inline(always)]
    pub fn wdtl(&self) -> WDTL_R {
        WDTL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc WDTH"]
    #[inline(always)]
    pub fn wdth(&self) -> WDTH_R {
        WDTH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc WDTR"]
    #[inline(always)]
    pub fn wdtr(&self) -> WDTR_R {
        WDTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc OVW"]
    #[inline(always)]
    pub fn ovw(&self) -> OVW_R {
        OVW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::svd::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::svd::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::svd::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
