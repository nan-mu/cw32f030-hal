#[doc = "Register `AFRL` reader"]
pub type R = crate::svd::R<AFRL_SPEC>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::svd::W<AFRL_SPEC>;
#[doc = "Field `AFR0` reader - desc AFR0"]
pub type AFR0_R = crate::svd::FieldReader;
#[doc = "Field `AFR0` writer - desc AFR0"]
pub type AFR0_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AFR1` reader - desc AFR1"]
pub type AFR1_R = crate::svd::FieldReader;
#[doc = "Field `AFR1` writer - desc AFR1"]
pub type AFR1_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AFR3` reader - desc AFR3"]
pub type AFR3_R = crate::svd::FieldReader;
#[doc = "Field `AFR3` writer - desc AFR3"]
pub type AFR3_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AFR6` reader - desc AFR6"]
pub type AFR6_R = crate::svd::FieldReader;
#[doc = "Field `AFR6` writer - desc AFR6"]
pub type AFR6_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AFR7` reader - desc AFR7"]
pub type AFR7_R = crate::svd::FieldReader;
#[doc = "Field `AFR7` writer - desc AFR7"]
pub type AFR7_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - desc AFR0"]
    #[inline(always)]
    pub fn afr0(&self) -> AFR0_R {
        AFR0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc AFR1"]
    #[inline(always)]
    pub fn afr1(&self) -> AFR1_R {
        AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc AFR3"]
    #[inline(always)]
    pub fn afr3(&self) -> AFR3_R {
        AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc AFR6"]
    #[inline(always)]
    pub fn afr6(&self) -> AFR6_R {
        AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - desc AFR7"]
    #[inline(always)]
    pub fn afr7(&self) -> AFR7_R {
        AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc AFR0"]
    #[inline(always)]
    #[must_use]
    pub fn afr0(&mut self) -> AFR0_W<AFRL_SPEC, 0> {
        AFR0_W::new(self)
    }
    #[doc = "Bits 4:7 - desc AFR1"]
    #[inline(always)]
    #[must_use]
    pub fn afr1(&mut self) -> AFR1_W<AFRL_SPEC, 4> {
        AFR1_W::new(self)
    }
    #[doc = "Bits 12:15 - desc AFR3"]
    #[inline(always)]
    #[must_use]
    pub fn afr3(&mut self) -> AFR3_W<AFRL_SPEC, 12> {
        AFR3_W::new(self)
    }
    #[doc = "Bits 24:27 - desc AFR6"]
    #[inline(always)]
    #[must_use]
    pub fn afr6(&mut self) -> AFR6_W<AFRL_SPEC, 24> {
        AFR6_W::new(self)
    }
    #[doc = "Bits 28:31 - desc AFR7"]
    #[inline(always)]
    #[must_use]
    pub fn afr7(&mut self) -> AFR7_W<AFRL_SPEC, 28> {
        AFR7_W::new(self)
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
#[doc = "desc AFRL\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`afrl::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRL_SPEC;
impl crate::svd::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::svd::Readable for AFRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::svd::Writable for AFRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::svd::Resettable for AFRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
