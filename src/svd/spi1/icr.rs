#[doc = "Register `ICR` reader"]
pub type R = crate::svd::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::svd::W<ICR_SPEC>;
#[doc = "Field `FLUSH` reader - desc FLUSH"]
pub type FLUSH_R = crate::svd::BitReader;
#[doc = "Field `FLUSH` writer - desc FLUSH"]
pub type FLUSH_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RXNE_R = crate::svd::BitReader;
#[doc = "Field `RXNE` writer - desc RXNE"]
pub type RXNE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SSF` reader - desc SSF"]
pub type SSF_R = crate::svd::BitReader;
#[doc = "Field `SSF` writer - desc SSF"]
pub type SSF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SSR` reader - desc SSR"]
pub type SSR_R = crate::svd::BitReader;
#[doc = "Field `SSR` writer - desc SSR"]
pub type SSR_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `UD` reader - desc UD"]
pub type UD_R = crate::svd::BitReader;
#[doc = "Field `UD` writer - desc UD"]
pub type UD_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `OV` reader - desc OV"]
pub type OV_R = crate::svd::BitReader;
#[doc = "Field `OV` writer - desc OV"]
pub type OV_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SSERR` reader - desc SSERR"]
pub type SSERR_R = crate::svd::BitReader;
#[doc = "Field `SSERR` writer - desc SSERR"]
pub type SSERR_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `MODF` reader - desc MODF"]
pub type MODF_R = crate::svd::BitReader;
#[doc = "Field `MODF` writer - desc MODF"]
pub type MODF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc FLUSH"]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SSF"]
    #[inline(always)]
    pub fn ssf(&self) -> SSF_R {
        SSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SSR"]
    #[inline(always)]
    pub fn ssr(&self) -> SSR_R {
        SSR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc UD"]
    #[inline(always)]
    pub fn ud(&self) -> UD_R {
        UD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SSERR"]
    #[inline(always)]
    pub fn sserr(&self) -> SSERR_R {
        SSERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc MODF"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc FLUSH"]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<ICR_SPEC, 0> {
        FLUSH_W::new(self)
    }
    #[doc = "Bit 1 - desc RXNE"]
    #[inline(always)]
    #[must_use]
    pub fn rxne(&mut self) -> RXNE_W<ICR_SPEC, 1> {
        RXNE_W::new(self)
    }
    #[doc = "Bit 2 - desc SSF"]
    #[inline(always)]
    #[must_use]
    pub fn ssf(&mut self) -> SSF_W<ICR_SPEC, 2> {
        SSF_W::new(self)
    }
    #[doc = "Bit 3 - desc SSR"]
    #[inline(always)]
    #[must_use]
    pub fn ssr(&mut self) -> SSR_W<ICR_SPEC, 3> {
        SSR_W::new(self)
    }
    #[doc = "Bit 4 - desc UD"]
    #[inline(always)]
    #[must_use]
    pub fn ud(&mut self) -> UD_W<ICR_SPEC, 4> {
        UD_W::new(self)
    }
    #[doc = "Bit 5 - desc OV"]
    #[inline(always)]
    #[must_use]
    pub fn ov(&mut self) -> OV_W<ICR_SPEC, 5> {
        OV_W::new(self)
    }
    #[doc = "Bit 6 - desc SSERR"]
    #[inline(always)]
    #[must_use]
    pub fn sserr(&mut self) -> SSERR_W<ICR_SPEC, 6> {
        SSERR_W::new(self)
    }
    #[doc = "Bit 7 - desc MODF"]
    #[inline(always)]
    #[must_use]
    pub fn modf(&mut self) -> MODF_W<ICR_SPEC, 7> {
        MODF_W::new(self)
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
#[doc = "Interrupt flag clear register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::svd::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::svd::Readable for ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::svd::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::svd::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
