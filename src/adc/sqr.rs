#[doc = "Register `SQR` reader"]
pub type R = crate::R<SQR_SPEC>;
#[doc = "Register `SQR` writer"]
pub type W = crate::W<SQR_SPEC>;
#[doc = "Field `SQR0` reader - desc SQR0"]
pub type SQR0_R = crate::FieldReader;
#[doc = "Field `SQR0` writer - desc SQR0"]
pub type SQR0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SQR1` reader - desc SQR1"]
pub type SQR1_R = crate::FieldReader;
#[doc = "Field `SQR1` writer - desc SQR1"]
pub type SQR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SQR2` reader - desc SQR2"]
pub type SQR2_R = crate::FieldReader;
#[doc = "Field `SQR2` writer - desc SQR2"]
pub type SQR2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SQR3` reader - desc SQR3"]
pub type SQR3_R = crate::FieldReader;
#[doc = "Field `SQR3` writer - desc SQR3"]
pub type SQR3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ENS` reader - desc ENS"]
pub type ENS_R = crate::FieldReader;
#[doc = "Field `ENS` writer - desc ENS"]
pub type ENS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:3 - desc SQR0"]
    #[inline(always)]
    pub fn sqr0(&self) -> SQR0_R {
        SQR0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc SQR1"]
    #[inline(always)]
    pub fn sqr1(&self) -> SQR1_R {
        SQR1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc SQR2"]
    #[inline(always)]
    pub fn sqr2(&self) -> SQR2_R {
        SQR2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc SQR3"]
    #[inline(always)]
    pub fn sqr3(&self) -> SQR3_R {
        SQR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - desc ENS"]
    #[inline(always)]
    pub fn ens(&self) -> ENS_R {
        ENS_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc SQR0"]
    #[inline(always)]
    #[must_use]
    pub fn sqr0(&mut self) -> SQR0_W<SQR_SPEC, 0> {
        SQR0_W::new(self)
    }
    #[doc = "Bits 4:7 - desc SQR1"]
    #[inline(always)]
    #[must_use]
    pub fn sqr1(&mut self) -> SQR1_W<SQR_SPEC, 4> {
        SQR1_W::new(self)
    }
    #[doc = "Bits 8:11 - desc SQR2"]
    #[inline(always)]
    #[must_use]
    pub fn sqr2(&mut self) -> SQR2_W<SQR_SPEC, 8> {
        SQR2_W::new(self)
    }
    #[doc = "Bits 12:15 - desc SQR3"]
    #[inline(always)]
    #[must_use]
    pub fn sqr3(&mut self) -> SQR3_W<SQR_SPEC, 12> {
        SQR3_W::new(self)
    }
    #[doc = "Bits 16:17 - desc ENS"]
    #[inline(always)]
    #[must_use]
    pub fn ens(&mut self) -> ENS_W<SQR_SPEC, 16> {
        ENS_W::new(self)
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
#[doc = "desc SQR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR_SPEC;
impl crate::RegisterSpec for SQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr::R`](R) reader structure"]
impl crate::Readable for SQR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sqr::W`](W) writer structure"]
impl crate::Writable for SQR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SQR to value 0"]
impl crate::Resettable for SQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
