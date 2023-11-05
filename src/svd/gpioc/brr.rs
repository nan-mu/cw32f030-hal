#[doc = "Register `BRR` reader"]
pub type R = crate::svd::R<BRR_SPEC>;
#[doc = "Register `BRR` writer"]
pub type W = crate::svd::W<BRR_SPEC>;
#[doc = "Field `BRR13` reader - desc BRR13"]
pub type BRR13_R = crate::svd::BitReader;
#[doc = "Field `BRR13` writer - desc BRR13"]
pub type BRR13_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BRR14` reader - desc BRR14"]
pub type BRR14_R = crate::svd::BitReader;
#[doc = "Field `BRR14` writer - desc BRR14"]
pub type BRR14_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BRR15` reader - desc BRR15"]
pub type BRR15_R = crate::svd::BitReader;
#[doc = "Field `BRR15` writer - desc BRR15"]
pub type BRR15_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 13 - desc BRR13"]
    #[inline(always)]
    pub fn brr13(&self) -> BRR13_R {
        BRR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc BRR14"]
    #[inline(always)]
    pub fn brr14(&self) -> BRR14_R {
        BRR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc BRR15"]
    #[inline(always)]
    pub fn brr15(&self) -> BRR15_R {
        BRR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - desc BRR13"]
    #[inline(always)]
    #[must_use]
    pub fn brr13(&mut self) -> BRR13_W<BRR_SPEC, 13> {
        BRR13_W::new(self)
    }
    #[doc = "Bit 14 - desc BRR14"]
    #[inline(always)]
    #[must_use]
    pub fn brr14(&mut self) -> BRR14_W<BRR_SPEC, 14> {
        BRR14_W::new(self)
    }
    #[doc = "Bit 15 - desc BRR15"]
    #[inline(always)]
    #[must_use]
    pub fn brr15(&mut self) -> BRR15_W<BRR_SPEC, 15> {
        BRR15_W::new(self)
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
#[doc = "desc BRR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`brr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRR_SPEC;
impl crate::svd::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::svd::Readable for BRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::svd::Writable for BRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::svd::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
