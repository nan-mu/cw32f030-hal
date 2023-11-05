#[doc = "Register `ADDR` reader"]
pub type R = crate::svd::R<ADDR_SPEC>;
#[doc = "Field `ADDR` reader - desc ADDR"]
pub type ADDR_R = crate::svd::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "Parity check error addr register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_SPEC;
impl crate::svd::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::svd::Readable for ADDR_SPEC {}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::svd::Resettable for ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
