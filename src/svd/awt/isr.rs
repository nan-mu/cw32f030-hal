#[doc = "Register `ISR` reader"]
pub type R = crate::svd::R<ISR_SPEC>;
#[doc = "Field `UD` reader - desc UD"]
pub type UD_R = crate::svd::BitReader;
impl R {
    #[doc = "Bit 3 - desc UD"]
    #[inline(always)]
    pub fn ud(&self) -> UD_R {
        UD_R::new(((self.bits >> 3) & 1) != 0)
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
