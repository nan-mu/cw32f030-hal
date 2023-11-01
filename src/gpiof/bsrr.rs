#[doc = "Register `BSRR` reader"]
pub type R = crate::R<BSRR_SPEC>;
#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BSRR_SPEC>;
#[doc = "Field `BSS0` reader - desc BSS0"]
pub type BSS0_R = crate::BitReader;
#[doc = "Field `BSS0` writer - desc BSS0"]
pub type BSS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BSS1` reader - desc BSS1"]
pub type BSS1_R = crate::BitReader;
#[doc = "Field `BSS1` writer - desc BSS1"]
pub type BSS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BSS3` reader - desc BSS3"]
pub type BSS3_R = crate::BitReader;
#[doc = "Field `BSS3` writer - desc BSS3"]
pub type BSS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BSS6` reader - desc BSS6"]
pub type BSS6_R = crate::BitReader;
#[doc = "Field `BSS6` writer - desc BSS6"]
pub type BSS6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BSS7` reader - desc BSS7"]
pub type BSS7_R = crate::BitReader;
#[doc = "Field `BSS7` writer - desc BSS7"]
pub type BSS7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc BSS0"]
    #[inline(always)]
    pub fn bss0(&self) -> BSS0_R {
        BSS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BSS1"]
    #[inline(always)]
    pub fn bss1(&self) -> BSS1_R {
        BSS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc BSS3"]
    #[inline(always)]
    pub fn bss3(&self) -> BSS3_R {
        BSS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - desc BSS6"]
    #[inline(always)]
    pub fn bss6(&self) -> BSS6_R {
        BSS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BSS7"]
    #[inline(always)]
    pub fn bss7(&self) -> BSS7_R {
        BSS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BSS0"]
    #[inline(always)]
    #[must_use]
    pub fn bss0(&mut self) -> BSS0_W<BSRR_SPEC, 0> {
        BSS0_W::new(self)
    }
    #[doc = "Bit 1 - desc BSS1"]
    #[inline(always)]
    #[must_use]
    pub fn bss1(&mut self) -> BSS1_W<BSRR_SPEC, 1> {
        BSS1_W::new(self)
    }
    #[doc = "Bit 3 - desc BSS3"]
    #[inline(always)]
    #[must_use]
    pub fn bss3(&mut self) -> BSS3_W<BSRR_SPEC, 3> {
        BSS3_W::new(self)
    }
    #[doc = "Bit 6 - desc BSS6"]
    #[inline(always)]
    #[must_use]
    pub fn bss6(&mut self) -> BSS6_W<BSRR_SPEC, 6> {
        BSS6_W::new(self)
    }
    #[doc = "Bit 7 - desc BSS7"]
    #[inline(always)]
    #[must_use]
    pub fn bss7(&mut self) -> BSS7_W<BSRR_SPEC, 7> {
        BSS7_W::new(self)
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
#[doc = "desc BSRR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSRR_SPEC;
impl crate::RegisterSpec for BSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsrr::R`](R) reader structure"]
impl crate::Readable for BSRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BSRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
