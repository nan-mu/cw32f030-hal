#[doc = "Register `BSRR` reader"]
pub type R = crate::svd::R<BSRR_SPEC>;
#[doc = "Register `BSRR` writer"]
pub type W = crate::svd::W<BSRR_SPEC>;
#[doc = "Field `BSS13` reader - desc BSS13"]
pub type BSS13_R = crate::svd::BitReader;
#[doc = "Field `BSS13` writer - desc BSS13"]
pub type BSS13_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BSS14` reader - desc BSS14"]
pub type BSS14_R = crate::svd::BitReader;
#[doc = "Field `BSS14` writer - desc BSS14"]
pub type BSS14_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BSS15` reader - desc BSS15"]
pub type BSS15_R = crate::svd::BitReader;
#[doc = "Field `BSS15` writer - desc BSS15"]
pub type BSS15_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 13 - desc BSS13"]
    #[inline(always)]
    pub fn bss13(&self) -> BSS13_R {
        BSS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc BSS14"]
    #[inline(always)]
    pub fn bss14(&self) -> BSS14_R {
        BSS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc BSS15"]
    #[inline(always)]
    pub fn bss15(&self) -> BSS15_R {
        BSS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - desc BRR13"]
    #[inline(always)]
    pub fn brr13(&self) -> BRR13_R {
        BRR13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc BRR14"]
    #[inline(always)]
    pub fn brr14(&self) -> BRR14_R {
        BRR14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc BRR15"]
    #[inline(always)]
    pub fn brr15(&self) -> BRR15_R {
        BRR15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - desc BSS13"]
    #[inline(always)]
    #[must_use]
    pub fn bss13(&mut self) -> BSS13_W<BSRR_SPEC, 13> {
        BSS13_W::new(self)
    }
    #[doc = "Bit 14 - desc BSS14"]
    #[inline(always)]
    #[must_use]
    pub fn bss14(&mut self) -> BSS14_W<BSRR_SPEC, 14> {
        BSS14_W::new(self)
    }
    #[doc = "Bit 15 - desc BSS15"]
    #[inline(always)]
    #[must_use]
    pub fn bss15(&mut self) -> BSS15_W<BSRR_SPEC, 15> {
        BSS15_W::new(self)
    }
    #[doc = "Bit 29 - desc BRR13"]
    #[inline(always)]
    #[must_use]
    pub fn brr13(&mut self) -> BRR13_W<BSRR_SPEC, 29> {
        BRR13_W::new(self)
    }
    #[doc = "Bit 30 - desc BRR14"]
    #[inline(always)]
    #[must_use]
    pub fn brr14(&mut self) -> BRR14_W<BSRR_SPEC, 30> {
        BRR14_W::new(self)
    }
    #[doc = "Bit 31 - desc BRR15"]
    #[inline(always)]
    #[must_use]
    pub fn brr15(&mut self) -> BRR15_W<BSRR_SPEC, 31> {
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
#[doc = "desc BSRR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`bsrr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`bsrr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSRR_SPEC;
impl crate::svd::RegisterSpec for BSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsrr::R`](R) reader structure"]
impl crate::svd::Readable for BSRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::svd::Writable for BSRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::svd::Resettable for BSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
