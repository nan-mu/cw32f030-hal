#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `CHMUX` reader - desc CHMUX"]
pub type CHMUX_R = crate::FieldReader;
#[doc = "Field `CHMUX` writer - desc CHMUX"]
pub type CHMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DISCARD` reader - desc DISCARD"]
pub type DISCARD_R = crate::BitReader;
#[doc = "Field `DISCARD` writer - desc DISCARD"]
pub type DISCARD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALIGN` reader - desc ALIGN"]
pub type ALIGN_R = crate::BitReader;
#[doc = "Field `ALIGN` writer - desc ALIGN"]
pub type ALIGN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAEN` reader - desc DMAEN"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - desc DMAEN"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDTCH` reader - desc WDTCH"]
pub type WDTCH_R = crate::FieldReader;
#[doc = "Field `WDTCH` writer - desc WDTCH"]
pub type WDTCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `WDTALL` reader - desc WDTALL"]
pub type WDTALL_R = crate::BitReader;
#[doc = "Field `WDTALL` writer - desc WDTALL"]
pub type WDTALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - desc CHMUX"]
    #[inline(always)]
    pub fn chmux(&self) -> CHMUX_R {
        CHMUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - desc DISCARD"]
    #[inline(always)]
    pub fn discard(&self) -> DISCARD_R {
        DISCARD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ALIGN"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - desc WDTCH"]
    #[inline(always)]
    pub fn wdtch(&self) -> WDTCH_R {
        WDTCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - desc WDTALL"]
    #[inline(always)]
    pub fn wdtall(&self) -> WDTALL_R {
        WDTALL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CHMUX"]
    #[inline(always)]
    #[must_use]
    pub fn chmux(&mut self) -> CHMUX_W<CR1_SPEC, 0> {
        CHMUX_W::new(self)
    }
    #[doc = "Bit 5 - desc DISCARD"]
    #[inline(always)]
    #[must_use]
    pub fn discard(&mut self) -> DISCARD_W<CR1_SPEC, 5> {
        DISCARD_W::new(self)
    }
    #[doc = "Bit 6 - desc ALIGN"]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<CR1_SPEC, 6> {
        ALIGN_W::new(self)
    }
    #[doc = "Bit 7 - desc DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CR1_SPEC, 7> {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 8:11 - desc WDTCH"]
    #[inline(always)]
    #[must_use]
    pub fn wdtch(&mut self) -> WDTCH_W<CR1_SPEC, 8> {
        WDTCH_W::new(self)
    }
    #[doc = "Bit 13 - desc WDTALL"]
    #[inline(always)]
    #[must_use]
    pub fn wdtall(&mut self) -> WDTALL_W<CR1_SPEC, 13> {
        WDTALL_W::new(self)
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
#[doc = "Control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
