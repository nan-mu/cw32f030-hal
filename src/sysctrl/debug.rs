#[doc = "Register `DEBUG` reader"]
pub type R = crate::R<DEBUG_SPEC>;
#[doc = "Register `DEBUG` writer"]
pub type W = crate::W<DEBUG_SPEC>;
#[doc = "Field `ATIM` reader - desc ATIM"]
pub type ATIM_R = crate::BitReader;
#[doc = "Field `ATIM` writer - desc ATIM"]
pub type ATIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GTIM1` reader - desc GTIM1"]
pub type GTIM1_R = crate::BitReader;
#[doc = "Field `GTIM1` writer - desc GTIM1"]
pub type GTIM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GTIM2` reader - desc GTIM2"]
pub type GTIM2_R = crate::BitReader;
#[doc = "Field `GTIM2` writer - desc GTIM2"]
pub type GTIM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GTIM3` reader - desc GTIM3"]
pub type GTIM3_R = crate::BitReader;
#[doc = "Field `GTIM3` writer - desc GTIM3"]
pub type GTIM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GTIM4` reader - desc GTIM4"]
pub type GTIM4_R = crate::BitReader;
#[doc = "Field `GTIM4` writer - desc GTIM4"]
pub type GTIM4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BTIM123` reader - desc BTIM123"]
pub type BTIM123_R = crate::BitReader;
#[doc = "Field `BTIM123` writer - desc BTIM123"]
pub type BTIM123_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AWT` reader - desc AWT"]
pub type AWT_R = crate::BitReader;
#[doc = "Field `AWT` writer - desc AWT"]
pub type AWT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTC` reader - desc RTC"]
pub type RTC_R = crate::BitReader;
#[doc = "Field `RTC` writer - desc RTC"]
pub type RTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IWDT` reader - desc IWDT"]
pub type IWDT_R = crate::BitReader;
#[doc = "Field `IWDT` writer - desc IWDT"]
pub type IWDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDT` reader - desc WWDT"]
pub type WWDT_R = crate::BitReader;
#[doc = "Field `WWDT` writer - desc WWDT"]
pub type WWDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&self) -> ATIM_R {
        ATIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc GTIM1"]
    #[inline(always)]
    pub fn gtim1(&self) -> GTIM1_R {
        GTIM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc GTIM2"]
    #[inline(always)]
    pub fn gtim2(&self) -> GTIM2_R {
        GTIM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc GTIM3"]
    #[inline(always)]
    pub fn gtim3(&self) -> GTIM3_R {
        GTIM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc GTIM4"]
    #[inline(always)]
    pub fn gtim4(&self) -> GTIM4_R {
        GTIM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BTIM123"]
    #[inline(always)]
    pub fn btim123(&self) -> BTIM123_R {
        BTIM123_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc AWT"]
    #[inline(always)]
    pub fn awt(&self) -> AWT_R {
        AWT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&self) -> IWDT_R {
        IWDT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc WWDT"]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ATIM"]
    #[inline(always)]
    #[must_use]
    pub fn atim(&mut self) -> ATIM_W<DEBUG_SPEC, 0> {
        ATIM_W::new(self)
    }
    #[doc = "Bit 1 - desc GTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn gtim1(&mut self) -> GTIM1_W<DEBUG_SPEC, 1> {
        GTIM1_W::new(self)
    }
    #[doc = "Bit 2 - desc GTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn gtim2(&mut self) -> GTIM2_W<DEBUG_SPEC, 2> {
        GTIM2_W::new(self)
    }
    #[doc = "Bit 3 - desc GTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn gtim3(&mut self) -> GTIM3_W<DEBUG_SPEC, 3> {
        GTIM3_W::new(self)
    }
    #[doc = "Bit 4 - desc GTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn gtim4(&mut self) -> GTIM4_W<DEBUG_SPEC, 4> {
        GTIM4_W::new(self)
    }
    #[doc = "Bit 5 - desc BTIM123"]
    #[inline(always)]
    #[must_use]
    pub fn btim123(&mut self) -> BTIM123_W<DEBUG_SPEC, 5> {
        BTIM123_W::new(self)
    }
    #[doc = "Bit 6 - desc AWT"]
    #[inline(always)]
    #[must_use]
    pub fn awt(&mut self) -> AWT_W<DEBUG_SPEC, 6> {
        AWT_W::new(self)
    }
    #[doc = "Bit 8 - desc RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<DEBUG_SPEC, 8> {
        RTC_W::new(self)
    }
    #[doc = "Bit 9 - desc IWDT"]
    #[inline(always)]
    #[must_use]
    pub fn iwdt(&mut self) -> IWDT_W<DEBUG_SPEC, 9> {
        IWDT_W::new(self)
    }
    #[doc = "Bit 10 - desc WWDT"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt(&mut self) -> WWDT_W<DEBUG_SPEC, 10> {
        WWDT_W::new(self)
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
#[doc = "Debug Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_SPEC;
impl crate::RegisterSpec for DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug::R`](R) reader structure"]
impl crate::Readable for DEBUG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug::W`](W) writer structure"]
impl crate::Writable for DEBUG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG to value 0"]
impl crate::Resettable for DEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
