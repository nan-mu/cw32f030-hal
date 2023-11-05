#[doc = "Register `MASK` reader"]
pub type R = crate::svd::R<MASK_SPEC>;
#[doc = "Register `MASK` writer"]
pub type W = crate::svd::W<MASK_SPEC>;
#[doc = "Field `MASK` reader - desc MASK"]
pub type MASK_R = crate::svd::FieldReader;
#[doc = "Field `MASK` writer - desc MASK"]
pub type MASK_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - desc MASK"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc MASK"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<MASK_SPEC, 0> {
        MASK_W::new(self)
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
#[doc = "Slave addr mask\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`mask::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASK_SPEC;
impl crate::svd::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask::R`](R) reader structure"]
impl crate::svd::Readable for MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mask::W`](W) writer structure"]
impl crate::svd::Writable for MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::svd::Resettable for MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
