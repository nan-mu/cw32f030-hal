#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `HSIRDY` reader - desc HSIRDY"]
pub type HSIRDY_R = crate::BitReader;
#[doc = "Field `HSERDY` reader - desc HSERDY"]
pub type HSERDY_R = crate::BitReader;
#[doc = "Field `PLLRDY` reader - desc PLLRDY"]
pub type PLLRDY_R = crate::BitReader;
#[doc = "Field `LSIRDY` reader - desc LSIRDY"]
pub type LSIRDY_R = crate::BitReader;
#[doc = "Field `LSERDY` reader - desc LSERDY"]
pub type LSERDY_R = crate::BitReader;
#[doc = "Field `LSEFAIL` reader - desc LSEFAIL"]
pub type LSEFAIL_R = crate::BitReader;
#[doc = "Field `HSEFAIL` reader - desc HSEFAIL"]
pub type HSEFAIL_R = crate::BitReader;
#[doc = "Field `LSEFAULT` reader - desc LSEFAULT"]
pub type LSEFAULT_R = crate::BitReader;
#[doc = "Field `HSEFAULT` reader - desc HSEFAULT"]
pub type HSEFAULT_R = crate::BitReader;
#[doc = "Field `HSISTABLE` reader - desc HSISTABLE"]
pub type HSISTABLE_R = crate::BitReader;
#[doc = "Field `HSESTABLE` reader - desc HSESTABLE"]
pub type HSESTABLE_R = crate::BitReader;
#[doc = "Field `PLLSTABLE` reader - desc PLLSTABLE"]
pub type PLLSTABLE_R = crate::BitReader;
#[doc = "Field `LSISTABLE` reader - desc LSISTABLE"]
pub type LSISTABLE_R = crate::BitReader;
#[doc = "Field `LSESTABLE` reader - desc LSESTABLE"]
pub type LSESTABLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSERDY"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PLLRDY"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc LSERDY"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc LSEFAIL"]
    #[inline(always)]
    pub fn lsefail(&self) -> LSEFAIL_R {
        LSEFAIL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HSEFAIL"]
    #[inline(always)]
    pub fn hsefail(&self) -> HSEFAIL_R {
        HSEFAIL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc LSEFAULT"]
    #[inline(always)]
    pub fn lsefault(&self) -> LSEFAULT_R {
        LSEFAULT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HSEFAULT"]
    #[inline(always)]
    pub fn hsefault(&self) -> HSEFAULT_R {
        HSEFAULT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HSISTABLE"]
    #[inline(always)]
    pub fn hsistable(&self) -> HSISTABLE_R {
        HSISTABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HSESTABLE"]
    #[inline(always)]
    pub fn hsestable(&self) -> HSESTABLE_R {
        HSESTABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PLLSTABLE"]
    #[inline(always)]
    pub fn pllstable(&self) -> PLLSTABLE_R {
        PLLSTABLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc LSISTABLE"]
    #[inline(always)]
    pub fn lsistable(&self) -> LSISTABLE_R {
        LSISTABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc LSESTABLE"]
    #[inline(always)]
    pub fn lsestable(&self) -> LSESTABLE_R {
        LSESTABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interupt Status Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
