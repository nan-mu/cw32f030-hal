#[doc = "Register `DR16` reader"]
pub type R = crate::svd::R<DR16_SPEC>;
#[doc = "Register `DR16` writer"]
pub type W = crate::svd::W<DR16_SPEC>;
#[doc = "Field `DR16` reader - desc DR16"]
pub type DR16_R = crate::svd::FieldReader<u16>;
#[doc = "Field `DR16` writer - desc DR16"]
pub type DR16_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - desc DR16"]
    #[inline(always)]
    pub fn dr16(&self) -> DR16_R {
        DR16_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc DR16"]
    #[inline(always)]
    #[must_use]
    pub fn dr16(&mut self) -> DR16_W<DR16_SPEC, 0> {
        DR16_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dr16::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dr16::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR16_SPEC;
impl crate::svd::RegisterSpec for DR16_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr16::R`](R) reader structure"]
impl crate::svd::Readable for DR16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr16::W`](W) writer structure"]
impl crate::svd::Writable for DR16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR16 to value 0"]
impl crate::svd::Resettable for DR16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
