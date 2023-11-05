#[doc = "Register `CH4CR` reader"]
pub type R = crate::svd::R<CH4CR_SPEC>;
#[doc = "Register `CH4CR` writer"]
pub type W = crate::svd::W<CH4CR_SPEC>;
#[doc = "Field `BUFE` reader - desc BUFE"]
pub type BUFE_R = crate::svd::BitReader;
#[doc = "Field `BUFE` writer - desc BUFE"]
pub type BUFE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CIE` reader - desc CIE"]
pub type CIE_R = crate::svd::BitReader;
#[doc = "Field `CIE` writer - desc CIE"]
pub type CIE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CDE` reader - desc CDE"]
pub type CDE_R = crate::svd::BitReader;
#[doc = "Field `CDE` writer - desc CDE"]
pub type CDE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CIS` reader - desc CIS"]
pub type CIS_R = crate::svd::FieldReader;
#[doc = "Field `CIS` writer - desc CIS"]
pub type CIS_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `C4EN` reader - desc C4EN"]
pub type C4EN_R = crate::svd::BitReader;
#[doc = "Field `C4EN` writer - desc C4EN"]
pub type C4EN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc BUFE"]
    #[inline(always)]
    pub fn bufe(&self) -> BUFE_R {
        BUFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CIE"]
    #[inline(always)]
    pub fn cie(&self) -> CIE_R {
        CIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CDE"]
    #[inline(always)]
    pub fn cde(&self) -> CDE_R {
        CDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - desc CIS"]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - desc C4EN"]
    #[inline(always)]
    pub fn c4en(&self) -> C4EN_R {
        C4EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BUFE"]
    #[inline(always)]
    #[must_use]
    pub fn bufe(&mut self) -> BUFE_W<CH4CR_SPEC, 0> {
        BUFE_W::new(self)
    }
    #[doc = "Bit 1 - desc CIE"]
    #[inline(always)]
    #[must_use]
    pub fn cie(&mut self) -> CIE_W<CH4CR_SPEC, 1> {
        CIE_W::new(self)
    }
    #[doc = "Bit 2 - desc CDE"]
    #[inline(always)]
    #[must_use]
    pub fn cde(&mut self) -> CDE_W<CH4CR_SPEC, 2> {
        CDE_W::new(self)
    }
    #[doc = "Bits 3:4 - desc CIS"]
    #[inline(always)]
    #[must_use]
    pub fn cis(&mut self) -> CIS_W<CH4CR_SPEC, 3> {
        CIS_W::new(self)
    }
    #[doc = "Bit 5 - desc C4EN"]
    #[inline(always)]
    #[must_use]
    pub fn c4en(&mut self) -> C4EN_W<CH4CR_SPEC, 5> {
        C4EN_W::new(self)
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
#[doc = "desc CH4CR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ch4cr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ch4cr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4CR_SPEC;
impl crate::svd::RegisterSpec for CH4CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4cr::R`](R) reader structure"]
impl crate::svd::Readable for CH4CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch4cr::W`](W) writer structure"]
impl crate::svd::Writable for CH4CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH4CR to value 0"]
impl crate::svd::Resettable for CH4CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
