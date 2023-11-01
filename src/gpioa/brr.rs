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
#[doc = "Field `BRR2` reader - desc BRR2"]
pub type BRR2_R = crate::BitReader;
#[doc = "Field `BRR2` writer - desc BRR2"]
pub type BRR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR3` reader - desc BRR3"]
pub type BRR3_R = crate::BitReader;
#[doc = "Field `BRR3` writer - desc BRR3"]
pub type BRR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR4` reader - desc BRR4"]
pub type BRR4_R = crate::BitReader;
#[doc = "Field `BRR4` writer - desc BRR4"]
pub type BRR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR5` reader - desc BRR5"]
pub type BRR5_R = crate::BitReader;
#[doc = "Field `BRR5` writer - desc BRR5"]
pub type BRR5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR6` reader - desc BRR6"]
pub type BRR6_R = crate::BitReader;
#[doc = "Field `BRR6` writer - desc BRR6"]
pub type BRR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR7` reader - desc BRR7"]
pub type BRR7_R = crate::BitReader;
#[doc = "Field `BRR7` writer - desc BRR7"]
pub type BRR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR8` reader - desc BRR8"]
pub type BRR8_R = crate::BitReader;
#[doc = "Field `BRR8` writer - desc BRR8"]
pub type BRR8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR9` reader - desc BRR9"]
pub type BRR9_R = crate::BitReader;
#[doc = "Field `BRR9` writer - desc BRR9"]
pub type BRR9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR10` reader - desc BRR10"]
pub type BRR10_R = crate::BitReader;
#[doc = "Field `BRR10` writer - desc BRR10"]
pub type BRR10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR11` reader - desc BRR11"]
pub type BRR11_R = crate::BitReader;
#[doc = "Field `BRR11` writer - desc BRR11"]
pub type BRR11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR12` reader - desc BRR12"]
pub type BRR12_R = crate::BitReader;
#[doc = "Field `BRR12` writer - desc BRR12"]
pub type BRR12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR13` reader - desc BRR13"]
pub type BRR13_R = crate::BitReader;
#[doc = "Field `BRR13` writer - desc BRR13"]
pub type BRR13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR14` reader - desc BRR14"]
pub type BRR14_R = crate::BitReader;
#[doc = "Field `BRR14` writer - desc BRR14"]
pub type BRR14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRR15` reader - desc BRR15"]
pub type BRR15_R = crate::BitReader;
#[doc = "Field `BRR15` writer - desc BRR15"]
pub type BRR15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 2 - desc BRR2"]
    #[inline(always)]
    pub fn brr2(&self) -> BRR2_R {
        BRR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc BRR3"]
    #[inline(always)]
    pub fn brr3(&self) -> BRR3_R {
        BRR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc BRR4"]
    #[inline(always)]
    pub fn brr4(&self) -> BRR4_R {
        BRR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BRR5"]
    #[inline(always)]
    pub fn brr5(&self) -> BRR5_R {
        BRR5_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 8 - desc BRR8"]
    #[inline(always)]
    pub fn brr8(&self) -> BRR8_R {
        BRR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc BRR9"]
    #[inline(always)]
    pub fn brr9(&self) -> BRR9_R {
        BRR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BRR10"]
    #[inline(always)]
    pub fn brr10(&self) -> BRR10_R {
        BRR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc BRR11"]
    #[inline(always)]
    pub fn brr11(&self) -> BRR11_R {
        BRR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc BRR12"]
    #[inline(always)]
    pub fn brr12(&self) -> BRR12_R {
        BRR12_R::new(((self.bits >> 12) & 1) != 0)
    }
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
    #[doc = "Bit 2 - desc BRR2"]
    #[inline(always)]
    #[must_use]
    pub fn brr2(&mut self) -> BRR2_W<BRR_SPEC, 2> {
        BRR2_W::new(self)
    }
    #[doc = "Bit 3 - desc BRR3"]
    #[inline(always)]
    #[must_use]
    pub fn brr3(&mut self) -> BRR3_W<BRR_SPEC, 3> {
        BRR3_W::new(self)
    }
    #[doc = "Bit 4 - desc BRR4"]
    #[inline(always)]
    #[must_use]
    pub fn brr4(&mut self) -> BRR4_W<BRR_SPEC, 4> {
        BRR4_W::new(self)
    }
    #[doc = "Bit 5 - desc BRR5"]
    #[inline(always)]
    #[must_use]
    pub fn brr5(&mut self) -> BRR5_W<BRR_SPEC, 5> {
        BRR5_W::new(self)
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
    #[doc = "Bit 8 - desc BRR8"]
    #[inline(always)]
    #[must_use]
    pub fn brr8(&mut self) -> BRR8_W<BRR_SPEC, 8> {
        BRR8_W::new(self)
    }
    #[doc = "Bit 9 - desc BRR9"]
    #[inline(always)]
    #[must_use]
    pub fn brr9(&mut self) -> BRR9_W<BRR_SPEC, 9> {
        BRR9_W::new(self)
    }
    #[doc = "Bit 10 - desc BRR10"]
    #[inline(always)]
    #[must_use]
    pub fn brr10(&mut self) -> BRR10_W<BRR_SPEC, 10> {
        BRR10_W::new(self)
    }
    #[doc = "Bit 11 - desc BRR11"]
    #[inline(always)]
    #[must_use]
    pub fn brr11(&mut self) -> BRR11_W<BRR_SPEC, 11> {
        BRR11_W::new(self)
    }
    #[doc = "Bit 12 - desc BRR12"]
    #[inline(always)]
    #[must_use]
    pub fn brr12(&mut self) -> BRR12_W<BRR_SPEC, 12> {
        BRR12_W::new(self)
    }
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
