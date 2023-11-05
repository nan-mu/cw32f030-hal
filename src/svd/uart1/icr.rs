#[doc = "Register `ICR` reader"]
pub type R = crate::svd::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::svd::W<ICR_SPEC>;
#[doc = "Field `TC` reader - Transmit complete"]
pub type TC_R = crate::svd::BitReader;
#[doc = "Field `TC` writer - Transmit complete"]
pub type TC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `RC` reader - Receive complete"]
pub type RC_R = crate::svd::BitReader;
#[doc = "Field `RC` writer - Receive complete"]
pub type RC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `FE` reader - Frame error"]
pub type FE_R = crate::svd::BitReader;
#[doc = "Field `FE` writer - Frame error"]
pub type FE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PE` reader - Parity error"]
pub type PE_R = crate::svd::BitReader;
#[doc = "Field `PE` writer - Parity error"]
pub type PE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CTS` reader - CTS change"]
pub type CTS_R = crate::svd::BitReader;
#[doc = "Field `CTS` writer - CTS change"]
pub type CTS_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Transmit complete"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive complete"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame error"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CTS change"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transmit complete"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<ICR_SPEC, 1> {
        TC_W::new(self)
    }
    #[doc = "Bit 2 - Receive complete"]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RC_W<ICR_SPEC, 2> {
        RC_W::new(self)
    }
    #[doc = "Bit 3 - Frame error"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<ICR_SPEC, 3> {
        FE_W::new(self)
    }
    #[doc = "Bit 4 - Parity error"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<ICR_SPEC, 4> {
        PE_W::new(self)
    }
    #[doc = "Bit 6 - CTS change"]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CTS_W<ICR_SPEC, 6> {
        CTS_W::new(self)
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
