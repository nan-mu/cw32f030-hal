#[doc = "Register `ICR` reader"]
pub type R = crate::svd::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::svd::W<ICR_SPEC>;
#[doc = "Field `TC1` reader - desc TC1"]
pub type TC1_R = crate::svd::BitReader;
#[doc = "Field `TC1` writer - desc TC1"]
pub type TC1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TE1` reader - desc TE1"]
pub type TE1_R = crate::svd::BitReader;
#[doc = "Field `TE1` writer - desc TE1"]
pub type TE1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TC2` reader - desc TC2"]
pub type TC2_R = crate::svd::BitReader;
#[doc = "Field `TC2` writer - desc TC2"]
pub type TC2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TE2` reader - desc TE2"]
pub type TE2_R = crate::svd::BitReader;
#[doc = "Field `TE2` writer - desc TE2"]
pub type TE2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TC3` reader - desc TC3"]
pub type TC3_R = crate::svd::BitReader;
#[doc = "Field `TC3` writer - desc TC3"]
pub type TC3_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TE3` reader - desc TE3"]
pub type TE3_R = crate::svd::BitReader;
#[doc = "Field `TE3` writer - desc TE3"]
pub type TE3_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TC4` reader - desc TC4"]
pub type TC4_R = crate::svd::BitReader;
#[doc = "Field `TC4` writer - desc TC4"]
pub type TC4_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TE4` reader - desc TE4"]
pub type TE4_R = crate::svd::BitReader;
#[doc = "Field `TE4` writer - desc TE4"]
pub type TE4_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TC5` reader - desc TC5"]
pub type TC5_R = crate::svd::BitReader;
#[doc = "Field `TC5` writer - desc TC5"]
pub type TC5_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TE5` reader - desc TE5"]
pub type TE5_R = crate::svd::BitReader;
#[doc = "Field `TE5` writer - desc TE5"]
pub type TE5_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc TC1"]
    #[inline(always)]
    pub fn tc1(&self) -> TC1_R {
        TC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TE1"]
    #[inline(always)]
    pub fn te1(&self) -> TE1_R {
        TE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TC2"]
    #[inline(always)]
    pub fn tc2(&self) -> TC2_R {
        TC2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TE2"]
    #[inline(always)]
    pub fn te2(&self) -> TE2_R {
        TE2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc TC3"]
    #[inline(always)]
    pub fn tc3(&self) -> TC3_R {
        TC3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc TE3"]
    #[inline(always)]
    pub fn te3(&self) -> TE3_R {
        TE3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - desc TC4"]
    #[inline(always)]
    pub fn tc4(&self) -> TC4_R {
        TC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc TE4"]
    #[inline(always)]
    pub fn te4(&self) -> TE4_R {
        TE4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - desc TC5"]
    #[inline(always)]
    pub fn tc5(&self) -> TC5_R {
        TC5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc TE5"]
    #[inline(always)]
    pub fn te5(&self) -> TE5_R {
        TE5_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TC1"]
    #[inline(always)]
    #[must_use]
    pub fn tc1(&mut self) -> TC1_W<ICR_SPEC, 0> {
        TC1_W::new(self)
    }
    #[doc = "Bit 1 - desc TE1"]
    #[inline(always)]
    #[must_use]
    pub fn te1(&mut self) -> TE1_W<ICR_SPEC, 1> {
        TE1_W::new(self)
    }
    #[doc = "Bit 4 - desc TC2"]
    #[inline(always)]
    #[must_use]
    pub fn tc2(&mut self) -> TC2_W<ICR_SPEC, 4> {
        TC2_W::new(self)
    }
    #[doc = "Bit 5 - desc TE2"]
    #[inline(always)]
    #[must_use]
    pub fn te2(&mut self) -> TE2_W<ICR_SPEC, 5> {
        TE2_W::new(self)
    }
    #[doc = "Bit 8 - desc TC3"]
    #[inline(always)]
    #[must_use]
    pub fn tc3(&mut self) -> TC3_W<ICR_SPEC, 8> {
        TC3_W::new(self)
    }
    #[doc = "Bit 9 - desc TE3"]
    #[inline(always)]
    #[must_use]
    pub fn te3(&mut self) -> TE3_W<ICR_SPEC, 9> {
        TE3_W::new(self)
    }
    #[doc = "Bit 12 - desc TC4"]
    #[inline(always)]
    #[must_use]
    pub fn tc4(&mut self) -> TC4_W<ICR_SPEC, 12> {
        TC4_W::new(self)
    }
    #[doc = "Bit 13 - desc TE4"]
    #[inline(always)]
    #[must_use]
    pub fn te4(&mut self) -> TE4_W<ICR_SPEC, 13> {
        TE4_W::new(self)
    }
    #[doc = "Bit 16 - desc TC5"]
    #[inline(always)]
    #[must_use]
    pub fn tc5(&mut self) -> TC5_W<ICR_SPEC, 16> {
        TC5_W::new(self)
    }
    #[doc = "Bit 17 - desc TE5"]
    #[inline(always)]
    #[must_use]
    pub fn te5(&mut self) -> TE5_W<ICR_SPEC, 17> {
        TE5_W::new(self)
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
#[doc = "Interrupt flag clear register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::svd::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::svd::Readable for ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::svd::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::svd::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
