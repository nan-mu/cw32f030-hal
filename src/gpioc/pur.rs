#[doc = "Register `PUR` reader"]
pub type R = crate::R<PUR_SPEC>;
#[doc = "Register `PUR` writer"]
pub type W = crate::W<PUR_SPEC>;
#[doc = "Field `PIN13` reader - desc PIN13"]
pub type PIN13_R = crate::BitReader;
#[doc = "Field `PIN13` writer - desc PIN13"]
pub type PIN13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN14` reader - desc PIN14"]
pub type PIN14_R = crate::BitReader;
#[doc = "Field `PIN14` writer - desc PIN14"]
pub type PIN14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN15` reader - desc PIN15"]
pub type PIN15_R = crate::BitReader;
#[doc = "Field `PIN15` writer - desc PIN15"]
pub type PIN15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 13 - desc PIN13"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PIN14"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PIN15"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - desc PIN13"]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> PIN13_W<PUR_SPEC, 13> {
        PIN13_W::new(self)
    }
    #[doc = "Bit 14 - desc PIN14"]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> PIN14_W<PUR_SPEC, 14> {
        PIN14_W::new(self)
    }
    #[doc = "Bit 15 - desc PIN15"]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> PIN15_W<PUR_SPEC, 15> {
        PIN15_W::new(self)
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
#[doc = "desc PUR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUR_SPEC;
impl crate::RegisterSpec for PUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pur::R`](R) reader structure"]
impl crate::Readable for PUR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pur::W`](W) writer structure"]
impl crate::Writable for PUR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUR to value 0"]
impl crate::Resettable for PUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
