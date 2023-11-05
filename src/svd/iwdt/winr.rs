#[doc = "Register `WINR` reader"]
pub type R = crate::svd::R<WINR_SPEC>;
#[doc = "Register `WINR` writer"]
pub type W = crate::svd::W<WINR_SPEC>;
#[doc = "Field `WINR` reader - desc WINR"]
pub type WINR_R = crate::svd::FieldReader<u16>;
#[doc = "Field `WINR` writer - desc WINR"]
pub type WINR_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - desc WINR"]
    #[inline(always)]
    pub fn winr(&self) -> WINR_R {
        WINR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc WINR"]
    #[inline(always)]
    #[must_use]
    pub fn winr(&mut self) -> WINR_W<WINR_SPEC, 0> {
        WINR_W::new(self)
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
#[doc = "Window register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`winr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`winr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WINR_SPEC;
impl crate::svd::RegisterSpec for WINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winr::R`](R) reader structure"]
impl crate::svd::Readable for WINR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`winr::W`](W) writer structure"]
impl crate::svd::Writable for WINR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WINR to value 0"]
impl crate::svd::Resettable for WINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
