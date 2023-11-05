#[doc = "Register `AFRH` reader"]
pub type R = crate::svd::R<AFRH_SPEC>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::svd::W<AFRH_SPEC>;
#[doc = "Field `AFR8` reader - desc AFR8"]
pub type AFR8_R = crate::svd::FieldReader;
#[doc = "Field `AFR8` writer - desc AFR8"]
pub type AFR8_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AFR9` reader - desc AFR9"]
pub type AFR9_R = crate::svd::FieldReader;
#[doc = "Field `AFR9` writer - desc AFR9"]
pub type AFR9_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AFR10` reader - desc AFR10"]
pub type AFR10_R = crate::svd::FieldReader;
#[doc = "Field `AFR10` writer - desc AFR10"]
pub type AFR10_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AFR11` reader - desc AFR11"]
pub type AFR11_R = crate::svd::FieldReader;
#[doc = "Field `AFR11` writer - desc AFR11"]
pub type AFR11_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AFR12` reader - desc AFR12"]
pub type AFR12_R = crate::svd::FieldReader;
#[doc = "Field `AFR12` writer - desc AFR12"]
pub type AFR12_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
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
    #[doc = "Bits 0:3 - desc AFR8"]
    #[inline(always)]
    pub fn afr8(&self) -> AFR8_R {
        AFR8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc AFR9"]
    #[inline(always)]
    pub fn afr9(&self) -> AFR9_R {
        AFR9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc AFR10"]
    #[inline(always)]
    pub fn afr10(&self) -> AFR10_R {
        AFR10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc AFR11"]
    #[inline(always)]
    pub fn afr11(&self) -> AFR11_R {
        AFR11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc AFR12"]
    #[inline(always)]
    pub fn afr12(&self) -> AFR12_R {
        AFR12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
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
    #[doc = "Bits 0:3 - desc AFR8"]
    #[inline(always)]
    #[must_use]
    pub fn afr8(&mut self) -> AFR8_W<AFRH_SPEC, 0> {
        AFR8_W::new(self)
    }
    #[doc = "Bits 4:7 - desc AFR9"]
    #[inline(always)]
    #[must_use]
    pub fn afr9(&mut self) -> AFR9_W<AFRH_SPEC, 4> {
        AFR9_W::new(self)
    }
    #[doc = "Bits 8:11 - desc AFR10"]
    #[inline(always)]
    #[must_use]
    pub fn afr10(&mut self) -> AFR10_W<AFRH_SPEC, 8> {
        AFR10_W::new(self)
    }
    #[doc = "Bits 12:15 - desc AFR11"]
    #[inline(always)]
    #[must_use]
    pub fn afr11(&mut self) -> AFR11_W<AFRH_SPEC, 12> {
        AFR11_W::new(self)
    }
    #[doc = "Bits 16:19 - desc AFR12"]
    #[inline(always)]
    #[must_use]
    pub fn afr12(&mut self) -> AFR12_W<AFRH_SPEC, 16> {
        AFR12_W::new(self)
    }
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
