#[doc = "Register `BRR` reader"]
pub type R = crate::R<BRR_SPEC>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRR_SPEC>;
#[doc = "Field `BRR0` reader - desc BRR0"]
pub type BRR0_R = crate::BitReader;
#[doc = "Field `BRR0` writer - desc BRR0"]
pub type BRR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR1` reader - desc BRR1"]
pub type BRR1_R = crate::BitReader;
#[doc = "Field `BRR1` writer - desc BRR1"]
pub type BRR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR3` reader - desc BRR3"]
pub type BRR3_R = crate::BitReader;
#[doc = "Field `BRR3` writer - desc BRR3"]
pub type BRR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR6` reader - desc BRR6"]
pub type BRR6_R = crate::BitReader;
#[doc = "Field `BRR6` writer - desc BRR6"]
pub type BRR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR7` reader - desc BRR7"]
pub type BRR7_R = crate::BitReader;
#[doc = "Field `BRR7` writer - desc BRR7"]
pub type BRR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc BRR0"]
    #[inline(always)]
    pub fn brr0(&self) -> BRR0_R {
        BRR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BRR1"]
    #[inline(always)]
    pub fn brr1(&self) -> BRR1_R {
        BRR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc BRR3"]
    #[inline(always)]
    pub fn brr3(&self) -> BRR3_R {
        BRR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - desc BRR6"]
    #[inline(always)]
    pub fn brr6(&self) -> BRR6_R {
        BRR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BRR7"]
    #[inline(always)]
    pub fn brr7(&self) -> BRR7_R {
        BRR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BRR0"]
    #[inline(always)]
    #[must_use]
    pub fn brr0(&mut self) -> BRR0_W<BRR_SPEC, 0> {
        BRR0_W::new(self)
    }
    #[doc = "Bit 1 - desc BRR1"]
    #[inline(always)]
    #[must_use]
    pub fn brr1(&mut self) -> BRR1_W<BRR_SPEC, 1> {
        BRR1_W::new(self)
    }
    #[doc = "Bit 3 - desc BRR3"]
    #[inline(always)]
    #[must_use]
    pub fn brr3(&mut self) -> BRR3_W<BRR_SPEC, 3> {
        BRR3_W::new(self)
    }
    #[doc = "Bit 6 - desc BRR6"]
    #[inline(always)]
    #[must_use]
    pub fn brr6(&mut self) -> BRR6_W<BRR_SPEC, 6> {
        BRR6_W::new(self)
    }
    #[doc = "Bit 7 - desc BRR7"]
    #[inline(always)]
    #[must_use]
    pub fn brr7(&mut self) -> BRR7_W<BRR_SPEC, 7> {
        BRR7_W::new(self)
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
#[doc = "desc BRR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
