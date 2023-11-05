#[doc = "Register `MATCH` reader"]
pub type R = crate::svd::R<MATCH_SPEC>;
#[doc = "Field `ADDR0` reader - desc ADDR0"]
pub type ADDR0_R = crate::svd::BitReader;
#[doc = "Field `ADDR1` reader - desc ADDR1"]
pub type ADDR1_R = crate::svd::BitReader;
#[doc = "Field `ADDR2` reader - desc ADDR2"]
pub type ADDR2_R = crate::svd::BitReader;
impl R {
    #[doc = "Bit 0 - desc ADDR0"]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ADDR1"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ADDR2"]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Slave Addrress match flag\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`match_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MATCH_SPEC;
impl crate::svd::RegisterSpec for MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`match_::R`](R) reader structure"]
impl crate::svd::Readable for MATCH_SPEC {}
#[doc = "`reset()` method sets MATCH to value 0"]
impl crate::svd::Resettable for MATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
