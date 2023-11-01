#[doc = "Register `SPEED` reader"]
pub type R = crate::R<SPEED_SPEC>;
#[doc = "Register `SPEED` writer"]
pub type W = crate::W<SPEED_SPEC>;
#[doc = "Field `PIN0` reader - desc PIN0"]
pub type PIN0_R = crate::BitReader;
#[doc = "Field `PIN0` writer - desc PIN0"]
pub type PIN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN1` reader - desc PIN1"]
pub type PIN1_R = crate::BitReader;
#[doc = "Field `PIN1` writer - desc PIN1"]
pub type PIN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN2` reader - desc PIN2"]
pub type PIN2_R = crate::BitReader;
#[doc = "Field `PIN2` writer - desc PIN2"]
pub type PIN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN3` reader - desc PIN3"]
pub type PIN3_R = crate::BitReader;
#[doc = "Field `PIN3` writer - desc PIN3"]
pub type PIN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN4` reader - desc PIN4"]
pub type PIN4_R = crate::BitReader;
#[doc = "Field `PIN4` writer - desc PIN4"]
pub type PIN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN5` reader - desc PIN5"]
pub type PIN5_R = crate::BitReader;
#[doc = "Field `PIN5` writer - desc PIN5"]
pub type PIN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN6` reader - desc PIN6"]
pub type PIN6_R = crate::BitReader;
#[doc = "Field `PIN6` writer - desc PIN6"]
pub type PIN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN7` reader - desc PIN7"]
pub type PIN7_R = crate::BitReader;
#[doc = "Field `PIN7` writer - desc PIN7"]
pub type PIN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN8` reader - desc PIN8"]
pub type PIN8_R = crate::BitReader;
#[doc = "Field `PIN8` writer - desc PIN8"]
pub type PIN8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN9` reader - desc PIN9"]
pub type PIN9_R = crate::BitReader;
#[doc = "Field `PIN9` writer - desc PIN9"]
pub type PIN9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN10` reader - desc PIN10"]
pub type PIN10_R = crate::BitReader;
#[doc = "Field `PIN10` writer - desc PIN10"]
pub type PIN10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN11` reader - desc PIN11"]
pub type PIN11_R = crate::BitReader;
#[doc = "Field `PIN11` writer - desc PIN11"]
pub type PIN11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN12` reader - desc PIN12"]
pub type PIN12_R = crate::BitReader;
#[doc = "Field `PIN12` writer - desc PIN12"]
pub type PIN12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN13` reader - desc PIN13"]
pub type PIN13_R = crate::BitReader;
#[doc = "Field `PIN13` writer - desc PIN13"]
pub type PIN13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN14` reader - desc PIN14"]
pub type PIN14_R = crate::BitReader;
#[doc = "Field `PIN14` writer - desc PIN14"]
pub type PIN14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIN15` reader - desc PIN15"]
pub type PIN15_R = crate::BitReader;
#[doc = "Field `PIN15` writer - desc PIN15"]
pub type PIN15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc PIN0"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PIN1"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PIN2"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PIN4"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PIN5"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PIN6"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PIN7"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PIN8"]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PIN9"]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PIN10"]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PIN11"]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PIN12"]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PIN13"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PIN14"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PIN15"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PIN0"]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> PIN0_W<SPEED_SPEC, 0> {
        PIN0_W::new(self)
    }
    #[doc = "Bit 1 - desc PIN1"]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> PIN1_W<SPEED_SPEC, 1> {
        PIN1_W::new(self)
    }
    #[doc = "Bit 2 - desc PIN2"]
    #[inline(always)]
    #[must_use]
    pub fn pin2(&mut self) -> PIN2_W<SPEED_SPEC, 2> {
        PIN2_W::new(self)
    }
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> PIN3_W<SPEED_SPEC, 3> {
        PIN3_W::new(self)
    }
    #[doc = "Bit 4 - desc PIN4"]
    #[inline(always)]
    #[must_use]
    pub fn pin4(&mut self) -> PIN4_W<SPEED_SPEC, 4> {
        PIN4_W::new(self)
    }
    #[doc = "Bit 5 - desc PIN5"]
    #[inline(always)]
    #[must_use]
    pub fn pin5(&mut self) -> PIN5_W<SPEED_SPEC, 5> {
        PIN5_W::new(self)
    }
    #[doc = "Bit 6 - desc PIN6"]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> PIN6_W<SPEED_SPEC, 6> {
        PIN6_W::new(self)
    }
    #[doc = "Bit 7 - desc PIN7"]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> PIN7_W<SPEED_SPEC, 7> {
        PIN7_W::new(self)
    }
    #[doc = "Bit 8 - desc PIN8"]
    #[inline(always)]
    #[must_use]
    pub fn pin8(&mut self) -> PIN8_W<SPEED_SPEC, 8> {
        PIN8_W::new(self)
    }
    #[doc = "Bit 9 - desc PIN9"]
    #[inline(always)]
    #[must_use]
    pub fn pin9(&mut self) -> PIN9_W<SPEED_SPEC, 9> {
        PIN9_W::new(self)
    }
    #[doc = "Bit 10 - desc PIN10"]
    #[inline(always)]
    #[must_use]
    pub fn pin10(&mut self) -> PIN10_W<SPEED_SPEC, 10> {
        PIN10_W::new(self)
    }
    #[doc = "Bit 11 - desc PIN11"]
    #[inline(always)]
    #[must_use]
    pub fn pin11(&mut self) -> PIN11_W<SPEED_SPEC, 11> {
        PIN11_W::new(self)
    }
    #[doc = "Bit 12 - desc PIN12"]
    #[inline(always)]
    #[must_use]
    pub fn pin12(&mut self) -> PIN12_W<SPEED_SPEC, 12> {
        PIN12_W::new(self)
    }
    #[doc = "Bit 13 - desc PIN13"]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> PIN13_W<SPEED_SPEC, 13> {
        PIN13_W::new(self)
    }
    #[doc = "Bit 14 - desc PIN14"]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> PIN14_W<SPEED_SPEC, 14> {
        PIN14_W::new(self)
    }
    #[doc = "Bit 15 - desc PIN15"]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> PIN15_W<SPEED_SPEC, 15> {
        PIN15_W::new(self)
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
#[doc = "desc SPEED\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`speed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`speed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPEED_SPEC;
impl crate::RegisterSpec for SPEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`speed::R`](R) reader structure"]
impl crate::Readable for SPEED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`speed::W`](W) writer structure"]
impl crate::Writable for SPEED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPEED to value 0"]
impl crate::Resettable for SPEED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
