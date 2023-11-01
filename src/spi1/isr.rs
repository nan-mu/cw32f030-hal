#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `SSF` reader - desc SSF"]
pub type SSF_R = crate::BitReader;
#[doc = "Field `SSR` reader - desc SSR"]
pub type SSR_R = crate::BitReader;
#[doc = "Field `UD` reader - desc UD"]
pub type UD_R = crate::BitReader;
#[doc = "Field `OV` reader - desc OV"]
pub type OV_R = crate::BitReader;
#[doc = "Field `SSERR` reader - desc SSERR"]
pub type SSERR_R = crate::BitReader;
#[doc = "Field `MODF` reader - desc MODF"]
pub type MODF_R = crate::BitReader;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `SSLVL` reader - desc SSLVL"]
pub type SSLVL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SSF"]
    #[inline(always)]
    pub fn ssf(&self) -> SSF_R {
        SSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SSR"]
    #[inline(always)]
    pub fn ssr(&self) -> SSR_R {
        SSR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc UD"]
    #[inline(always)]
    pub fn ud(&self) -> UD_R {
        UD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SSERR"]
    #[inline(always)]
    pub fn sserr(&self) -> SSERR_R {
        SSERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc MODF"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SSLVL"]
    #[inline(always)]
    pub fn sslvl(&self) -> SSLVL_R {
        SSLVL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
