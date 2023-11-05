#[doc = "Register `BRRI` reader"]
pub type R = crate::svd::R<BRRI_SPEC>;
#[doc = "Register `BRRI` writer"]
pub type W = crate::svd::W<BRRI_SPEC>;
#[doc = "Field `BRRI` reader - desc BRRI"]
pub type BRRI_R = crate::svd::FieldReader<u16>;
#[doc = "Field `BRRI` writer - desc BRRI"]
pub type BRRI_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - desc BRRI"]
    #[inline(always)]
    pub fn brri(&self) -> BRRI_R {
        BRRI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc BRRI"]
    #[inline(always)]
    #[must_use]
    pub fn brri(&mut self) -> BRRI_W<BRRI_SPEC, 0> {
        BRRI_W::new(self)
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
#[doc = "desc BRRI\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`brri::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`brri::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRRI_SPEC;
impl crate::svd::RegisterSpec for BRRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brri::R`](R) reader structure"]
impl crate::svd::Readable for BRRI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brri::W`](W) writer structure"]
impl crate::svd::Writable for BRRI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRRI to value 0"]
impl crate::svd::Resettable for BRRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
