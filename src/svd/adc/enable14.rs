#[doc = "Register `ENABLE14` writer"]
pub type W = crate::svd::W<ENABLE14_SPEC>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KEY_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - desc KEY"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<ENABLE14_SPEC, 0> {
        KEY_W::new(self)
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
#[doc = "desc ENABLE14\n\nYou can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`enable14::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE14_SPEC;
impl crate::svd::RegisterSpec for ENABLE14_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable14::W`](W) writer structure"]
impl crate::svd::Writable for ENABLE14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLE14 to value 0"]
impl crate::svd::Resettable for ENABLE14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
