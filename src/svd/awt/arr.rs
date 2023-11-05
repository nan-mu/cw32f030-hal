#[doc = "Register `ARR` reader"]
pub type R = crate::svd::R<ARR_SPEC>;
#[doc = "Register `ARR` writer"]
pub type W = crate::svd::W<ARR_SPEC>;
#[doc = "Field `ARR` reader - desc ARR"]
pub type ARR_R = crate::svd::FieldReader<u16>;
#[doc = "Field `ARR` writer - desc ARR"]
pub type ARR_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - desc ARR"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc ARR"]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<ARR_SPEC, 0> {
        ARR_W::new(self)
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
#[doc = "Auto reload register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`arr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARR_SPEC;
impl crate::svd::RegisterSpec for ARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arr::R`](R) reader structure"]
impl crate::svd::Readable for ARR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arr::W`](W) writer structure"]
impl crate::svd::Writable for ARR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARR to value 0"]
impl crate::svd::Resettable for ARR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
