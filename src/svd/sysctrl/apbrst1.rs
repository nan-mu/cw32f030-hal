#[doc = "Register `APBRST1` reader"]
pub type R = crate::svd::R<APBRST1_SPEC>;
#[doc = "Register `APBRST1` writer"]
pub type W = crate::svd::W<APBRST1_SPEC>;
#[doc = "Field `GTIM1` reader - desc GTIM1"]
pub type GTIM1_R = crate::svd::BitReader;
#[doc = "Field `GTIM1` writer - desc GTIM1"]
pub type GTIM1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `GTIM2` reader - desc GTIM2"]
pub type GTIM2_R = crate::svd::BitReader;
#[doc = "Field `GTIM2` writer - desc GTIM2"]
pub type GTIM2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `RTC` reader - desc RTC"]
pub type RTC_R = crate::svd::BitReader;
#[doc = "Field `RTC` writer - desc RTC"]
pub type RTC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `WWDT` reader - desc WWDT"]
pub type WWDT_R = crate::svd::BitReader;
#[doc = "Field `WWDT` writer - desc WWDT"]
pub type WWDT_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `IWDT` reader - desc IWDT"]
pub type IWDT_R = crate::svd::BitReader;
#[doc = "Field `IWDT` writer - desc IWDT"]
pub type IWDT_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SPI2` reader - desc SPI2"]
pub type SPI2_R = crate::svd::BitReader;
#[doc = "Field `SPI2` writer - desc SPI2"]
pub type SPI2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `UART2` reader - desc UART2"]
pub type UART2_R = crate::svd::BitReader;
#[doc = "Field `UART2` writer - desc UART2"]
pub type UART2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `UART3` reader - desc UART3"]
pub type UART3_R = crate::svd::BitReader;
#[doc = "Field `UART3` writer - desc UART3"]
pub type UART3_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1` reader - desc I2C1"]
pub type I2C1_R = crate::svd::BitReader;
#[doc = "Field `I2C1` writer - desc I2C1"]
pub type I2C1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2` reader - desc I2C2"]
pub type I2C2_R = crate::svd::BitReader;
#[doc = "Field `I2C2` writer - desc I2C2"]
pub type I2C2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
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
    #[doc = "Bit 3 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc WWDT"]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&self) -> IWDT_R {
        IWDT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc UART3"]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc I2C2"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc GTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn gtim1(&mut self) -> GTIM1_W<APBRST1_SPEC, 1> {
        GTIM1_W::new(self)
    }
    #[doc = "Bit 2 - desc GTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn gtim2(&mut self) -> GTIM2_W<APBRST1_SPEC, 2> {
        GTIM2_W::new(self)
    }
    #[doc = "Bit 3 - desc RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<APBRST1_SPEC, 3> {
        RTC_W::new(self)
    }
    #[doc = "Bit 4 - desc WWDT"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt(&mut self) -> WWDT_W<APBRST1_SPEC, 4> {
        WWDT_W::new(self)
    }
    #[doc = "Bit 5 - desc IWDT"]
    #[inline(always)]
    #[must_use]
    pub fn iwdt(&mut self) -> IWDT_W<APBRST1_SPEC, 5> {
        IWDT_W::new(self)
    }
    #[doc = "Bit 6 - desc SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<APBRST1_SPEC, 6> {
        SPI2_W::new(self)
    }
    #[doc = "Bit 7 - desc UART2"]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> UART2_W<APBRST1_SPEC, 7> {
        UART2_W::new(self)
    }
    #[doc = "Bit 8 - desc UART3"]
    #[inline(always)]
    #[must_use]
    pub fn uart3(&mut self) -> UART3_W<APBRST1_SPEC, 8> {
        UART3_W::new(self)
    }
    #[doc = "Bit 11 - desc I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<APBRST1_SPEC, 11> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 12 - desc I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<APBRST1_SPEC, 12> {
        I2C2_W::new(self)
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
#[doc = "APB Reset Control Reg1\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`apbrst1::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`apbrst1::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBRST1_SPEC;
impl crate::svd::RegisterSpec for APBRST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrst1::R`](R) reader structure"]
impl crate::svd::Readable for APBRST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbrst1::W`](W) writer structure"]
impl crate::svd::Writable for APBRST1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBRST1 to value 0"]
impl crate::svd::Resettable for APBRST1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
