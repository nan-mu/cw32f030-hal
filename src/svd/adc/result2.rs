#[doc = "Register `RESULT2` reader"]
pub type R = crate::svd::R<RESULT2_SPEC>;
#[doc = "Field `RESULT` reader - desc RESULT"]
pub type RESULT_R = crate::svd::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc RESULT"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc RESULT2\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`result2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT2_SPEC;
impl crate::svd::RegisterSpec for RESULT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result2::R`](R) reader structure"]
impl crate::svd::Readable for RESULT2_SPEC {}
#[doc = "`reset()` method sets RESULT2 to value 0"]
impl crate::svd::Resettable for RESULT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
