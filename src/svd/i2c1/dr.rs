#[doc = "Register `DR` reader"]
pub type R = crate::svd::R<DR_SPEC>;
#[doc = "Register `DR` writer"]
pub type W = crate::svd::W<DR_SPEC>;
#[doc = "Field `DR` reader - desc DR"]
pub type DR_R = crate::svd::FieldReader;
#[doc = "Field `DR` writer - desc DR"]
pub type DR_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - desc DR"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DR"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<DR_SPEC, 0> {
        DR_W::new(self)
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
#[doc = "Data register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_SPEC;
impl crate::svd::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::svd::Readable for DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::svd::Writable for DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::svd::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
