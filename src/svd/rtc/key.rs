#[doc = "Register `KEY` writer"]
pub type W = crate::svd::W<KEY_SPEC>;
#[doc = "Field `KEY` writer - Key = 0xCA - 0x53"]
pub type KEY_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Key = 0xCA - 0x53"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEY_SPEC, 0> {
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
#[doc = "desc KEY\n\nYou can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`key::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_SPEC;
impl crate::svd::RegisterSpec for KEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::svd::Writable for KEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY to value 0"]
impl crate::svd::Resettable for KEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
