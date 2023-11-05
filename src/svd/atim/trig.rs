#[doc = "Register `TRIG` reader"]
pub type R = crate::svd::R<TRIG_SPEC>;
#[doc = "Register `TRIG` writer"]
pub type W = crate::svd::W<TRIG_SPEC>;
#[doc = "Field `UEVE` reader - desc UEVE"]
pub type UEVE_R = crate::svd::BitReader;
#[doc = "Field `UEVE` writer - desc UEVE"]
pub type UEVE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CM1AE` reader - desc CM1AE"]
pub type CM1AE_R = crate::svd::BitReader;
#[doc = "Field `CM1AE` writer - desc CM1AE"]
pub type CM1AE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CM2AE` reader - desc CM2AE"]
pub type CM2AE_R = crate::svd::BitReader;
#[doc = "Field `CM2AE` writer - desc CM2AE"]
pub type CM2AE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CM3AE` reader - desc CM3AE"]
pub type CM3AE_R = crate::svd::BitReader;
#[doc = "Field `CM3AE` writer - desc CM3AE"]
pub type CM3AE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CM1BE` reader - desc CM1BE"]
pub type CM1BE_R = crate::svd::BitReader;
#[doc = "Field `CM1BE` writer - desc CM1BE"]
pub type CM1BE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CM2BE` reader - desc CM2BE"]
pub type CM2BE_R = crate::svd::BitReader;
#[doc = "Field `CM2BE` writer - desc CM2BE"]
pub type CM2BE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CM3BE` reader - desc CM3BE"]
pub type CM3BE_R = crate::svd::BitReader;
#[doc = "Field `CM3BE` writer - desc CM3BE"]
pub type CM3BE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `ADTE` reader - desc ADTE"]
pub type ADTE_R = crate::svd::BitReader;
#[doc = "Field `ADTE` writer - desc ADTE"]
pub type ADTE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc UEVE"]
    #[inline(always)]
    pub fn ueve(&self) -> UEVE_R {
        UEVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CM1AE"]
    #[inline(always)]
    pub fn cm1ae(&self) -> CM1AE_R {
        CM1AE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CM2AE"]
    #[inline(always)]
    pub fn cm2ae(&self) -> CM2AE_R {
        CM2AE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CM3AE"]
    #[inline(always)]
    pub fn cm3ae(&self) -> CM3AE_R {
        CM3AE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CM1BE"]
    #[inline(always)]
    pub fn cm1be(&self) -> CM1BE_R {
        CM1BE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CM2BE"]
    #[inline(always)]
    pub fn cm2be(&self) -> CM2BE_R {
        CM2BE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CM3BE"]
    #[inline(always)]
    pub fn cm3be(&self) -> CM3BE_R {
        CM3BE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ADTE"]
    #[inline(always)]
    pub fn adte(&self) -> ADTE_R {
        ADTE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UEVE"]
    #[inline(always)]
    #[must_use]
    pub fn ueve(&mut self) -> UEVE_W<TRIG_SPEC, 0> {
        UEVE_W::new(self)
    }
    #[doc = "Bit 1 - desc CM1AE"]
    #[inline(always)]
    #[must_use]
    pub fn cm1ae(&mut self) -> CM1AE_W<TRIG_SPEC, 1> {
        CM1AE_W::new(self)
    }
    #[doc = "Bit 2 - desc CM2AE"]
    #[inline(always)]
    #[must_use]
    pub fn cm2ae(&mut self) -> CM2AE_W<TRIG_SPEC, 2> {
        CM2AE_W::new(self)
    }
    #[doc = "Bit 3 - desc CM3AE"]
    #[inline(always)]
    #[must_use]
    pub fn cm3ae(&mut self) -> CM3AE_W<TRIG_SPEC, 3> {
        CM3AE_W::new(self)
    }
    #[doc = "Bit 4 - desc CM1BE"]
    #[inline(always)]
    #[must_use]
    pub fn cm1be(&mut self) -> CM1BE_W<TRIG_SPEC, 4> {
        CM1BE_W::new(self)
    }
    #[doc = "Bit 5 - desc CM2BE"]
    #[inline(always)]
    #[must_use]
    pub fn cm2be(&mut self) -> CM2BE_W<TRIG_SPEC, 5> {
        CM2BE_W::new(self)
    }
    #[doc = "Bit 6 - desc CM3BE"]
    #[inline(always)]
    #[must_use]
    pub fn cm3be(&mut self) -> CM3BE_W<TRIG_SPEC, 6> {
        CM3BE_W::new(self)
    }
    #[doc = "Bit 7 - desc ADTE"]
    #[inline(always)]
    #[must_use]
    pub fn adte(&mut self) -> ADTE_W<TRIG_SPEC, 7> {
        ADTE_W::new(self)
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
#[doc = "desc TRIG\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`trig::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`trig::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIG_SPEC;
impl crate::svd::RegisterSpec for TRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trig::R`](R) reader structure"]
impl crate::svd::Readable for TRIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trig::W`](W) writer structure"]
impl crate::svd::Writable for TRIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIG to value 0"]
impl crate::svd::Resettable for TRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
