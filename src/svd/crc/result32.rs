#[doc = "Register `RESULT32` reader"]
pub type R = crate::svd::R<RESULT32_SPEC>;
#[doc = "Field `RESULT32` reader - desc RESULT32"]
pub type RESULT32_R = crate::svd::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc RESULT32"]
    #[inline(always)]
    pub fn result32(&self) -> RESULT32_R {
        RESULT32_R::new(self.bits)
    }
}
#[doc = "Result register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`result32::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT32_SPEC;
impl crate::svd::RegisterSpec for RESULT32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result32::R`](R) reader structure"]
impl crate::svd::Readable for RESULT32_SPEC {}
#[doc = "`reset()` method sets RESULT32 to value 0"]
impl crate::svd::Resettable for RESULT32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
