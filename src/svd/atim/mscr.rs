#[doc = "Register `MSCR` reader"]
pub type R = crate::svd::R<MSCR_SPEC>;
#[doc = "Register `MSCR` writer"]
pub type W = crate::svd::W<MSCR_SPEC>;
#[doc = "Field `MMS` reader - desc MMS"]
pub type MMS_R = crate::svd::FieldReader;
#[doc = "Field `MMS` writer - desc MMS"]
pub type MMS_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CCDS` reader - desc CCDS"]
pub type CCDS_R = crate::svd::BitReader;
#[doc = "Field `CCDS` writer - desc CCDS"]
pub type CCDS_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TS` reader - desc TS"]
pub type TS_R = crate::svd::FieldReader;
#[doc = "Field `TS` writer - desc TS"]
pub type TS_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMS` reader - desc SMS"]
pub type SMS_R = crate::svd::FieldReader;
#[doc = "Field `SMS` writer - desc SMS"]
pub type SMS_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `IA1S` reader - desc IA1S"]
pub type IA1S_R = crate::svd::BitReader;
#[doc = "Field `IA1S` writer - desc IA1S"]
pub type IA1S_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `IB1S` reader - desc IB1S"]
pub type IB1S_R = crate::svd::BitReader;
#[doc = "Field `IB1S` writer - desc IB1S"]
pub type IB1S_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - desc MMS"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc CCDS"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:7 - desc TS"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - desc SMS"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - desc IA1S"]
    #[inline(always)]
    pub fn ia1s(&self) -> IA1S_R {
        IA1S_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc IB1S"]
    #[inline(always)]
    pub fn ib1s(&self) -> IB1S_R {
        IB1S_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc MMS"]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<MSCR_SPEC, 0> {
        MMS_W::new(self)
    }
    #[doc = "Bit 3 - desc CCDS"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<MSCR_SPEC, 3> {
        CCDS_W::new(self)
    }
    #[doc = "Bits 5:7 - desc TS"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<MSCR_SPEC, 5> {
        TS_W::new(self)
    }
    #[doc = "Bits 8:10 - desc SMS"]
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SMS_W<MSCR_SPEC, 8> {
        SMS_W::new(self)
    }
    #[doc = "Bit 11 - desc IA1S"]
    #[inline(always)]
    #[must_use]
    pub fn ia1s(&mut self) -> IA1S_W<MSCR_SPEC, 11> {
        IA1S_W::new(self)
    }
    #[doc = "Bit 12 - desc IB1S"]
    #[inline(always)]
    #[must_use]
    pub fn ib1s(&mut self) -> IB1S_W<MSCR_SPEC, 12> {
        IB1S_W::new(self)
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
#[doc = "desc MSCR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`mscr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`mscr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSCR_SPEC;
impl crate::svd::RegisterSpec for MSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mscr::R`](R) reader structure"]
impl crate::svd::Readable for MSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mscr::W`](W) writer structure"]
impl crate::svd::Writable for MSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSCR to value 0"]
impl crate::svd::Resettable for MSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
