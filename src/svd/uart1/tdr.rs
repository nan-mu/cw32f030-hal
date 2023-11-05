#[doc = "Register `TDR` writer"]
pub type W = crate::svd::W<TDR_SPEC>;
#[doc = "Field `TDR` writer - desc TDR"]
pub type TDR_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 9, O, u16>;
impl W {
    #[doc = "Bits 0:8 - desc TDR"]
    #[inline(always)]
    #[must_use]
    pub fn tdr(&mut self) -> TDR_W<TDR_SPEC, 0> {
        TDR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data reg for write\n\nYou can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDR_SPEC;
impl crate::svd::RegisterSpec for TDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::svd::Writable for TDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::svd::Resettable for TDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
