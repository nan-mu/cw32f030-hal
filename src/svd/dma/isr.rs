#[doc = "Register `ISR` reader"]
pub type R = crate::svd::R<ISR_SPEC>;
#[doc = "Field `TC1` reader - desc TC1"]
pub type TC1_R = crate::svd::BitReader;
#[doc = "Field `TE1` reader - desc TE1"]
pub type TE1_R = crate::svd::BitReader;
#[doc = "Field `TC2` reader - desc TC2"]
pub type TC2_R = crate::svd::BitReader;
#[doc = "Field `TE2` reader - desc TE2"]
pub type TE2_R = crate::svd::BitReader;
#[doc = "Field `TC3` reader - desc TC3"]
pub type TC3_R = crate::svd::BitReader;
#[doc = "Field `TE3` reader - desc TE3"]
pub type TE3_R = crate::svd::BitReader;
#[doc = "Field `TC4` reader - desc TC4"]
pub type TC4_R = crate::svd::BitReader;
#[doc = "Field `TE4` reader - desc TE4"]
pub type TE4_R = crate::svd::BitReader;
#[doc = "Field `TC5` reader - desc TC5"]
pub type TC5_R = crate::svd::BitReader;
#[doc = "Field `TE5` reader - desc TE5"]
pub type TE5_R = crate::svd::BitReader;
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
#[doc = "Interrupt status register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::svd::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::svd::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::svd::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
