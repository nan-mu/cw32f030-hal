#[doc = "Register `RESULT1` reader"]
pub type R = crate::R<RESULT1_SPEC>;
#[doc = "Field `RESULT` reader - desc RESULT"]
pub type RESULT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc RESULT"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc RESULT1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT1_SPEC;
impl crate::RegisterSpec for RESULT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result1::R`](R) reader structure"]
impl crate::Readable for RESULT1_SPEC {}
#[doc = "`reset()` method sets RESULT1 to value 0"]
impl crate::Resettable for RESULT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
