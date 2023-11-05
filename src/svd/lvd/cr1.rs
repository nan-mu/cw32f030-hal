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
#[doc = "Field `RISE` reader - desc RISE"]
pub type RISE_R = crate::svd::BitReader;
#[doc = "Field `RISE` writer - desc RISE"]
pub type RISE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `FALL` reader - desc FALL"]
pub type FALL_R = crate::svd::BitReader;
#[doc = "Field `FALL` writer - desc FALL"]
pub type FALL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LEVEL` reader - desc LEVEL"]
pub type LEVEL_R = crate::svd::BitReader;
#[doc = "Field `LEVEL` writer - desc LEVEL"]
pub type LEVEL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 5 - desc RISE"]
    #[inline(always)]
    pub fn rise(&self) -> RISE_R {
        RISE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc FALL"]
    #[inline(always)]
    pub fn fall(&self) -> FALL_R {
        FALL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc LEVEL"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 5 - desc RISE"]
    #[inline(always)]
    #[must_use]
    pub fn rise(&mut self) -> RISE_W<CR1_SPEC, 5> {
        RISE_W::new(self)
    }
    #[doc = "Bit 6 - desc FALL"]
    #[inline(always)]
    #[must_use]
    pub fn fall(&mut self) -> FALL_W<CR1_SPEC, 6> {
        FALL_W::new(self)
    }
    #[doc = "Bit 7 - desc LEVEL"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<CR1_SPEC, 7> {
        LEVEL_W::new(self)
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
