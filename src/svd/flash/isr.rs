#[doc = "Register `ISR` reader"]
pub type R = crate::svd::R<ISR_SPEC>;
#[doc = "Field `PC` reader - desc PC"]
pub type PC_R = crate::svd::BitReader;
#[doc = "Field `PAGELOCK` reader - desc PAGELOCK"]
pub type PAGELOCK_R = crate::svd::BitReader;
#[doc = "Field `PROG` reader - desc PROG"]
pub type PROG_R = crate::svd::BitReader;
impl R {
    #[doc = "Bit 0 - desc PC"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PAGELOCK"]
    #[inline(always)]
    pub fn pagelock(&self) -> PAGELOCK_R {
        PAGELOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PROG"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 4) & 1) != 0)
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
