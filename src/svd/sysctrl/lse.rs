#[doc = "Register `LSE` reader"]
pub type R = crate::svd::R<LSE_SPEC>;
#[doc = "Register `LSE` writer"]
pub type W = crate::svd::W<LSE_SPEC>;
#[doc = "Field `DRIVER` reader - desc DRIVER"]
pub type DRIVER_R = crate::svd::FieldReader;
#[doc = "Field `DRIVER` writer - desc DRIVER"]
pub type DRIVER_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `AMP` reader - desc AMP"]
pub type AMP_R = crate::svd::FieldReader;
#[doc = "Field `AMP` writer - desc AMP"]
pub type AMP_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `WAITCYCLE` reader - desc WAITCYCLE"]
pub type WAITCYCLE_R = crate::svd::FieldReader;
#[doc = "Field `WAITCYCLE` writer - desc WAITCYCLE"]
pub type WAITCYCLE_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::svd::BitReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type STABLE_R = crate::svd::BitReader;
impl R {
    #[doc = "Bits 0:1 - desc DRIVER"]
    #[inline(always)]
    pub fn driver(&self) -> DRIVER_R {
        DRIVER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc AMP"]
    #[inline(always)]
    pub fn amp(&self) -> AMP_R {
        AMP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&self) -> WAITCYCLE_R {
        WAITCYCLE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 15 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc DRIVER"]
    #[inline(always)]
    #[must_use]
    pub fn driver(&mut self) -> DRIVER_W<LSE_SPEC, 0> {
        DRIVER_W::new(self)
    }
    #[doc = "Bits 2:3 - desc AMP"]
    #[inline(always)]
    #[must_use]
    pub fn amp(&mut self) -> AMP_W<LSE_SPEC, 2> {
        AMP_W::new(self)
    }
    #[doc = "Bits 4:5 - desc WAITCYCLE"]
    #[inline(always)]
    #[must_use]
    pub fn waitcycle(&mut self) -> WAITCYCLE_W<LSE_SPEC, 4> {
        WAITCYCLE_W::new(self)
    }
    #[doc = "Bit 6 - desc MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<LSE_SPEC, 6> {
        MODE_W::new(self)
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
#[doc = "LSE Control Reg\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`lse::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`lse::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSE_SPEC;
impl crate::svd::RegisterSpec for LSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lse::R`](R) reader structure"]
impl crate::svd::Readable for LSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lse::W`](W) writer structure"]
impl crate::svd::Writable for LSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSE to value 0"]
impl crate::svd::Resettable for LSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
