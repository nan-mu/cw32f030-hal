#[doc = "Register `DR8` reader"]
pub type R = crate::svd::R<DR8_SPEC>;
#[doc = "Register `DR8` writer"]
pub type W = crate::svd::W<DR8_SPEC>;
#[doc = "Field `DR8` reader - desc DR8"]
pub type DR8_R = crate::svd::FieldReader;
#[doc = "Field `DR8` writer - desc DR8"]
pub type DR8_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - desc DR8"]
    #[inline(always)]
    pub fn dr8(&self) -> DR8_R {
        DR8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DR8"]
    #[inline(always)]
    #[must_use]
    pub fn dr8(&mut self) -> DR8_W<DR8_SPEC, 0> {
        DR8_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dr8::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dr8::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR8_SPEC;
impl crate::svd::RegisterSpec for DR8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dr8::R`](R) reader structure"]
impl crate::svd::Readable for DR8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr8::W`](W) writer structure"]
impl crate::svd::Writable for DR8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR8 to value 0"]
impl crate::svd::Resettable for DR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
