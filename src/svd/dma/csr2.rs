#[doc = "Register `CSR2` reader"]
pub type R = crate::svd::R<CSR2_SPEC>;
#[doc = "Register `CSR2` writer"]
pub type W = crate::svd::W<CSR2_SPEC>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::svd::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TCIE` reader - desc TCIE"]
pub type TCIE_R = crate::svd::BitReader;
#[doc = "Field `TCIE` writer - desc TCIE"]
pub type TCIE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TEIE` reader - desc TEIE"]
pub type TEIE_R = crate::svd::BitReader;
#[doc = "Field `TEIE` writer - desc TEIE"]
pub type TEIE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TRANS` reader - desc TRANS"]
pub type TRANS_R = crate::svd::BitReader;
#[doc = "Field `TRANS` writer - desc TRANS"]
pub type TRANS_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SRCINC` reader - desc SRCINC"]
pub type SRCINC_R = crate::svd::BitReader;
#[doc = "Field `SRCINC` writer - desc SRCINC"]
pub type SRCINC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `DSTINC` reader - desc DSTINC"]
pub type DSTINC_R = crate::svd::BitReader;
#[doc = "Field `DSTINC` writer - desc DSTINC"]
pub type DSTINC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SIZE` reader - desc SIZE"]
pub type SIZE_R = crate::svd::FieldReader;
#[doc = "Field `SIZE` writer - desc SIZE"]
pub type SIZE_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `STATUS` reader - desc STATUS"]
pub type STATUS_R = crate::svd::FieldReader;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TRANS"]
    #[inline(always)]
    pub fn trans(&self) -> TRANS_R {
        TRANS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SRCINC"]
    #[inline(always)]
    pub fn srcinc(&self) -> SRCINC_R {
        SRCINC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DSTINC"]
    #[inline(always)]
    pub fn dstinc(&self) -> DSTINC_R {
        DSTINC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc SIZE"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - desc STATUS"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CSR2_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - desc TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CSR2_SPEC, 1> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 2 - desc TEIE"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<CSR2_SPEC, 2> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 3 - desc TRANS"]
    #[inline(always)]
    #[must_use]
    pub fn trans(&mut self) -> TRANS_W<CSR2_SPEC, 3> {
        TRANS_W::new(self)
    }
    #[doc = "Bit 4 - desc SRCINC"]
    #[inline(always)]
    #[must_use]
    pub fn srcinc(&mut self) -> SRCINC_W<CSR2_SPEC, 4> {
        SRCINC_W::new(self)
    }
    #[doc = "Bit 5 - desc DSTINC"]
    #[inline(always)]
    #[must_use]
    pub fn dstinc(&mut self) -> DSTINC_W<CSR2_SPEC, 5> {
        DSTINC_W::new(self)
    }
    #[doc = "Bits 6:7 - desc SIZE"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<CSR2_SPEC, 6> {
        SIZE_W::new(self)
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
#[doc = "Channel2 control and status register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`csr2::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`csr2::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR2_SPEC;
impl crate::svd::RegisterSpec for CSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr2::R`](R) reader structure"]
impl crate::svd::Readable for CSR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr2::W`](W) writer structure"]
impl crate::svd::Writable for CSR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR2 to value 0"]
impl crate::svd::Resettable for CSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
