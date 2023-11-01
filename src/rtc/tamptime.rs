#[doc = "Register `TAMPTIME` reader"]
pub type R = crate::R<TAMPTIME_SPEC>;
#[doc = "Register `TAMPTIME` writer"]
pub type W = crate::W<TAMPTIME_SPEC>;
#[doc = "Field `SECOND` reader - desc SECOND"]
pub type SECOND_R = crate::FieldReader;
#[doc = "Field `SECOND` writer - desc SECOND"]
pub type SECOND_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `MINUTE` reader - desc MINUTE"]
pub type MINUTE_R = crate::FieldReader;
#[doc = "Field `MINUTE` writer - desc MINUTE"]
pub type MINUTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `HOUR` reader - desc HOUR"]
pub type HOUR_R = crate::FieldReader;
#[doc = "Field `HOUR` writer - desc HOUR"]
pub type HOUR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:6 - desc SECOND"]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - desc MINUTE"]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - desc HOUR"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc SECOND"]
    #[inline(always)]
    #[must_use]
    pub fn second(&mut self) -> SECOND_W<TAMPTIME_SPEC, 0> {
        SECOND_W::new(self)
    }
    #[doc = "Bits 8:14 - desc MINUTE"]
    #[inline(always)]
    #[must_use]
    pub fn minute(&mut self) -> MINUTE_W<TAMPTIME_SPEC, 8> {
        MINUTE_W::new(self)
    }
    #[doc = "Bits 16:21 - desc HOUR"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<TAMPTIME_SPEC, 16> {
        HOUR_W::new(self)
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
#[doc = "desc TAMPTIME\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamptime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamptime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMPTIME_SPEC;
impl crate::RegisterSpec for TAMPTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamptime::R`](R) reader structure"]
impl crate::Readable for TAMPTIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tamptime::W`](W) writer structure"]
impl crate::Writable for TAMPTIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMPTIME to value 0"]
impl crate::Resettable for TAMPTIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
