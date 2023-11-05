#[doc = "Register `TRIGGER` reader"]
pub type R = crate::svd::R<TRIGGER_SPEC>;
#[doc = "Register `TRIGGER` writer"]
pub type W = crate::svd::W<TRIGGER_SPEC>;
#[doc = "Field `ATIM` reader - desc ATIM"]
pub type ATIM_R = crate::svd::BitReader;
#[doc = "Field `ATIM` writer - desc ATIM"]
pub type ATIM_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `GTIM1` reader - desc GTIM1"]
pub type GTIM1_R = crate::svd::BitReader;
#[doc = "Field `GTIM1` writer - desc GTIM1"]
pub type GTIM1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `GTIM2` reader - desc GTIM2"]
pub type GTIM2_R = crate::svd::BitReader;
#[doc = "Field `GTIM2` writer - desc GTIM2"]
pub type GTIM2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `GTIM3` reader - desc GTIM3"]
pub type GTIM3_R = crate::svd::BitReader;
#[doc = "Field `GTIM3` writer - desc GTIM3"]
pub type GTIM3_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `GTIM4` reader - desc GTIM4"]
pub type GTIM4_R = crate::svd::BitReader;
#[doc = "Field `GTIM4` writer - desc GTIM4"]
pub type GTIM4_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BTIM1` reader - desc BTIM1"]
pub type BTIM1_R = crate::svd::BitReader;
#[doc = "Field `BTIM1` writer - desc BTIM1"]
pub type BTIM1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BTIM2` reader - desc BTIM2"]
pub type BTIM2_R = crate::svd::BitReader;
#[doc = "Field `BTIM2` writer - desc BTIM2"]
pub type BTIM2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BTIM3` reader - desc BTIM3"]
pub type BTIM3_R = crate::svd::BitReader;
#[doc = "Field `BTIM3` writer - desc BTIM3"]
pub type BTIM3_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `UART1` reader - desc UART1"]
pub type UART1_R = crate::svd::BitReader;
#[doc = "Field `UART1` writer - desc UART1"]
pub type UART1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `UART2` reader - desc UART2"]
pub type UART2_R = crate::svd::BitReader;
#[doc = "Field `UART2` writer - desc UART2"]
pub type UART2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `UART3` reader - desc UART3"]
pub type UART3_R = crate::svd::BitReader;
#[doc = "Field `UART3` writer - desc UART3"]
pub type UART3_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1` reader - desc SPI1"]
pub type SPI1_R = crate::svd::BitReader;
#[doc = "Field `SPI1` writer - desc SPI1"]
pub type SPI1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SPI2` reader - desc SPI2"]
pub type SPI2_R = crate::svd::BitReader;
#[doc = "Field `SPI2` writer - desc SPI2"]
pub type SPI2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1` reader - desc I2C1"]
pub type I2C1_R = crate::svd::BitReader;
#[doc = "Field `I2C1` writer - desc I2C1"]
pub type I2C1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2` reader - desc I2C2"]
pub type I2C2_R = crate::svd::BitReader;
#[doc = "Field `I2C2` writer - desc I2C2"]
pub type I2C2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `DMA` reader - desc DMA"]
pub type DMA_R = crate::svd::BitReader;
#[doc = "Field `DMA` writer - desc DMA"]
pub type DMA_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 5 - desc BTIM1"]
    #[inline(always)]
    pub fn btim1(&self) -> BTIM1_R {
        BTIM1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc BTIM2"]
    #[inline(always)]
    pub fn btim2(&self) -> BTIM2_R {
        BTIM2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BTIM3"]
    #[inline(always)]
    pub fn btim3(&self) -> BTIM3_R {
        BTIM3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc UART3"]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc I2C2"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ATIM"]
    #[inline(always)]
    #[must_use]
    pub fn atim(&mut self) -> ATIM_W<TRIGGER_SPEC, 0> {
        ATIM_W::new(self)
    }
    #[doc = "Bit 1 - desc GTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn gtim1(&mut self) -> GTIM1_W<TRIGGER_SPEC, 1> {
        GTIM1_W::new(self)
    }
    #[doc = "Bit 2 - desc GTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn gtim2(&mut self) -> GTIM2_W<TRIGGER_SPEC, 2> {
        GTIM2_W::new(self)
    }
    #[doc = "Bit 3 - desc GTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn gtim3(&mut self) -> GTIM3_W<TRIGGER_SPEC, 3> {
        GTIM3_W::new(self)
    }
    #[doc = "Bit 4 - desc GTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn gtim4(&mut self) -> GTIM4_W<TRIGGER_SPEC, 4> {
        GTIM4_W::new(self)
    }
    #[doc = "Bit 5 - desc BTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn btim1(&mut self) -> BTIM1_W<TRIGGER_SPEC, 5> {
        BTIM1_W::new(self)
    }
    #[doc = "Bit 6 - desc BTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn btim2(&mut self) -> BTIM2_W<TRIGGER_SPEC, 6> {
        BTIM2_W::new(self)
    }
    #[doc = "Bit 7 - desc BTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn btim3(&mut self) -> BTIM3_W<TRIGGER_SPEC, 7> {
        BTIM3_W::new(self)
    }
    #[doc = "Bit 8 - desc UART1"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<TRIGGER_SPEC, 8> {
        UART1_W::new(self)
    }
    #[doc = "Bit 9 - desc UART2"]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> UART2_W<TRIGGER_SPEC, 9> {
        UART2_W::new(self)
    }
    #[doc = "Bit 10 - desc UART3"]
    #[inline(always)]
    #[must_use]
    pub fn uart3(&mut self) -> UART3_W<TRIGGER_SPEC, 10> {
        UART3_W::new(self)
    }
    #[doc = "Bit 11 - desc SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<TRIGGER_SPEC, 11> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 12 - desc SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<TRIGGER_SPEC, 12> {
        SPI2_W::new(self)
    }
    #[doc = "Bit 13 - desc I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<TRIGGER_SPEC, 13> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 14 - desc I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<TRIGGER_SPEC, 14> {
        I2C2_W::new(self)
    }
    #[doc = "Bit 15 - desc DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<TRIGGER_SPEC, 15> {
        DMA_W::new(self)
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
#[doc = "desc TRIGGER\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`trigger::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`trigger::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIGGER_SPEC;
impl crate::svd::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger::R`](R) reader structure"]
impl crate::svd::Readable for TRIGGER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::svd::Writable for TRIGGER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::svd::Resettable for TRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
