#[doc = "Register `RCR` reader"]
pub type R = crate::svd::R<RCR_SPEC>;
#[doc = "Register `RCR` writer"]
pub type W = crate::svd::W<RCR_SPEC>;
#[doc = "Field `RCR` reader - desc RCR"]
pub type RCR_R = crate::svd::FieldReader;
#[doc = "Field `RCR` writer - desc RCR"]
pub type RCR_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `OV` reader - desc OV"]
pub type OV_R = crate::svd::BitReader;
#[doc = "Field `OV` writer - desc OV"]
pub type OV_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `UD` reader - desc UD"]
pub type UD_R = crate::svd::BitReader;
#[doc = "Field `UD` writer - desc UD"]
pub type UD_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - desc RCR"]
    #[inline(always)]
    pub fn rcr(&self) -> RCR_R {
        RCR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc UD"]
    #[inline(always)]
    pub fn ud(&self) -> UD_R {
        UD_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc RCR"]
    #[inline(always)]
    #[must_use]
    pub fn rcr(&mut self) -> RCR_W<RCR_SPEC, 0> {
        RCR_W::new(self)
    }
    #[doc = "Bit 8 - desc OV"]
    #[inline(always)]
    #[must_use]
    pub fn ov(&mut self) -> OV_W<RCR_SPEC, 8> {
        OV_W::new(self)
    }
    #[doc = "Bit 9 - desc UD"]
    #[inline(always)]
    #[must_use]
    pub fn ud(&mut self) -> UD_W<RCR_SPEC, 9> {
        UD_W::new(self)
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
#[doc = "desc RCR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`rcr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCR_SPEC;
impl crate::svd::RegisterSpec for RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::svd::Readable for RCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::svd::Writable for RCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::svd::Resettable for RCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
