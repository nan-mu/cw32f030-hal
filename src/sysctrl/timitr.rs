#[doc = "Register `TIMITR` reader"]
pub type R = crate::R<TIMITR_SPEC>;
#[doc = "Register `TIMITR` writer"]
pub type W = crate::W<TIMITR_SPEC>;
#[doc = "Field `ATIMITR` reader - desc ATIMITR"]
pub type ATIMITR_R = crate::FieldReader;
#[doc = "Field `ATIMITR` writer - desc ATIMITR"]
pub type ATIMITR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GTIM1ITR` reader - desc GTIM1ITR"]
pub type GTIM1ITR_R = crate::FieldReader;
#[doc = "Field `GTIM1ITR` writer - desc GTIM1ITR"]
pub type GTIM1ITR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GTIM2ITR` reader - desc GTIM2ITR"]
pub type GTIM2ITR_R = crate::FieldReader;
#[doc = "Field `GTIM2ITR` writer - desc GTIM2ITR"]
pub type GTIM2ITR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GTIM3ITR` reader - desc GTIM3ITR"]
pub type GTIM3ITR_R = crate::FieldReader;
#[doc = "Field `GTIM3ITR` writer - desc GTIM3ITR"]
pub type GTIM3ITR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GTIM4ITR` reader - desc GTIM4ITR"]
pub type GTIM4ITR_R = crate::FieldReader;
#[doc = "Field `GTIM4ITR` writer - desc GTIM4ITR"]
pub type GTIM4ITR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BTIM1ITR` reader - desc BTIM1ITR"]
pub type BTIM1ITR_R = crate::FieldReader;
#[doc = "Field `BTIM1ITR` writer - desc BTIM1ITR"]
pub type BTIM1ITR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BTIM2ITR` reader - desc BTIM2ITR"]
pub type BTIM2ITR_R = crate::FieldReader;
#[doc = "Field `BTIM2ITR` writer - desc BTIM2ITR"]
pub type BTIM2ITR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BTIM3ITR` reader - desc BTIM3ITR"]
pub type BTIM3ITR_R = crate::FieldReader;
#[doc = "Field `BTIM3ITR` writer - desc BTIM3ITR"]
pub type BTIM3ITR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - desc ATIMITR"]
    #[inline(always)]
    pub fn atimitr(&self) -> ATIMITR_R {
        ATIMITR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - desc GTIM1ITR"]
    #[inline(always)]
    pub fn gtim1itr(&self) -> GTIM1ITR_R {
        GTIM1ITR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - desc GTIM2ITR"]
    #[inline(always)]
    pub fn gtim2itr(&self) -> GTIM2ITR_R {
        GTIM2ITR_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - desc GTIM3ITR"]
    #[inline(always)]
    pub fn gtim3itr(&self) -> GTIM3ITR_R {
        GTIM3ITR_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc GTIM4ITR"]
    #[inline(always)]
    pub fn gtim4itr(&self) -> GTIM4ITR_R {
        GTIM4ITR_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - desc BTIM1ITR"]
    #[inline(always)]
    pub fn btim1itr(&self) -> BTIM1ITR_R {
        BTIM1ITR_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - desc BTIM2ITR"]
    #[inline(always)]
    pub fn btim2itr(&self) -> BTIM2ITR_R {
        BTIM2ITR_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - desc BTIM3ITR"]
    #[inline(always)]
    pub fn btim3itr(&self) -> BTIM3ITR_R {
        BTIM3ITR_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc ATIMITR"]
    #[inline(always)]
    #[must_use]
    pub fn atimitr(&mut self) -> ATIMITR_W<TIMITR_SPEC, 0> {
        ATIMITR_W::new(self)
    }
    #[doc = "Bits 3:5 - desc GTIM1ITR"]
    #[inline(always)]
    #[must_use]
    pub fn gtim1itr(&mut self) -> GTIM1ITR_W<TIMITR_SPEC, 3> {
        GTIM1ITR_W::new(self)
    }
    #[doc = "Bits 6:8 - desc GTIM2ITR"]
    #[inline(always)]
    #[must_use]
    pub fn gtim2itr(&mut self) -> GTIM2ITR_W<TIMITR_SPEC, 6> {
        GTIM2ITR_W::new(self)
    }
    #[doc = "Bits 9:11 - desc GTIM3ITR"]
    #[inline(always)]
    #[must_use]
    pub fn gtim3itr(&mut self) -> GTIM3ITR_W<TIMITR_SPEC, 9> {
        GTIM3ITR_W::new(self)
    }
    #[doc = "Bits 12:14 - desc GTIM4ITR"]
    #[inline(always)]
    #[must_use]
    pub fn gtim4itr(&mut self) -> GTIM4ITR_W<TIMITR_SPEC, 12> {
        GTIM4ITR_W::new(self)
    }
    #[doc = "Bits 15:17 - desc BTIM1ITR"]
    #[inline(always)]
    #[must_use]
    pub fn btim1itr(&mut self) -> BTIM1ITR_W<TIMITR_SPEC, 15> {
        BTIM1ITR_W::new(self)
    }
    #[doc = "Bits 18:20 - desc BTIM2ITR"]
    #[inline(always)]
    #[must_use]
    pub fn btim2itr(&mut self) -> BTIM2ITR_W<TIMITR_SPEC, 18> {
        BTIM2ITR_W::new(self)
    }
    #[doc = "Bits 21:23 - desc BTIM3ITR"]
    #[inline(always)]
    #[must_use]
    pub fn btim3itr(&mut self) -> BTIM3ITR_W<TIMITR_SPEC, 21> {
        BTIM3ITR_W::new(self)
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
#[doc = "BTIMx GTIMx ATIM ITR Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timitr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timitr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMITR_SPEC;
impl crate::RegisterSpec for TIMITR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timitr::R`](R) reader structure"]
impl crate::Readable for TIMITR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timitr::W`](W) writer structure"]
impl crate::Writable for TIMITR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMITR to value 0"]
impl crate::Resettable for TIMITR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
