#[doc = "Register `CR1` reader"]
pub type R = crate::svd::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::svd::W<CR1_SPEC>;
#[doc = "Field `FLTEN` reader - desc FLTEN"]
pub type FLTEN_R = crate::svd::BitReader;
#[doc = "Field `FLTEN` writer - desc FLTEN"]
pub type FLTEN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `FLTTIME` reader - desc FLTTIME"]
pub type FLTTIME_R = crate::svd::FieldReader;
#[doc = "Field `FLTTIME` writer - desc FLTTIME"]
pub type FLTTIME_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `FLTCLK` reader - desc FLTCLK"]
pub type FLTCLK_R = crate::svd::BitReader;
#[doc = "Field `FLTCLK` writer - desc FLTCLK"]
pub type FLTCLK_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `FALLIE` reader - desc FALLIE"]
pub type FALLIE_R = crate::svd::BitReader;
#[doc = "Field `FALLIE` writer - desc FALLIE"]
pub type FALLIE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `RISEIE` reader - desc RISEIE"]
pub type RISEIE_R = crate::svd::BitReader;
#[doc = "Field `RISEIE` writer - desc RISEIE"]
pub type RISEIE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `HIGHIE` reader - desc HIGHIE"]
pub type HIGHIE_R = crate::svd::BitReader;
#[doc = "Field `HIGHIE` writer - desc HIGHIE"]
pub type HIGHIE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `ATIMCLR` reader - desc ATIMCLR"]
pub type ATIMCLR_R = crate::svd::BitReader;
#[doc = "Field `ATIMCLR` writer - desc ATIMCLR"]
pub type ATIMCLR_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `ATIMBK` reader - desc ATIMBK"]
pub type ATIMBK_R = crate::svd::BitReader;
#[doc = "Field `ATIMBK` writer - desc ATIMBK"]
pub type ATIMBK_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BLANKCH1B` reader - desc BLANKCH1B"]
pub type BLANKCH1B_R = crate::svd::BitReader;
#[doc = "Field `BLANKCH1B` writer - desc BLANKCH1B"]
pub type BLANKCH1B_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BLANKCH2B` reader - desc BLANKCH2B"]
pub type BLANKCH2B_R = crate::svd::BitReader;
#[doc = "Field `BLANKCH2B` writer - desc BLANKCH2B"]
pub type BLANKCH2B_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BLANKCH3B` reader - desc BLANKCH3B"]
pub type BLANKCH3B_R = crate::svd::BitReader;
#[doc = "Field `BLANKCH3B` writer - desc BLANKCH3B"]
pub type BLANKCH3B_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BLANKFLT` reader - desc BLANKFLT"]
pub type BLANKFLT_R = crate::svd::FieldReader;
#[doc = "Field `BLANKFLT` writer - desc BLANKFLT"]
pub type BLANKFLT_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - desc FLTEN"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - desc FLTTIME"]
    #[inline(always)]
    pub fn flttime(&self) -> FLTTIME_R {
        FLTTIME_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&self) -> FLTCLK_R {
        FLTCLK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc FALLIE"]
    #[inline(always)]
    pub fn fallie(&self) -> FALLIE_R {
        FALLIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RISEIE"]
    #[inline(always)]
    pub fn riseie(&self) -> RISEIE_R {
        RISEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HIGHIE"]
    #[inline(always)]
    pub fn highie(&self) -> HIGHIE_R {
        HIGHIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ATIMCLR"]
    #[inline(always)]
    pub fn atimclr(&self) -> ATIMCLR_R {
        ATIMCLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ATIMBK"]
    #[inline(always)]
    pub fn atimbk(&self) -> ATIMBK_R {
        ATIMBK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BLANKCH1B"]
    #[inline(always)]
    pub fn blankch1b(&self) -> BLANKCH1B_R {
        BLANKCH1B_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc BLANKCH2B"]
    #[inline(always)]
    pub fn blankch2b(&self) -> BLANKCH2B_R {
        BLANKCH2B_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc BLANKCH3B"]
    #[inline(always)]
    pub fn blankch3b(&self) -> BLANKCH3B_R {
        BLANKCH3B_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - desc BLANKFLT"]
    #[inline(always)]
    pub fn blankflt(&self) -> BLANKFLT_R {
        BLANKFLT_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc FLTEN"]
    #[inline(always)]
    #[must_use]
    pub fn flten(&mut self) -> FLTEN_W<CR1_SPEC, 0> {
        FLTEN_W::new(self)
    }
    #[doc = "Bits 1:3 - desc FLTTIME"]
    #[inline(always)]
    #[must_use]
    pub fn flttime(&mut self) -> FLTTIME_W<CR1_SPEC, 1> {
        FLTTIME_W::new(self)
    }
    #[doc = "Bit 4 - desc FLTCLK"]
    #[inline(always)]
    #[must_use]
    pub fn fltclk(&mut self) -> FLTCLK_W<CR1_SPEC, 4> {
        FLTCLK_W::new(self)
    }
    #[doc = "Bit 5 - desc FALLIE"]
    #[inline(always)]
    #[must_use]
    pub fn fallie(&mut self) -> FALLIE_W<CR1_SPEC, 5> {
        FALLIE_W::new(self)
    }
    #[doc = "Bit 6 - desc RISEIE"]
    #[inline(always)]
    #[must_use]
    pub fn riseie(&mut self) -> RISEIE_W<CR1_SPEC, 6> {
        RISEIE_W::new(self)
    }
    #[doc = "Bit 7 - desc HIGHIE"]
    #[inline(always)]
    #[must_use]
    pub fn highie(&mut self) -> HIGHIE_W<CR1_SPEC, 7> {
        HIGHIE_W::new(self)
    }
    #[doc = "Bit 8 - desc ATIMCLR"]
    #[inline(always)]
    #[must_use]
    pub fn atimclr(&mut self) -> ATIMCLR_W<CR1_SPEC, 8> {
        ATIMCLR_W::new(self)
    }
    #[doc = "Bit 9 - desc ATIMBK"]
    #[inline(always)]
    #[must_use]
    pub fn atimbk(&mut self) -> ATIMBK_W<CR1_SPEC, 9> {
        ATIMBK_W::new(self)
    }
    #[doc = "Bit 10 - desc BLANKCH1B"]
    #[inline(always)]
    #[must_use]
    pub fn blankch1b(&mut self) -> BLANKCH1B_W<CR1_SPEC, 10> {
        BLANKCH1B_W::new(self)
    }
    #[doc = "Bit 11 - desc BLANKCH2B"]
    #[inline(always)]
    #[must_use]
    pub fn blankch2b(&mut self) -> BLANKCH2B_W<CR1_SPEC, 11> {
        BLANKCH2B_W::new(self)
    }
    #[doc = "Bit 12 - desc BLANKCH3B"]
    #[inline(always)]
    #[must_use]
    pub fn blankch3b(&mut self) -> BLANKCH3B_W<CR1_SPEC, 12> {
        BLANKCH3B_W::new(self)
    }
    #[doc = "Bits 13:15 - desc BLANKFLT"]
    #[inline(always)]
    #[must_use]
    pub fn blankflt(&mut self) -> BLANKFLT_W<CR1_SPEC, 13> {
        BLANKFLT_W::new(self)
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
#[doc = "Control register1\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::svd::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::svd::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::svd::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::svd::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
