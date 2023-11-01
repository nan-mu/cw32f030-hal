#[doc = "Register `PLL` reader"]
pub type R = crate::R<PLL_SPEC>;
#[doc = "Register `PLL` writer"]
pub type W = crate::W<PLL_SPEC>;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SOURCE_R = crate::FieldReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SOURCE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FREQIN` reader - desc FREQIN"]
pub type FREQIN_R = crate::FieldReader;
#[doc = "Field `FREQIN` writer - desc FREQIN"]
pub type FREQIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MUL` reader - desc MUL"]
pub type MUL_R = crate::FieldReader;
#[doc = "Field `MUL` writer - desc MUL"]
pub type MUL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `FREQOUT` reader - desc FREQOUT"]
pub type FREQOUT_R = crate::FieldReader;
#[doc = "Field `FREQOUT` writer - desc FREQOUT"]
pub type FREQOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `WAITCYCLE` reader - desc WAITCYCLE"]
pub type WAITCYCLE_R = crate::FieldReader;
#[doc = "Field `WAITCYCLE` writer - desc WAITCYCLE"]
pub type WAITCYCLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type STABLE_R = crate::BitReader;
#[doc = "Field `RFU` reader - desc RFU"]
pub type RFU_R = crate::FieldReader;
#[doc = "Field `RFU` writer - desc RFU"]
pub type RFU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:1 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc FREQIN"]
    #[inline(always)]
    pub fn freqin(&self) -> FREQIN_R {
        FREQIN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:8 - desc MUL"]
    #[inline(always)]
    pub fn mul(&self) -> MUL_R {
        MUL_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:11 - desc FREQOUT"]
    #[inline(always)]
    pub fn freqout(&self) -> FREQOUT_R {
        FREQOUT_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&self) -> WAITCYCLE_R {
        WAITCYCLE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - desc RFU"]
    #[inline(always)]
    pub fn rfu(&self) -> RFU_R {
        RFU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<PLL_SPEC, 0> {
        SOURCE_W::new(self)
    }
    #[doc = "Bits 2:3 - desc FREQIN"]
    #[inline(always)]
    #[must_use]
    pub fn freqin(&mut self) -> FREQIN_W<PLL_SPEC, 2> {
        FREQIN_W::new(self)
    }
    #[doc = "Bits 4:8 - desc MUL"]
    #[inline(always)]
    #[must_use]
    pub fn mul(&mut self) -> MUL_W<PLL_SPEC, 4> {
        MUL_W::new(self)
    }
    #[doc = "Bits 9:11 - desc FREQOUT"]
    #[inline(always)]
    #[must_use]
    pub fn freqout(&mut self) -> FREQOUT_W<PLL_SPEC, 9> {
        FREQOUT_W::new(self)
    }
    #[doc = "Bits 12:14 - desc WAITCYCLE"]
    #[inline(always)]
    #[must_use]
    pub fn waitcycle(&mut self) -> WAITCYCLE_W<PLL_SPEC, 12> {
        WAITCYCLE_W::new(self)
    }
    #[doc = "Bits 16:19 - desc RFU"]
    #[inline(always)]
    #[must_use]
    pub fn rfu(&mut self) -> RFU_W<PLL_SPEC, 16> {
        RFU_W::new(self)
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
#[doc = "PLL Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_SPEC;
impl crate::RegisterSpec for PLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll::R`](R) reader structure"]
impl crate::Readable for PLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll::W`](W) writer structure"]
impl crate::Writable for PLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL to value 0"]
impl crate::Resettable for PLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
