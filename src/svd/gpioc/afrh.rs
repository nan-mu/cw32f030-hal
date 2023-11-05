#[doc = "Register `AFRH` reader"]
pub type R = crate::svd::R<AFRH_SPEC>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::svd::W<AFRH_SPEC>;
#[doc = "Field `AFR13` reader - desc AFR13"]
pub type AFR13_R = crate::svd::FieldReader;
#[doc = "Field `AFR13` writer - desc AFR13"]
pub type AFR13_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AFR14` reader - desc AFR14"]
pub type AFR14_R = crate::svd::FieldReader;
#[doc = "Field `AFR14` writer - desc AFR14"]
pub type AFR14_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AFR15` reader - desc AFR15"]
pub type AFR15_R = crate::svd::FieldReader;
#[doc = "Field `AFR15` writer - desc AFR15"]
pub type AFR15_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 20:23 - desc AFR13"]
    #[inline(always)]
    pub fn afr13(&self) -> AFR13_R {
        AFR13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc AFR14"]
    #[inline(always)]
    pub fn afr14(&self) -> AFR14_R {
        AFR14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - desc AFR15"]
    #[inline(always)]
    pub fn afr15(&self) -> AFR15_R {
        AFR15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - desc AFR13"]
    #[inline(always)]
    #[must_use]
    pub fn afr13(&mut self) -> AFR13_W<AFRH_SPEC, 20> {
        AFR13_W::new(self)
    }
    #[doc = "Bits 24:27 - desc AFR14"]
    #[inline(always)]
    #[must_use]
    pub fn afr14(&mut self) -> AFR14_W<AFRH_SPEC, 24> {
        AFR14_W::new(self)
    }
    #[doc = "Bits 28:31 - desc AFR15"]
    #[inline(always)]
    #[must_use]
    pub fn afr15(&mut self) -> AFR15_W<AFRH_SPEC, 28> {
        AFR15_W::new(self)
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
#[doc = "desc AFRH\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`afrh::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRH_SPEC;
impl crate::svd::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::svd::Readable for AFRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::svd::Writable for AFRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::svd::Resettable for AFRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
