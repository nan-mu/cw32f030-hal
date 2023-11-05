#[doc = "Register `CH1CCRB` reader"]
pub type R = crate::svd::R<CH1CCRB_SPEC>;
#[doc = "Register `CH1CCRB` writer"]
pub type W = crate::svd::W<CH1CCRB_SPEC>;
#[doc = "Field `CCR1B` reader - desc CCR1B"]
pub type CCR1B_R = crate::svd::FieldReader<u16>;
#[doc = "Field `CCR1B` writer - desc CCR1B"]
pub type CCR1B_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR1B"]
    #[inline(always)]
    pub fn ccr1b(&self) -> CCR1B_R {
        CCR1B_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR1B"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1b(&mut self) -> CCR1B_W<CH1CCRB_SPEC, 0> {
        CCR1B_W::new(self)
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
#[doc = "desc CH1CCRB\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch1ccrb::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch1ccrb::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1CCRB_SPEC;
impl crate::svd::RegisterSpec for CH1CCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1ccrb::R`](R) reader structure"]
impl crate::svd::Readable for CH1CCRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch1ccrb::W`](W) writer structure"]
impl crate::svd::Writable for CH1CCRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1CCRB to value 0"]
impl crate::svd::Resettable for CH1CCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
