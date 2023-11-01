#[doc = "Register `GTIMETR` reader"]
pub type R = crate::R<GTIMETR_SPEC>;
#[doc = "Register `GTIMETR` writer"]
pub type W = crate::W<GTIMETR_SPEC>;
#[doc = "Field `GTIM1ETR` reader - desc GTIM1ETR"]
pub type GTIM1ETR_R = crate::FieldReader;
#[doc = "Field `GTIM1ETR` writer - desc GTIM1ETR"]
pub type GTIM1ETR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GTIM2ETR` reader - desc GTIM2ETR"]
pub type GTIM2ETR_R = crate::FieldReader;
#[doc = "Field `GTIM2ETR` writer - desc GTIM2ETR"]
pub type GTIM2ETR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GTIM3ETR` reader - desc GTIM3ETR"]
pub type GTIM3ETR_R = crate::FieldReader;
#[doc = "Field `GTIM3ETR` writer - desc GTIM3ETR"]
pub type GTIM3ETR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GTIM4ETR` reader - desc GTIM4ETR"]
pub type GTIM4ETR_R = crate::FieldReader;
#[doc = "Field `GTIM4ETR` writer - desc GTIM4ETR"]
pub type GTIM4ETR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - desc GTIM1ETR"]
    #[inline(always)]
    pub fn gtim1etr(&self) -> GTIM1ETR_R {
        GTIM1ETR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - desc GTIM2ETR"]
    #[inline(always)]
    pub fn gtim2etr(&self) -> GTIM2ETR_R {
        GTIM2ETR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - desc GTIM3ETR"]
    #[inline(always)]
    pub fn gtim3etr(&self) -> GTIM3ETR_R {
        GTIM3ETR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc GTIM4ETR"]
    #[inline(always)]
    pub fn gtim4etr(&self) -> GTIM4ETR_R {
        GTIM4ETR_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc GTIM1ETR"]
    #[inline(always)]
    #[must_use]
    pub fn gtim1etr(&mut self) -> GTIM1ETR_W<GTIMETR_SPEC, 0> {
        GTIM1ETR_W::new(self)
    }
    #[doc = "Bits 4:6 - desc GTIM2ETR"]
    #[inline(always)]
    #[must_use]
    pub fn gtim2etr(&mut self) -> GTIM2ETR_W<GTIMETR_SPEC, 4> {
        GTIM2ETR_W::new(self)
    }
    #[doc = "Bits 8:10 - desc GTIM3ETR"]
    #[inline(always)]
    #[must_use]
    pub fn gtim3etr(&mut self) -> GTIM3ETR_W<GTIMETR_SPEC, 8> {
        GTIM3ETR_W::new(self)
    }
    #[doc = "Bits 12:14 - desc GTIM4ETR"]
    #[inline(always)]
    #[must_use]
    pub fn gtim4etr(&mut self) -> GTIM4ETR_W<GTIMETR_SPEC, 12> {
        GTIM4ETR_W::new(self)
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
#[doc = "GTIM1-4 ETR Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtimetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtimetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTIMETR_SPEC;
impl crate::RegisterSpec for GTIMETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtimetr::R`](R) reader structure"]
impl crate::Readable for GTIMETR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtimetr::W`](W) writer structure"]
impl crate::Writable for GTIMETR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTIMETR to value 0"]
impl crate::Resettable for GTIMETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
