#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `ALARMA` reader - desc ALARMA"]
pub type ALARMA_R = crate::BitReader;
#[doc = "Field `ALARMB` reader - desc ALARMB"]
pub type ALARMB_R = crate::BitReader;
#[doc = "Field `AWTIMER` reader - desc AWTIMER"]
pub type AWTIMER_R = crate::BitReader;
#[doc = "Field `TAMP` reader - desc TAMP"]
pub type TAMP_R = crate::BitReader;
#[doc = "Field `TAMPOV` reader - desc TAMPOV"]
pub type TAMPOV_R = crate::BitReader;
#[doc = "Field `INTERVAL` reader - desc INTERVAL"]
pub type INTERVAL_R = crate::BitReader;
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
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
