#[doc = "Register `IER` reader"]
pub type R = crate::svd::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::svd::W<IER_SPEC>;
#[doc = "Field `ALARMA` reader - desc ALARMA"]
pub type ALARMA_R = crate::svd::BitReader;
#[doc = "Field `ALARMA` writer - desc ALARMA"]
pub type ALARMA_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `ALARMB` reader - desc ALARMB"]
pub type ALARMB_R = crate::svd::BitReader;
#[doc = "Field `ALARMB` writer - desc ALARMB"]
pub type ALARMB_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `AWTIMER` reader - desc AWTIMER"]
pub type AWTIMER_R = crate::svd::BitReader;
#[doc = "Field `AWTIMER` writer - desc AWTIMER"]
pub type AWTIMER_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TAMP` reader - desc TAMP"]
pub type TAMP_R = crate::svd::BitReader;
#[doc = "Field `TAMP` writer - desc TAMP"]
pub type TAMP_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TAMPOV` reader - desc TAMPOV"]
pub type TAMPOV_R = crate::svd::BitReader;
#[doc = "Field `TAMPOV` writer - desc TAMPOV"]
pub type TAMPOV_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `INTERVAL` reader - desc INTERVAL"]
pub type INTERVAL_R = crate::svd::BitReader;
#[doc = "Field `INTERVAL` writer - desc INTERVAL"]
pub type INTERVAL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc ALARMA"]
    #[inline(always)]
    pub fn alarma(&self) -> ALARMA_R {
        ALARMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ALARMB"]
    #[inline(always)]
    pub fn alarmb(&self) -> ALARMB_R {
        ALARMB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc AWTIMER"]
    #[inline(always)]
    pub fn awtimer(&self) -> AWTIMER_R {
        AWTIMER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TAMP"]
    #[inline(always)]
    pub fn tamp(&self) -> TAMP_R {
        TAMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TAMPOV"]
    #[inline(always)]
    pub fn tampov(&self) -> TAMPOV_R {
        TAMPOV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc INTERVAL"]
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ALARMA"]
    #[inline(always)]
    #[must_use]
    pub fn alarma(&mut self) -> ALARMA_W<IER_SPEC, 0> {
        ALARMA_W::new(self)
    }
    #[doc = "Bit 1 - desc ALARMB"]
    #[inline(always)]
    #[must_use]
    pub fn alarmb(&mut self) -> ALARMB_W<IER_SPEC, 1> {
        ALARMB_W::new(self)
    }
    #[doc = "Bit 2 - desc AWTIMER"]
    #[inline(always)]
    #[must_use]
    pub fn awtimer(&mut self) -> AWTIMER_W<IER_SPEC, 2> {
        AWTIMER_W::new(self)
    }
    #[doc = "Bit 3 - desc TAMP"]
    #[inline(always)]
    #[must_use]
    pub fn tamp(&mut self) -> TAMP_W<IER_SPEC, 3> {
        TAMP_W::new(self)
    }
    #[doc = "Bit 4 - desc TAMPOV"]
    #[inline(always)]
    #[must_use]
    pub fn tampov(&mut self) -> TAMPOV_W<IER_SPEC, 4> {
        TAMPOV_W::new(self)
    }
    #[doc = "Bit 6 - desc INTERVAL"]
    #[inline(always)]
    #[must_use]
    pub fn interval(&mut self) -> INTERVAL_W<IER_SPEC, 6> {
        INTERVAL_W::new(self)
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
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::svd::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::svd::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::svd::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::svd::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
