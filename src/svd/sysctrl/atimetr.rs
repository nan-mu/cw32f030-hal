#[doc = "Register `ATIMETR` reader"]
pub type R = crate::svd::R<ATIMETR_SPEC>;
#[doc = "Register `ATIMETR` writer"]
pub type W = crate::svd::W<ATIMETR_SPEC>;
#[doc = "Field `ATIMETR` reader - desc ATIMETR"]
pub type ATIMETR_R = crate::svd::FieldReader;
#[doc = "Field `ATIMETR` writer - desc ATIMETR"]
pub type ATIMETR_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - desc ATIMETR"]
    #[inline(always)]
    pub fn atimetr(&self) -> ATIMETR_R {
        ATIMETR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc ATIMETR"]
    #[inline(always)]
    #[must_use]
    pub fn atimetr(&mut self) -> ATIMETR_W<ATIMETR_SPEC, 0> {
        ATIMETR_W::new(self)
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
#[doc = "ATIM ETR Control Reg\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`atimetr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`atimetr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATIMETR_SPEC;
impl crate::svd::RegisterSpec for ATIMETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atimetr::R`](R) reader structure"]
impl crate::svd::Readable for ATIMETR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`atimetr::W`](W) writer structure"]
impl crate::svd::Writable for ATIMETR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATIMETR to value 0"]
impl crate::svd::Resettable for ATIMETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
