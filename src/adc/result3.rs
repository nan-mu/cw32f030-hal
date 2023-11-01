#[doc = "Register `RESULT3` reader"]
pub type R = crate::R<RESULT3_SPEC>;
#[doc = "Field `RESULT` reader - desc RESULT"]
pub type RESULT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc RESULT"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc RESULT3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT3_SPEC;
impl crate::RegisterSpec for RESULT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result3::R`](R) reader structure"]
impl crate::Readable for RESULT3_SPEC {}
#[doc = "`reset()` method sets RESULT3 to value 0"]
impl crate::Resettable for RESULT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
