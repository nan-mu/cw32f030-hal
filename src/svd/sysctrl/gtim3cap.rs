#[doc = "Register `GTIM3CAP` reader"]
pub type R = crate::svd::R<GTIM3CAP_SPEC>;
#[doc = "Register `GTIM3CAP` writer"]
pub type W = crate::svd::W<GTIM3CAP_SPEC>;
#[doc = "Field `CH1` reader - desc CH1"]
pub type CH1_R = crate::svd::FieldReader;
#[doc = "Field `CH1` writer - desc CH1"]
pub type CH1_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CH2` reader - desc CH2"]
pub type CH2_R = crate::svd::FieldReader;
#[doc = "Field `CH2` writer - desc CH2"]
pub type CH2_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CH3` reader - desc CH3"]
pub type CH3_R = crate::svd::FieldReader;
#[doc = "Field `CH3` writer - desc CH3"]
pub type CH3_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CH4` reader - desc CH4"]
pub type CH4_R = crate::svd::FieldReader;
#[doc = "Field `CH4` writer - desc CH4"]
pub type CH4_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - desc CH1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - desc CH2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - desc CH3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc CH4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc CH1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<GTIM3CAP_SPEC, 0> {
        CH1_W::new(self)
    }
    #[doc = "Bits 4:6 - desc CH2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<GTIM3CAP_SPEC, 4> {
        CH2_W::new(self)
    }
    #[doc = "Bits 8:10 - desc CH3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<GTIM3CAP_SPEC, 8> {
        CH3_W::new(self)
    }
    #[doc = "Bits 12:14 - desc CH4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<GTIM3CAP_SPEC, 12> {
        CH4_W::new(self)
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
#[doc = "GTIM3 CAP Control Reg\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`gtim3cap::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`gtim3cap::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTIM3CAP_SPEC;
impl crate::svd::RegisterSpec for GTIM3CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtim3cap::R`](R) reader structure"]
impl crate::svd::Readable for GTIM3CAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtim3cap::W`](W) writer structure"]
impl crate::svd::Writable for GTIM3CAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTIM3CAP to value 0"]
impl crate::svd::Resettable for GTIM3CAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
