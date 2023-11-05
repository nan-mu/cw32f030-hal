#[doc = "Register `AHBRST` reader"]
pub type R = crate::svd::R<AHBRST_SPEC>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::svd::W<AHBRST_SPEC>;
#[doc = "Field `DMA` reader - desc DMA"]
pub type DMA_R = crate::svd::BitReader;
#[doc = "Field `DMA` writer - desc DMA"]
pub type DMA_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH` reader - desc FLASH"]
pub type FLASH_R = crate::svd::BitReader;
#[doc = "Field `FLASH` writer - desc FLASH"]
pub type FLASH_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CRC` reader - desc CRC"]
pub type CRC_R = crate::svd::BitReader;
#[doc = "Field `CRC` writer - desc CRC"]
pub type CRC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOA` reader - desc GPIOA"]
pub type GPIOA_R = crate::svd::BitReader;
#[doc = "Field `GPIOA` writer - desc GPIOA"]
pub type GPIOA_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOB` reader - desc GPIOB"]
pub type GPIOB_R = crate::svd::BitReader;
#[doc = "Field `GPIOB` writer - desc GPIOB"]
pub type GPIOB_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOC` reader - desc GPIOC"]
pub type GPIOC_R = crate::svd::BitReader;
#[doc = "Field `GPIOC` writer - desc GPIOC"]
pub type GPIOC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOF` reader - desc GPIOF"]
pub type GPIOF_R = crate::svd::BitReader;
#[doc = "Field `GPIOF` writer - desc GPIOF"]
pub type GPIOF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
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
    pub fn dma(&mut self) -> DMA_W<AHBRST_SPEC, 0> {
        DMA_W::new(self)
    }
    #[doc = "Bit 1 - desc FLASH"]
    #[inline(always)]
    #[must_use]
    pub fn flash(&mut self) -> FLASH_W<AHBRST_SPEC, 1> {
        FLASH_W::new(self)
    }
    #[doc = "Bit 2 - desc CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<AHBRST_SPEC, 2> {
        CRC_W::new(self)
    }
    #[doc = "Bit 4 - desc GPIOA"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<AHBRST_SPEC, 4> {
        GPIOA_W::new(self)
    }
    #[doc = "Bit 5 - desc GPIOB"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<AHBRST_SPEC, 5> {
        GPIOB_W::new(self)
    }
    #[doc = "Bit 6 - desc GPIOC"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<AHBRST_SPEC, 6> {
        GPIOC_W::new(self)
    }
    #[doc = "Bit 9 - desc GPIOF"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<AHBRST_SPEC, 9> {
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
#[doc = "AHB Reset Control Reg\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ahbrst::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST_SPEC;
impl crate::svd::RegisterSpec for AHBRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::svd::Readable for AHBRST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::svd::Writable for AHBRST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::svd::Resettable for AHBRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
