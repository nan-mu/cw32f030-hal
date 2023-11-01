#[doc = "Register `BRRF` reader"]
pub type R = crate::R<BRRF_SPEC>;
#[doc = "Register `BRRF` writer"]
pub type W = crate::W<BRRF_SPEC>;
#[doc = "Field `BRRF` reader - desc BRRF"]
pub type BRRF_R = crate::FieldReader;
#[doc = "Field `BRRF` writer - desc BRRF"]
pub type BRRF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - desc BRRF"]
    #[inline(always)]
    pub fn brrf(&self) -> BRRF_R {
        BRRF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc BRRF"]
    #[inline(always)]
    #[must_use]
    pub fn brrf(&mut self) -> BRRF_W<BRRF_SPEC, 0> {
        BRRF_W::new(self)
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
#[doc = "desc BRRF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brrf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brrf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRRF_SPEC;
impl crate::RegisterSpec for BRRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brrf::R`](R) reader structure"]
impl crate::Readable for BRRF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brrf::W`](W) writer structure"]
impl crate::Writable for BRRF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRRF to value 0"]
impl crate::Resettable for BRRF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
