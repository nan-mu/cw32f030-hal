#[doc = "Register `CH4CCR` reader"]
pub type R = crate::svd::R<CH4CCR_SPEC>;
#[doc = "Register `CH4CCR` writer"]
pub type W = crate::svd::W<CH4CCR_SPEC>;
#[doc = "Field `CCR4` reader - desc CCR4"]
pub type CCR4_R = crate::svd::FieldReader<u16>;
#[doc = "Field `CCR4` writer - desc CCR4"]
pub type CCR4_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR4"]
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR4"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4(&mut self) -> CCR4_W<CH4CCR_SPEC, 0> {
        CCR4_W::new(self)
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
#[doc = "desc CH4CCR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch4ccr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch4ccr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4CCR_SPEC;
impl crate::svd::RegisterSpec for CH4CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4ccr::R`](R) reader structure"]
impl crate::svd::Readable for CH4CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch4ccr::W`](W) writer structure"]
impl crate::svd::Writable for CH4CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH4CCR to value 0"]
impl crate::svd::Resettable for CH4CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
