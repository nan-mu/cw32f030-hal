#[doc = "Register `FLTR` reader"]
pub type R = crate::R<FLTR_SPEC>;
#[doc = "Register `FLTR` writer"]
pub type W = crate::W<FLTR_SPEC>;
#[doc = "Field `OCM1AFLT1A` reader - desc OCM1AFLT1A"]
pub type OCM1AFLT1A_R = crate::FieldReader;
#[doc = "Field `OCM1AFLT1A` writer - desc OCM1AFLT1A"]
pub type OCM1AFLT1A_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CCP1A` reader - desc CCP1A"]
pub type CCP1A_R = crate::BitReader;
#[doc = "Field `CCP1A` writer - desc CCP1A"]
pub type CCP1A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCM1BFLT1B` reader - desc OCM1BFLT1B"]
pub type OCM1BFLT1B_R = crate::FieldReader;
#[doc = "Field `OCM1BFLT1B` writer - desc OCM1BFLT1B"]
pub type OCM1BFLT1B_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CCP1B` reader - desc CCP1B"]
pub type CCP1B_R = crate::BitReader;
#[doc = "Field `CCP1B` writer - desc CCP1B"]
pub type CCP1B_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCM2AFLT2A` reader - desc OCM2AFLT2A"]
pub type OCM2AFLT2A_R = crate::FieldReader;
#[doc = "Field `OCM2AFLT2A` writer - desc OCM2AFLT2A"]
pub type OCM2AFLT2A_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CCP2A` reader - desc CCP2A"]
pub type CCP2A_R = crate::BitReader;
#[doc = "Field `CCP2A` writer - desc CCP2A"]
pub type CCP2A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCM2BFLT2B` reader - desc OCM2BFLT2B"]
pub type OCM2BFLT2B_R = crate::FieldReader;
#[doc = "Field `OCM2BFLT2B` writer - desc OCM2BFLT2B"]
pub type OCM2BFLT2B_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CCP2B` reader - desc CCP2B"]
pub type CCP2B_R = crate::BitReader;
#[doc = "Field `CCP2B` writer - desc CCP2B"]
pub type CCP2B_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCM3AFLT3A` reader - desc OCM3AFLT3A"]
pub type OCM3AFLT3A_R = crate::FieldReader;
#[doc = "Field `OCM3AFLT3A` writer - desc OCM3AFLT3A"]
pub type OCM3AFLT3A_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CCP3A` reader - desc CCP3A"]
pub type CCP3A_R = crate::BitReader;
#[doc = "Field `CCP3A` writer - desc CCP3A"]
pub type CCP3A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCM3BFLT3B` reader - desc OCM3BFLT3B"]
pub type OCM3BFLT3B_R = crate::FieldReader;
#[doc = "Field `OCM3BFLT3B` writer - desc OCM3BFLT3B"]
pub type OCM3BFLT3B_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CCP3B` reader - desc CCP3B"]
pub type CCP3B_R = crate::BitReader;
#[doc = "Field `CCP3B` writer - desc CCP3B"]
pub type CCP3B_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLTBK` reader - desc FLTBK"]
pub type FLTBK_R = crate::FieldReader;
#[doc = "Field `FLTBK` writer - desc FLTBK"]
pub type FLTBK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BKP` reader - desc BKP"]
pub type BKP_R = crate::BitReader;
#[doc = "Field `BKP` writer - desc BKP"]
pub type BKP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLTET` reader - desc FLTET"]
pub type FLTET_R = crate::FieldReader;
#[doc = "Field `FLTET` writer - desc FLTET"]
pub type FLTET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ETP` reader - desc ETP"]
pub type ETP_R = crate::BitReader;
#[doc = "Field `ETP` writer - desc ETP"]
pub type ETP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - desc OCM1AFLT1A"]
    #[inline(always)]
    pub fn ocm1aflt1a(&self) -> OCM1AFLT1A_R {
        OCM1AFLT1A_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc CCP1A"]
    #[inline(always)]
    pub fn ccp1a(&self) -> CCP1A_R {
        CCP1A_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc OCM1BFLT1B"]
    #[inline(always)]
    pub fn ocm1bflt1b(&self) -> OCM1BFLT1B_R {
        OCM1BFLT1B_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc CCP1B"]
    #[inline(always)]
    pub fn ccp1b(&self) -> CCP1B_R {
        CCP1B_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc OCM2AFLT2A"]
    #[inline(always)]
    pub fn ocm2aflt2a(&self) -> OCM2AFLT2A_R {
        OCM2AFLT2A_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - desc CCP2A"]
    #[inline(always)]
    pub fn ccp2a(&self) -> CCP2A_R {
        CCP2A_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - desc OCM2BFLT2B"]
    #[inline(always)]
    pub fn ocm2bflt2b(&self) -> OCM2BFLT2B_R {
        OCM2BFLT2B_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - desc CCP2B"]
    #[inline(always)]
    pub fn ccp2b(&self) -> CCP2B_R {
        CCP2B_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - desc OCM3AFLT3A"]
    #[inline(always)]
    pub fn ocm3aflt3a(&self) -> OCM3AFLT3A_R {
        OCM3AFLT3A_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - desc CCP3A"]
    #[inline(always)]
    pub fn ccp3a(&self) -> CCP3A_R {
        CCP3A_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - desc OCM3BFLT3B"]
    #[inline(always)]
    pub fn ocm3bflt3b(&self) -> OCM3BFLT3B_R {
        OCM3BFLT3B_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - desc CCP3B"]
    #[inline(always)]
    pub fn ccp3b(&self) -> CCP3B_R {
        CCP3B_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - desc FLTBK"]
    #[inline(always)]
    pub fn fltbk(&self) -> FLTBK_R {
        FLTBK_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - desc BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - desc FLTET"]
    #[inline(always)]
    pub fn fltet(&self) -> FLTET_R {
        FLTET_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - desc ETP"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc OCM1AFLT1A"]
    #[inline(always)]
    #[must_use]
    pub fn ocm1aflt1a(&mut self) -> OCM1AFLT1A_W<FLTR_SPEC, 0> {
        OCM1AFLT1A_W::new(self)
    }
    #[doc = "Bit 3 - desc CCP1A"]
    #[inline(always)]
    #[must_use]
    pub fn ccp1a(&mut self) -> CCP1A_W<FLTR_SPEC, 3> {
        CCP1A_W::new(self)
    }
    #[doc = "Bits 4:6 - desc OCM1BFLT1B"]
    #[inline(always)]
    #[must_use]
    pub fn ocm1bflt1b(&mut self) -> OCM1BFLT1B_W<FLTR_SPEC, 4> {
        OCM1BFLT1B_W::new(self)
    }
    #[doc = "Bit 7 - desc CCP1B"]
    #[inline(always)]
    #[must_use]
    pub fn ccp1b(&mut self) -> CCP1B_W<FLTR_SPEC, 7> {
        CCP1B_W::new(self)
    }
    #[doc = "Bits 8:10 - desc OCM2AFLT2A"]
    #[inline(always)]
    #[must_use]
    pub fn ocm2aflt2a(&mut self) -> OCM2AFLT2A_W<FLTR_SPEC, 8> {
        OCM2AFLT2A_W::new(self)
    }
    #[doc = "Bit 11 - desc CCP2A"]
    #[inline(always)]
    #[must_use]
    pub fn ccp2a(&mut self) -> CCP2A_W<FLTR_SPEC, 11> {
        CCP2A_W::new(self)
    }
    #[doc = "Bits 12:14 - desc OCM2BFLT2B"]
    #[inline(always)]
    #[must_use]
    pub fn ocm2bflt2b(&mut self) -> OCM2BFLT2B_W<FLTR_SPEC, 12> {
        OCM2BFLT2B_W::new(self)
    }
    #[doc = "Bit 15 - desc CCP2B"]
    #[inline(always)]
    #[must_use]
    pub fn ccp2b(&mut self) -> CCP2B_W<FLTR_SPEC, 15> {
        CCP2B_W::new(self)
    }
    #[doc = "Bits 16:18 - desc OCM3AFLT3A"]
    #[inline(always)]
    #[must_use]
    pub fn ocm3aflt3a(&mut self) -> OCM3AFLT3A_W<FLTR_SPEC, 16> {
        OCM3AFLT3A_W::new(self)
    }
    #[doc = "Bit 19 - desc CCP3A"]
    #[inline(always)]
    #[must_use]
    pub fn ccp3a(&mut self) -> CCP3A_W<FLTR_SPEC, 19> {
        CCP3A_W::new(self)
    }
    #[doc = "Bits 20:22 - desc OCM3BFLT3B"]
    #[inline(always)]
    #[must_use]
    pub fn ocm3bflt3b(&mut self) -> OCM3BFLT3B_W<FLTR_SPEC, 20> {
        OCM3BFLT3B_W::new(self)
    }
    #[doc = "Bit 23 - desc CCP3B"]
    #[inline(always)]
    #[must_use]
    pub fn ccp3b(&mut self) -> CCP3B_W<FLTR_SPEC, 23> {
        CCP3B_W::new(self)
    }
    #[doc = "Bits 24:26 - desc FLTBK"]
    #[inline(always)]
    #[must_use]
    pub fn fltbk(&mut self) -> FLTBK_W<FLTR_SPEC, 24> {
        FLTBK_W::new(self)
    }
    #[doc = "Bit 27 - desc BKP"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<FLTR_SPEC, 27> {
        BKP_W::new(self)
    }
    #[doc = "Bits 28:30 - desc FLTET"]
    #[inline(always)]
    #[must_use]
    pub fn fltet(&mut self) -> FLTET_W<FLTR_SPEC, 28> {
        FLTET_W::new(self)
    }
    #[doc = "Bit 31 - desc ETP"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<FLTR_SPEC, 31> {
        ETP_W::new(self)
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
#[doc = "desc FLTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTR_SPEC;
impl crate::RegisterSpec for FLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltr::R`](R) reader structure"]
impl crate::Readable for FLTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fltr::W`](W) writer structure"]
impl crate::Writable for FLTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTR to value 0"]
impl crate::Resettable for FLTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
