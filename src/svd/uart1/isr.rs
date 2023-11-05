#[doc = "Register `ISR` reader"]
pub type R = crate::svd::R<ISR_SPEC>;
#[doc = "Field `TXE` reader - TxBuf empty"]
pub type TXE_R = crate::svd::BitReader;
#[doc = "Field `TC` reader - Transmit complete"]
pub type TC_R = crate::svd::BitReader;
#[doc = "Field `RC` reader - Receive complete"]
pub type RC_R = crate::svd::BitReader;
#[doc = "Field `FE` reader - Frame error"]
pub type FE_R = crate::svd::BitReader;
#[doc = "Field `PE` reader - Parity error"]
pub type PE_R = crate::svd::BitReader;
#[doc = "Field `MATCH` reader - Slave addr match"]
pub type MATCH_R = crate::svd::BitReader;
#[doc = "Field `CTS` reader - CTS change"]
pub type CTS_R = crate::svd::BitReader;
#[doc = "Field `CTSLV` reader - CTS PIN level"]
pub type CTSLV_R = crate::svd::BitReader;
#[doc = "Field `TXBUSY` reader - desc TXBUSY"]
pub type TXBUSY_R = crate::svd::BitReader;
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
    #[doc = "Bit 5 - Slave addr match"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CTS change"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS PIN level"]
    #[inline(always)]
    pub fn ctslv(&self) -> CTSLV_R {
        CTSLV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc TXBUSY"]
    #[inline(always)]
    pub fn txbusy(&self) -> TXBUSY_R {
        TXBUSY_R::new(((self.bits >> 8) & 1) != 0)
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
