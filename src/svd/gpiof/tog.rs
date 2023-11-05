#[doc = "Register `TOG` reader"]
pub type R = crate::svd::R<TOG_SPEC>;
#[doc = "Register `TOG` writer"]
pub type W = crate::svd::W<TOG_SPEC>;
#[doc = "Field `PIN0` reader - desc PIN0"]
pub type PIN0_R = crate::svd::BitReader;
#[doc = "Field `PIN0` writer - desc PIN0"]
pub type PIN0_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PIN1` reader - desc PIN1"]
pub type PIN1_R = crate::svd::BitReader;
#[doc = "Field `PIN1` writer - desc PIN1"]
pub type PIN1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PIN3` reader - desc PIN3"]
pub type PIN3_R = crate::svd::BitReader;
#[doc = "Field `PIN3` writer - desc PIN3"]
pub type PIN3_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PIN6` reader - desc PIN6"]
pub type PIN6_R = crate::svd::BitReader;
#[doc = "Field `PIN6` writer - desc PIN6"]
pub type PIN6_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PIN7` reader - desc PIN7"]
pub type PIN7_R = crate::svd::BitReader;
#[doc = "Field `PIN7` writer - desc PIN7"]
pub type PIN7_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 0 - desc PIN0"]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> PIN0_W<TOG_SPEC, 0> {
        PIN0_W::new(self)
    }
    #[doc = "Bit 1 - desc PIN1"]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> PIN1_W<TOG_SPEC, 1> {
        PIN1_W::new(self)
    }
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> PIN3_W<TOG_SPEC, 3> {
        PIN3_W::new(self)
    }
    #[doc = "Bit 6 - desc PIN6"]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> PIN6_W<TOG_SPEC, 6> {
        PIN6_W::new(self)
    }
    #[doc = "Bit 7 - desc PIN7"]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> PIN7_W<TOG_SPEC, 7> {
        PIN7_W::new(self)
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
#[doc = "desc TOG\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`tog::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`tog::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOG_SPEC;
impl crate::svd::RegisterSpec for TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tog::R`](R) reader structure"]
impl crate::svd::Readable for TOG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tog::W`](W) writer structure"]
impl crate::svd::Writable for TOG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOG to value 0"]
impl crate::svd::Resettable for TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
