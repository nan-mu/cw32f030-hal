#[doc = "Register `CMMR` reader"]
pub type R = crate::svd::R<CMMR_SPEC>;
#[doc = "Register `CMMR` writer"]
pub type W = crate::svd::W<CMMR_SPEC>;
#[doc = "Field `CC1M` reader - desc CC1M"]
pub type CC1M_R = crate::svd::FieldReader;
#[doc = "Field `CC1M` writer - desc CC1M"]
pub type CC1M_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CC2M` reader - desc CC2M"]
pub type CC2M_R = crate::svd::FieldReader;
#[doc = "Field `CC2M` writer - desc CC2M"]
pub type CC2M_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CC3M` reader - desc CC3M"]
pub type CC3M_R = crate::svd::FieldReader;
#[doc = "Field `CC3M` writer - desc CC3M"]
pub type CC3M_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CC4M` reader - desc CC4M"]
pub type CC4M_R = crate::svd::FieldReader;
#[doc = "Field `CC4M` writer - desc CC4M"]
pub type CC4M_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - desc CC1M"]
    #[inline(always)]
    pub fn cc1m(&self) -> CC1M_R {
        CC1M_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc CC2M"]
    #[inline(always)]
    pub fn cc2m(&self) -> CC2M_R {
        CC2M_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc CC3M"]
    #[inline(always)]
    pub fn cc3m(&self) -> CC3M_R {
        CC3M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc CC4M"]
    #[inline(always)]
    pub fn cc4m(&self) -> CC4M_R {
        CC4M_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CC1M"]
    #[inline(always)]
    #[must_use]
    pub fn cc1m(&mut self) -> CC1M_W<CMMR_SPEC, 0> {
        CC1M_W::new(self)
    }
    #[doc = "Bits 4:7 - desc CC2M"]
    #[inline(always)]
    #[must_use]
    pub fn cc2m(&mut self) -> CC2M_W<CMMR_SPEC, 4> {
        CC2M_W::new(self)
    }
    #[doc = "Bits 8:11 - desc CC3M"]
    #[inline(always)]
    #[must_use]
    pub fn cc3m(&mut self) -> CC3M_W<CMMR_SPEC, 8> {
        CC3M_W::new(self)
    }
    #[doc = "Bits 12:15 - desc CC4M"]
    #[inline(always)]
    #[must_use]
    pub fn cc4m(&mut self) -> CC4M_W<CMMR_SPEC, 12> {
        CC4M_W::new(self)
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
#[doc = "Capture compare control Register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cmmr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cmmr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMMR_SPEC;
impl crate::svd::RegisterSpec for CMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmmr::R`](R) reader structure"]
impl crate::svd::Readable for CMMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmmr::W`](W) writer structure"]
impl crate::svd::Writable for CMMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMMR to value 0"]
impl crate::svd::Resettable for CMMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
