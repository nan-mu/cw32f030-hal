#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `CRF` reader - desc CRF"]
pub type CRF_R = crate::BitReader;
#[doc = "Field `ARRF` reader - desc ARRF"]
pub type ARRF_R = crate::BitReader;
#[doc = "Field `WINRF` reader - desc WINRF"]
pub type WINRF_R = crate::BitReader;
#[doc = "Field `OV` reader - desc OV"]
pub type OV_R = crate::BitReader;
#[doc = "Field `OV` writer - desc OV"]
pub type OV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUN` reader - desc RUN"]
pub type RUN_R = crate::BitReader;
#[doc = "Field `RELOAD` reader - desc RELOAD"]
pub type RELOAD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc CRF"]
    #[inline(always)]
    pub fn crf(&self) -> CRF_R {
        CRF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ARRF"]
    #[inline(always)]
    pub fn arrf(&self) -> ARRF_R {
        ARRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc WINRF"]
    #[inline(always)]
    pub fn winrf(&self) -> WINRF_R {
        WINRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RUN"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RELOAD"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - desc OV"]
    #[inline(always)]
    #[must_use]
    pub fn ov(&mut self) -> OV_W<SR_SPEC, 3> {
        OV_W::new(self)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
