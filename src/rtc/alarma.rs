#[doc = "Register `ALARMA` reader"]
pub type R = crate::R<ALARMA_SPEC>;
#[doc = "Register `ALARMA` writer"]
pub type W = crate::W<ALARMA_SPEC>;
#[doc = "Field `SECOND` reader - desc SECOND"]
pub type SECOND_R = crate::FieldReader;
#[doc = "Field `SECOND` writer - desc SECOND"]
pub type SECOND_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SECONDMASK` reader - desc SECONDMASK"]
pub type SECONDMASK_R = crate::BitReader;
#[doc = "Field `SECONDMASK` writer - desc SECONDMASK"]
pub type SECONDMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MINUTE` reader - desc MINUTE"]
pub type MINUTE_R = crate::FieldReader;
#[doc = "Field `MINUTE` writer - desc MINUTE"]
pub type MINUTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `MINUTEMASK` reader - desc MINUTEMASK"]
pub type MINUTEMASK_R = crate::BitReader;
#[doc = "Field `MINUTEMASK` writer - desc MINUTEMASK"]
pub type MINUTEMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HOUR` reader - desc HOUR"]
pub type HOUR_R = crate::FieldReader;
#[doc = "Field `HOUR` writer - desc HOUR"]
pub type HOUR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `HOURMASK` reader - desc HOURMASK"]
pub type HOURMASK_R = crate::BitReader;
#[doc = "Field `HOURMASK` writer - desc HOURMASK"]
pub type HOURMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WEEK` reader - desc WEEK"]
pub type WEEK_R = crate::FieldReader;
#[doc = "Field `WEEK` writer - desc WEEK"]
pub type WEEK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - desc SECOND"]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - desc SECONDMASK"]
    #[inline(always)]
    pub fn secondmask(&self) -> SECONDMASK_R {
        SECONDMASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - desc MINUTE"]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - desc MINUTEMASK"]
    #[inline(always)]
    pub fn minutemask(&self) -> MINUTEMASK_R {
        MINUTEMASK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - desc HOUR"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - desc HOURMASK"]
    #[inline(always)]
    pub fn hourmask(&self) -> HOURMASK_R {
        HOURMASK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - desc WEEK"]
    #[inline(always)]
    pub fn week(&self) -> WEEK_R {
        WEEK_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc SECOND"]
    #[inline(always)]
    #[must_use]
    pub fn second(&mut self) -> SECOND_W<ALARMA_SPEC, 0> {
        SECOND_W::new(self)
    }
    #[doc = "Bit 7 - desc SECONDMASK"]
    #[inline(always)]
    #[must_use]
    pub fn secondmask(&mut self) -> SECONDMASK_W<ALARMA_SPEC, 7> {
        SECONDMASK_W::new(self)
    }
    #[doc = "Bits 8:14 - desc MINUTE"]
    #[inline(always)]
    #[must_use]
    pub fn minute(&mut self) -> MINUTE_W<ALARMA_SPEC, 8> {
        MINUTE_W::new(self)
    }
    #[doc = "Bit 15 - desc MINUTEMASK"]
    #[inline(always)]
    #[must_use]
    pub fn minutemask(&mut self) -> MINUTEMASK_W<ALARMA_SPEC, 15> {
        MINUTEMASK_W::new(self)
    }
    #[doc = "Bits 16:21 - desc HOUR"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<ALARMA_SPEC, 16> {
        HOUR_W::new(self)
    }
    #[doc = "Bit 23 - desc HOURMASK"]
    #[inline(always)]
    #[must_use]
    pub fn hourmask(&mut self) -> HOURMASK_W<ALARMA_SPEC, 23> {
        HOURMASK_W::new(self)
    }
    #[doc = "Bits 24:30 - desc WEEK"]
    #[inline(always)]
    #[must_use]
    pub fn week(&mut self) -> WEEK_W<ALARMA_SPEC, 24> {
        WEEK_W::new(self)
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
#[doc = "Alarm - A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARMA_SPEC;
impl crate::RegisterSpec for ALARMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarma::R`](R) reader structure"]
impl crate::Readable for ALARMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarma::W`](W) writer structure"]
impl crate::Writable for ALARMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALARMA to value 0"]
impl crate::Resettable for ALARMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
