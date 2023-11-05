#[doc = "Register `DMA` reader"]
pub type R = crate::svd::R<DMA_SPEC>;
#[doc = "Register `DMA` writer"]
pub type W = crate::svd::W<DMA_SPEC>;
#[doc = "Field `OV` reader - desc OV"]
pub type OV_R = crate::svd::BitReader;
#[doc = "Field `OV` writer - desc OV"]
pub type OV_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TRS` reader - desc TRS"]
pub type TRS_R = crate::svd::BitReader;
#[doc = "Field `TRS` writer - desc TRS"]
pub type TRS_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CC1` reader - desc CC1"]
pub type CC1_R = crate::svd::BitReader;
#[doc = "Field `CC1` writer - desc CC1"]
pub type CC1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CC2` reader - desc CC2"]
pub type CC2_R = crate::svd::BitReader;
#[doc = "Field `CC2` writer - desc CC2"]
pub type CC2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CC3` reader - desc CC3"]
pub type CC3_R = crate::svd::BitReader;
#[doc = "Field `CC3` writer - desc CC3"]
pub type CC3_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CC4` reader - desc CC4"]
pub type CC4_R = crate::svd::BitReader;
#[doc = "Field `CC4` writer - desc CC4"]
pub type CC4_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TRS"]
    #[inline(always)]
    pub fn trs(&self) -> TRS_R {
        TRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CC1"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CC2"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CC3"]
    #[inline(always)]
    pub fn cc3(&self) -> CC3_R {
        CC3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CC4"]
    #[inline(always)]
    pub fn cc4(&self) -> CC4_R {
        CC4_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OV"]
    #[inline(always)]
    #[must_use]
    pub fn ov(&mut self) -> OV_W<DMA_SPEC, 0> {
        OV_W::new(self)
    }
    #[doc = "Bit 1 - desc TRS"]
    #[inline(always)]
    #[must_use]
    pub fn trs(&mut self) -> TRS_W<DMA_SPEC, 1> {
        TRS_W::new(self)
    }
    #[doc = "Bit 2 - desc CC1"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> CC1_W<DMA_SPEC, 2> {
        CC1_W::new(self)
    }
    #[doc = "Bit 3 - desc CC2"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> CC2_W<DMA_SPEC, 3> {
        CC2_W::new(self)
    }
    #[doc = "Bit 4 - desc CC3"]
    #[inline(always)]
    #[must_use]
    pub fn cc3(&mut self) -> CC3_W<DMA_SPEC, 4> {
        CC3_W::new(self)
    }
    #[doc = "Bit 5 - desc CC4"]
    #[inline(always)]
    #[must_use]
    pub fn cc4(&mut self) -> CC4_W<DMA_SPEC, 5> {
        CC4_W::new(self)
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
#[doc = "DMA Control register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_SPEC;
impl crate::svd::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::svd::Readable for DMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::svd::Writable for DMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::svd::Resettable for DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
