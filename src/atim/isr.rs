#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `UIF` reader - desc UIF"]
pub type UIF_R = crate::BitReader;
#[doc = "Field `C1AF` reader - desc C1AF"]
pub type C1AF_R = crate::BitReader;
#[doc = "Field `C2AF` reader - desc C2AF"]
pub type C2AF_R = crate::BitReader;
#[doc = "Field `C3AF` reader - desc C3AF"]
pub type C3AF_R = crate::BitReader;
#[doc = "Field `C1BF` reader - desc C1BF"]
pub type C1BF_R = crate::BitReader;
#[doc = "Field `C2BF` reader - desc C2BF"]
pub type C2BF_R = crate::BitReader;
#[doc = "Field `C3BF` reader - desc C3BF"]
pub type C3BF_R = crate::BitReader;
#[doc = "Field `C1AE` reader - desc C1AE"]
pub type C1AE_R = crate::BitReader;
#[doc = "Field `C2AE` reader - desc C2AE"]
pub type C2AE_R = crate::BitReader;
#[doc = "Field `C3AE` reader - desc C3AE"]
pub type C3AE_R = crate::BitReader;
#[doc = "Field `C1BE` reader - desc C1BE"]
pub type C1BE_R = crate::BitReader;
#[doc = "Field `C2BE` reader - desc C2BE"]
pub type C2BE_R = crate::BitReader;
#[doc = "Field `C3BE` reader - desc C3BE"]
pub type C3BE_R = crate::BitReader;
#[doc = "Field `BIF` reader - desc BIF"]
pub type BIF_R = crate::BitReader;
#[doc = "Field `TIF` reader - desc TIF"]
pub type TIF_R = crate::BitReader;
#[doc = "Field `OVF` reader - desc OVF"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `UNDF` reader - desc UNDF"]
pub type UNDF_R = crate::BitReader;
#[doc = "Field `C4AF` reader - desc C4AF"]
pub type C4AF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc UIF"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc C1AF"]
    #[inline(always)]
    pub fn c1af(&self) -> C1AF_R {
        C1AF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc C2AF"]
    #[inline(always)]
    pub fn c2af(&self) -> C2AF_R {
        C2AF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc C3AF"]
    #[inline(always)]
    pub fn c3af(&self) -> C3AF_R {
        C3AF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc C1BF"]
    #[inline(always)]
    pub fn c1bf(&self) -> C1BF_R {
        C1BF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc C2BF"]
    #[inline(always)]
    pub fn c2bf(&self) -> C2BF_R {
        C2BF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc C3BF"]
    #[inline(always)]
    pub fn c3bf(&self) -> C3BF_R {
        C3BF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc C1AE"]
    #[inline(always)]
    pub fn c1ae(&self) -> C1AE_R {
        C1AE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc C2AE"]
    #[inline(always)]
    pub fn c2ae(&self) -> C2AE_R {
        C2AE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc C3AE"]
    #[inline(always)]
    pub fn c3ae(&self) -> C3AE_R {
        C3AE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc C1BE"]
    #[inline(always)]
    pub fn c1be(&self) -> C1BE_R {
        C1BE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc C2BE"]
    #[inline(always)]
    pub fn c2be(&self) -> C2BE_R {
        C2BE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc C3BE"]
    #[inline(always)]
    pub fn c3be(&self) -> C3BE_R {
        C3BE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc BIF"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TIF"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc OVF"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc UNDF"]
    #[inline(always)]
    pub fn undf(&self) -> UNDF_R {
        UNDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc C4AF"]
    #[inline(always)]
    pub fn c4af(&self) -> C4AF_R {
        C4AF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "desc ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
