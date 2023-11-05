#[doc = "Register `CH3CCRA` reader"]
pub type R = crate::svd::R<CH3CCRA_SPEC>;
#[doc = "Register `CH3CCRA` writer"]
pub type W = crate::svd::W<CH3CCRA_SPEC>;
#[doc = "Field `CCR3A` reader - desc CCR3A"]
pub type CCR3A_R = crate::svd::FieldReader<u16>;
#[doc = "Field `CCR3A` writer - desc CCR3A"]
pub type CCR3A_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR3A"]
    #[inline(always)]
    pub fn ccr3a(&self) -> CCR3A_R {
        CCR3A_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR3A"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3a(&mut self) -> CCR3A_W<CH3CCRA_SPEC, 0> {
        CCR3A_W::new(self)
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
#[doc = "desc CH3CCRA\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch3ccra::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch3ccra::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3CCRA_SPEC;
impl crate::svd::RegisterSpec for CH3CCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3ccra::R`](R) reader structure"]
impl crate::svd::Readable for CH3CCRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch3ccra::W`](W) writer structure"]
impl crate::svd::Writable for CH3CCRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3CCRA to value 0"]
impl crate::svd::Resettable for CH3CCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
