#[doc = "Register `ACR` reader"]
pub type R = crate::svd::R<ACR_SPEC>;
#[doc = "Register `ACR` writer"]
pub type W = crate::svd::W<ACR_SPEC>;
#[doc = "Field `ETRFLT` reader - desc ETRFLT"]
pub type ETRFLT_R = crate::svd::FieldReader;
#[doc = "Field `ETRFLT` writer - desc ETRFLT"]
pub type ETRFLT_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 4:6 - desc ETRFLT"]
    #[inline(always)]
    pub fn etrflt(&self) -> ETRFLT_R {
        ETRFLT_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - desc ETRFLT"]
    #[inline(always)]
    #[must_use]
    pub fn etrflt(&mut self) -> ETRFLT_W<ACR_SPEC, 4> {
        ETRFLT_W::new(self)
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
#[doc = "Advanced Control register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACR_SPEC;
impl crate::svd::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::svd::Readable for ACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::svd::Writable for ACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::svd::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
