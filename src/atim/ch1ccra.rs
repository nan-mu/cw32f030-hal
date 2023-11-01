#[doc = "Register `CH1CCRA` reader"]
pub type R = crate::R<CH1CCRA_SPEC>;
#[doc = "Register `CH1CCRA` writer"]
pub type W = crate::W<CH1CCRA_SPEC>;
#[doc = "Field `CCR1A` reader - desc CCR1A"]
pub type CCR1A_R = crate::FieldReader<u16>;
#[doc = "Field `CCR1A` writer - desc CCR1A"]
pub type CCR1A_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR1A"]
    #[inline(always)]
    pub fn ccr1a(&self) -> CCR1A_R {
        CCR1A_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR1A"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1a(&mut self) -> CCR1A_W<CH1CCRA_SPEC, 0> {
        CCR1A_W::new(self)
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
#[doc = "desc CH1CCRA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ccra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ccra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1CCRA_SPEC;
impl crate::RegisterSpec for CH1CCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1ccra::R`](R) reader structure"]
impl crate::Readable for CH1CCRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch1ccra::W`](W) writer structure"]
impl crate::Writable for CH1CCRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1CCRA to value 0"]
impl crate::Resettable for CH1CCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
