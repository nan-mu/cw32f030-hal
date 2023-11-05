#[doc = "Register `RESULT16` reader"]
pub type R = crate::svd::R<RESULT16_SPEC>;
#[doc = "Field `RESULT16` reader - desc RESULT16"]
pub type RESULT16_R = crate::svd::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc RESULT16"]
    #[inline(always)]
    pub fn result16(&self) -> RESULT16_R {
        RESULT16_R::new(self.bits)
    }
}
#[doc = "Result register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`result16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT16_SPEC;
impl crate::svd::RegisterSpec for RESULT16_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`result16::R`](R) reader structure"]
impl crate::svd::Readable for RESULT16_SPEC {}
#[doc = "`reset()` method sets RESULT16 to value 0"]
impl crate::svd::Resettable for RESULT16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
