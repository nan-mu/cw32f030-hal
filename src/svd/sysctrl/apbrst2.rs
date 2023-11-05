#[doc = "Register `APBRST2` reader"]
pub type R = crate::svd::R<APBRST2_SPEC>;
#[doc = "Register `APBRST2` writer"]
pub type W = crate::svd::W<APBRST2_SPEC>;
#[doc = "Field `ADC` reader - desc ADC"]
pub type ADC_R = crate::svd::BitReader;
#[doc = "Field `ADC` writer - desc ADC"]
pub type ADC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `VC` reader - desc VC"]
pub type VC_R = crate::svd::BitReader;
#[doc = "Field `VC` writer - desc VC"]
pub type VC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `ATIM` reader - desc ATIM"]
pub type ATIM_R = crate::svd::BitReader;
#[doc = "Field `ATIM` writer - desc ATIM"]
pub type ATIM_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1` reader - desc SPI1"]
pub type SPI1_R = crate::svd::BitReader;
#[doc = "Field `SPI1` writer - desc SPI1"]
pub type SPI1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `UART1` reader - desc UART1"]
pub type UART1_R = crate::svd::BitReader;
#[doc = "Field `UART1` writer - desc UART1"]
pub type UART1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `GTIM3` reader - desc GTIM3"]
pub type GTIM3_R = crate::svd::BitReader;
#[doc = "Field `GTIM3` writer - desc GTIM3"]
pub type GTIM3_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `GTIM4` reader - desc GTIM4"]
pub type GTIM4_R = crate::svd::BitReader;
#[doc = "Field `GTIM4` writer - desc GTIM4"]
pub type GTIM4_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BTIM` reader - desc BTIM"]
pub type BTIM_R = crate::svd::BitReader;
#[doc = "Field `BTIM` writer - desc BTIM"]
pub type BTIM_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `AWT` reader - desc AWT"]
pub type AWT_R = crate::svd::BitReader;
#[doc = "Field `AWT` writer - desc AWT"]
pub type AWT_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - desc ADC"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc VC"]
    #[inline(always)]
    pub fn vc(&self) -> VC_R {
        VC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&self) -> ATIM_R {
        ATIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc GTIM3"]
    #[inline(always)]
    pub fn gtim3(&self) -> GTIM3_R {
        GTIM3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc GTIM4"]
    #[inline(always)]
    pub fn gtim4(&self) -> GTIM4_R {
        GTIM4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc BTIM"]
    #[inline(always)]
    pub fn btim(&self) -> BTIM_R {
        BTIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc AWT"]
    #[inline(always)]
    pub fn awt(&self) -> AWT_R {
        AWT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - desc ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<APBRST2_SPEC, 2> {
        ADC_W::new(self)
    }
    #[doc = "Bit 4 - desc VC"]
    #[inline(always)]
    #[must_use]
    pub fn vc(&mut self) -> VC_W<APBRST2_SPEC, 4> {
        VC_W::new(self)
    }
    #[doc = "Bit 7 - desc ATIM"]
    #[inline(always)]
    #[must_use]
    pub fn atim(&mut self) -> ATIM_W<APBRST2_SPEC, 7> {
        ATIM_W::new(self)
    }
    #[doc = "Bit 8 - desc SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<APBRST2_SPEC, 8> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 9 - desc UART1"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<APBRST2_SPEC, 9> {
        UART1_W::new(self)
    }
    #[doc = "Bit 10 - desc GTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn gtim3(&mut self) -> GTIM3_W<APBRST2_SPEC, 10> {
        GTIM3_W::new(self)
    }
    #[doc = "Bit 11 - desc GTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn gtim4(&mut self) -> GTIM4_W<APBRST2_SPEC, 11> {
        GTIM4_W::new(self)
    }
    #[doc = "Bit 12 - desc BTIM"]
    #[inline(always)]
    #[must_use]
    pub fn btim(&mut self) -> BTIM_W<APBRST2_SPEC, 12> {
        BTIM_W::new(self)
    }
    #[doc = "Bit 13 - desc AWT"]
    #[inline(always)]
    #[must_use]
    pub fn awt(&mut self) -> AWT_W<APBRST2_SPEC, 13> {
        AWT_W::new(self)
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
#[doc = "APB Reset Control Reg2\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`apbrst2::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`apbrst2::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBRST2_SPEC;
impl crate::svd::RegisterSpec for APBRST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrst2::R`](R) reader structure"]
impl crate::svd::Readable for APBRST2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbrst2::W`](W) writer structure"]
impl crate::svd::Writable for APBRST2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBRST2 to value 0"]
impl crate::svd::Resettable for APBRST2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
