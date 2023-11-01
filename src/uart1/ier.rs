#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `TXE` reader - TxBuf empty"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `TXE` writer - TxBuf empty"]
pub type TXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC` reader - Transmit complete"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TC` writer - Transmit complete"]
pub type TC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RC` reader - Receive complete"]
pub type RC_R = crate::BitReader;
#[doc = "Field `RC` writer - Receive complete"]
pub type RC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FE` reader - Frame error"]
pub type FE_R = crate::BitReader;
#[doc = "Field `FE` writer - Frame error"]
pub type FE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PE` reader - Parity error"]
pub type PE_R = crate::BitReader;
#[doc = "Field `PE` writer - Parity error"]
pub type PE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTS` reader - CTS change"]
pub type CTS_R = crate::BitReader;
#[doc = "Field `CTS` writer - CTS change"]
pub type CTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TxBuf empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
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
    #[doc = "Bit 0 - TxBuf empty"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<IER_SPEC, 0> {
        TXE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit complete"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<IER_SPEC, 1> {
        TC_W::new(self)
    }
    #[doc = "Bit 2 - Receive complete"]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RC_W<IER_SPEC, 2> {
        RC_W::new(self)
    }
    #[doc = "Bit 3 - Frame error"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<IER_SPEC, 3> {
        FE_W::new(self)
    }
    #[doc = "Bit 4 - Parity error"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<IER_SPEC, 4> {
        PE_W::new(self)
    }
    #[doc = "Bit 6 - CTS change"]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CTS_W<IER_SPEC, 6> {
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
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
