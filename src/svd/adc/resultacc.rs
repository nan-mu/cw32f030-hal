#[doc = "Register `RESULTACC` reader"]
pub type R = crate::svd::R<RESULTACC_SPEC>;
#[doc = "Field `RESULT` reader - desc RESULT"]
pub type RESULT_R = crate::svd::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - desc RESULT"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "desc RESULTACC\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`resultacc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULTACC_SPEC;
impl crate::svd::RegisterSpec for RESULTACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resultacc::R`](R) reader structure"]
impl crate::svd::Readable for RESULTACC_SPEC {}
#[doc = "`reset()` method sets RESULTACC to value 0"]
impl crate::svd::Resettable for RESULTACC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
