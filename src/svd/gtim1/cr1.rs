#[doc = "Register `CR1` reader"]
pub type R = crate::svd::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::svd::W<CR1_SPEC>;
#[doc = "Field `CH1FLT` reader - desc CH1FLT"]
pub type CH1FLT_R = crate::svd::FieldReader;
#[doc = "Field `CH1FLT` writer - desc CH1FLT"]
pub type CH1FLT_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CH1POL` reader - desc CH1POL"]
pub type CH1POL_R = crate::svd::BitReader;
#[doc = "Field `CH1POL` writer - desc CH1POL"]
pub type CH1POL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CH2FLT` reader - desc CH2FLT"]
pub type CH2FLT_R = crate::svd::FieldReader;
#[doc = "Field `CH2FLT` writer - desc CH2FLT"]
pub type CH2FLT_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CH2POL` reader - desc CH2POL"]
pub type CH2POL_R = crate::svd::BitReader;
#[doc = "Field `CH2POL` writer - desc CH2POL"]
pub type CH2POL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CH3FLT` reader - desc CH3FLT"]
pub type CH3FLT_R = crate::svd::FieldReader;
#[doc = "Field `CH3FLT` writer - desc CH3FLT"]
pub type CH3FLT_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CH3POL` reader - desc CH3POL"]
pub type CH3POL_R = crate::svd::BitReader;
#[doc = "Field `CH3POL` writer - desc CH3POL"]
pub type CH3POL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CH4FLT` reader - desc CH4FLT"]
pub type CH4FLT_R = crate::svd::FieldReader;
#[doc = "Field `CH4FLT` writer - desc CH4FLT"]
pub type CH4FLT_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CH4POL` reader - desc CH4POL"]
pub type CH4POL_R = crate::svd::BitReader;
#[doc = "Field `CH4POL` writer - desc CH4POL"]
pub type CH4POL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - desc CH1FLT"]
    #[inline(always)]
    pub fn ch1flt(&self) -> CH1FLT_R {
        CH1FLT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc CH1POL"]
    #[inline(always)]
    pub fn ch1pol(&self) -> CH1POL_R {
        CH1POL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc CH2FLT"]
    #[inline(always)]
    pub fn ch2flt(&self) -> CH2FLT_R {
        CH2FLT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc CH2POL"]
    #[inline(always)]
    pub fn ch2pol(&self) -> CH2POL_R {
        CH2POL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc CH3FLT"]
    #[inline(always)]
    pub fn ch3flt(&self) -> CH3FLT_R {
        CH3FLT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - desc CH3POL"]
    #[inline(always)]
    pub fn ch3pol(&self) -> CH3POL_R {
        CH3POL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - desc CH4FLT"]
    #[inline(always)]
    pub fn ch4flt(&self) -> CH4FLT_R {
        CH4FLT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - desc CH4POL"]
    #[inline(always)]
    pub fn ch4pol(&self) -> CH4POL_R {
        CH4POL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc CH1FLT"]
    #[inline(always)]
    #[must_use]
    pub fn ch1flt(&mut self) -> CH1FLT_W<CR1_SPEC, 0> {
        CH1FLT_W::new(self)
    }
    #[doc = "Bit 3 - desc CH1POL"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pol(&mut self) -> CH1POL_W<CR1_SPEC, 3> {
        CH1POL_W::new(self)
    }
    #[doc = "Bits 4:6 - desc CH2FLT"]
    #[inline(always)]
    #[must_use]
    pub fn ch2flt(&mut self) -> CH2FLT_W<CR1_SPEC, 4> {
        CH2FLT_W::new(self)
    }
    #[doc = "Bit 7 - desc CH2POL"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pol(&mut self) -> CH2POL_W<CR1_SPEC, 7> {
        CH2POL_W::new(self)
    }
    #[doc = "Bits 8:10 - desc CH3FLT"]
    #[inline(always)]
    #[must_use]
    pub fn ch3flt(&mut self) -> CH3FLT_W<CR1_SPEC, 8> {
        CH3FLT_W::new(self)
    }
    #[doc = "Bit 11 - desc CH3POL"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pol(&mut self) -> CH3POL_W<CR1_SPEC, 11> {
        CH3POL_W::new(self)
    }
    #[doc = "Bits 12:14 - desc CH4FLT"]
    #[inline(always)]
    #[must_use]
    pub fn ch4flt(&mut self) -> CH4FLT_W<CR1_SPEC, 12> {
        CH4FLT_W::new(self)
    }
    #[doc = "Bit 15 - desc CH4POL"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pol(&mut self) -> CH4POL_W<CR1_SPEC, 15> {
        CH4POL_W::new(self)
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
