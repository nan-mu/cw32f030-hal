#[doc = "Register `RDR` reader"]
pub type R = crate::svd::R<RDR_SPEC>;
#[doc = "Field `RDR` reader - desc RDR"]
pub type RDR_R = crate::svd::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - desc RDR"]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Data reg for read\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDR_SPEC;
impl crate::svd::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::svd::Readable for RDR_SPEC {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::svd::Resettable for RDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
