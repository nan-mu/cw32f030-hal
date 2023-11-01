#[doc = "Register `AHBEN` reader"]
pub type R = crate::R<AHBEN_SPEC>;
#[doc = "Register `AHBEN` writer"]
pub type W = crate::W<AHBEN_SPEC>;
#[doc = "Field `DMA` reader - desc DMA"]
pub type DMA_R = crate::BitReader;
#[doc = "Field `DMA` writer - desc DMA"]
pub type DMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH` reader - desc FLASH"]
pub type FLASH_R = crate::BitReader;
#[doc = "Field `FLASH` writer - desc FLASH"]
pub type FLASH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRC` reader - desc CRC"]
pub type CRC_R = crate::BitReader;
#[doc = "Field `CRC` writer - desc CRC"]
pub type CRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOA` reader - desc GPIOA"]
pub type GPIOA_R = crate::BitReader;
#[doc = "Field `GPIOA` writer - desc GPIOA"]
pub type GPIOA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOB` reader - desc GPIOB"]
pub type GPIOB_R = crate::BitReader;
#[doc = "Field `GPIOB` writer - desc GPIOB"]
pub type GPIOB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOC` reader - desc GPIOC"]
pub type GPIOC_R = crate::BitReader;
#[doc = "Field `GPIOC` writer - desc GPIOC"]
pub type GPIOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOF` reader - desc GPIOF"]
pub type GPIOF_R = crate::BitReader;
#[doc = "Field `GPIOF` writer - desc GPIOF"]
pub type GPIOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FLASH"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc GPIOA"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc GPIOB"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc GPIOC"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - desc GPIOF"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<AHBEN_SPEC, 0> {
        DMA_W::new(self)
    }
    #[doc = "Bit 1 - desc FLASH"]
    #[inline(always)]
    #[must_use]
    pub fn flash(&mut self) -> FLASH_W<AHBEN_SPEC, 1> {
        FLASH_W::new(self)
    }
    #[doc = "Bit 2 - desc CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<AHBEN_SPEC, 2> {
        CRC_W::new(self)
    }
    #[doc = "Bit 4 - desc GPIOA"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<AHBEN_SPEC, 4> {
        GPIOA_W::new(self)
    }
    #[doc = "Bit 5 - desc GPIOB"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<AHBEN_SPEC, 5> {
        GPIOB_W::new(self)
    }
    #[doc = "Bit 6 - desc GPIOC"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<AHBEN_SPEC, 6> {
        GPIOC_W::new(self)
    }
    #[doc = "Bit 9 - desc GPIOF"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<AHBEN_SPEC, 9> {
        GPIOF_W::new(self)
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
#[doc = "AHB Clock Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN_SPEC;
impl crate::RegisterSpec for AHBEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben::R`](R) reader structure"]
impl crate::Readable for AHBEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben::W`](W) writer structure"]
impl crate::Writable for AHBEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBEN to value 0"]
impl crate::Resettable for AHBEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
