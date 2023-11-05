#[doc = "Register `TOG` reader"]
pub type R = crate::svd::R<TOG_SPEC>;
#[doc = "Register `TOG` writer"]
pub type W = crate::svd::W<TOG_SPEC>;
#[doc = "Field `PIN13` reader - desc PIN13"]
pub type PIN13_R = crate::svd::BitReader;
#[doc = "Field `PIN13` writer - desc PIN13"]
pub type PIN13_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PIN14` reader - desc PIN14"]
pub type PIN14_R = crate::svd::BitReader;
#[doc = "Field `PIN14` writer - desc PIN14"]
pub type PIN14_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PIN15` reader - desc PIN15"]
pub type PIN15_R = crate::svd::BitReader;
#[doc = "Field `PIN15` writer - desc PIN15"]
pub type PIN15_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
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
    pub fn pin13(&mut self) -> PIN13_W<TOG_SPEC, 13> {
        PIN13_W::new(self)
    }
    #[doc = "Bit 14 - desc PIN14"]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> PIN14_W<TOG_SPEC, 14> {
        PIN14_W::new(self)
    }
    #[doc = "Bit 15 - desc PIN15"]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> PIN15_W<TOG_SPEC, 15> {
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
#[doc = "desc TOG\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`tog::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`tog::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOG_SPEC;
impl crate::svd::RegisterSpec for TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tog::R`](R) reader structure"]
impl crate::svd::Readable for TOG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tog::W`](W) writer structure"]
impl crate::svd::Writable for TOG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOG to value 0"]
impl crate::svd::Resettable for TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
