#[doc = "Register `CH3CCRB` reader"]
pub type R = crate::svd::R<CH3CCRB_SPEC>;
#[doc = "Register `CH3CCRB` writer"]
pub type W = crate::svd::W<CH3CCRB_SPEC>;
#[doc = "Field `CCR3B` reader - desc CCR3B"]
pub type CCR3B_R = crate::svd::FieldReader<u16>;
#[doc = "Field `CCR3B` writer - desc CCR3B"]
pub type CCR3B_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR3B"]
    #[inline(always)]
    pub fn ccr3b(&self) -> CCR3B_R {
        CCR3B_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR3B"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3b(&mut self) -> CCR3B_W<CH3CCRB_SPEC, 0> {
        CCR3B_W::new(self)
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
#[doc = "desc CH3CCRB\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch3ccrb::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch3ccrb::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3CCRB_SPEC;
impl crate::svd::RegisterSpec for CH3CCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3ccrb::R`](R) reader structure"]
impl crate::svd::Readable for CH3CCRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch3ccrb::W`](W) writer structure"]
impl crate::svd::Writable for CH3CCRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3CCRB to value 0"]
impl crate::svd::Resettable for CH3CCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
