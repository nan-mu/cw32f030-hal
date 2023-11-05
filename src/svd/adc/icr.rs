#[doc = "Register `ICR` reader"]
pub type R = crate::svd::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::svd::W<ICR_SPEC>;
#[doc = "Field `EOC` reader - desc EOC"]
pub type EOC_R = crate::svd::BitReader;
#[doc = "Field `EOC` writer - desc EOC"]
pub type EOC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `EOS` reader - desc EOS"]
pub type EOS_R = crate::svd::BitReader;
#[doc = "Field `EOS` writer - desc EOS"]
pub type EOS_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `EOA` reader - desc EOA"]
pub type EOA_R = crate::svd::BitReader;
#[doc = "Field `EOA` writer - desc EOA"]
pub type EOA_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `WDTL` reader - desc WDTL"]
pub type WDTL_R = crate::svd::BitReader;
#[doc = "Field `WDTL` writer - desc WDTL"]
pub type WDTL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `WDTH` reader - desc WDTH"]
pub type WDTH_R = crate::svd::BitReader;
#[doc = "Field `WDTH` writer - desc WDTH"]
pub type WDTH_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `WDTR` reader - desc WDTR"]
pub type WDTR_R = crate::svd::BitReader;
#[doc = "Field `WDTR` writer - desc WDTR"]
pub type WDTR_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `OVW` reader - desc OVW"]
pub type OVW_R = crate::svd::BitReader;
#[doc = "Field `OVW` writer - desc OVW"]
pub type OVW_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EOA"]
    #[inline(always)]
    pub fn eoa(&self) -> EOA_R {
        EOA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc WDTL"]
    #[inline(always)]
    pub fn wdtl(&self) -> WDTL_R {
        WDTL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc WDTH"]
    #[inline(always)]
    pub fn wdth(&self) -> WDTH_R {
        WDTH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc WDTR"]
    #[inline(always)]
    pub fn wdtr(&self) -> WDTR_R {
        WDTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc OVW"]
    #[inline(always)]
    pub fn ovw(&self) -> OVW_R {
        OVW_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<ICR_SPEC, 0> {
        EOC_W::new(self)
    }
    #[doc = "Bit 1 - desc EOS"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<ICR_SPEC, 1> {
        EOS_W::new(self)
    }
    #[doc = "Bit 2 - desc EOA"]
    #[inline(always)]
    #[must_use]
    pub fn eoa(&mut self) -> EOA_W<ICR_SPEC, 2> {
        EOA_W::new(self)
    }
    #[doc = "Bit 3 - desc WDTL"]
    #[inline(always)]
    #[must_use]
    pub fn wdtl(&mut self) -> WDTL_W<ICR_SPEC, 3> {
        WDTL_W::new(self)
    }
    #[doc = "Bit 4 - desc WDTH"]
    #[inline(always)]
    #[must_use]
    pub fn wdth(&mut self) -> WDTH_W<ICR_SPEC, 4> {
        WDTH_W::new(self)
    }
    #[doc = "Bit 5 - desc WDTR"]
    #[inline(always)]
    #[must_use]
    pub fn wdtr(&mut self) -> WDTR_W<ICR_SPEC, 5> {
        WDTR_W::new(self)
    }
    #[doc = "Bit 6 - desc OVW"]
    #[inline(always)]
    #[must_use]
    pub fn ovw(&mut self) -> OVW_W<ICR_SPEC, 6> {
        OVW_W::new(self)
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
