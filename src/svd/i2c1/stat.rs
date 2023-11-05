#[doc = "Register `STAT` reader"]
pub type R = crate::svd::R<STAT_SPEC>;
#[doc = "Field `STAT` reader - desc STAT"]
pub type STAT_R = crate::svd::FieldReader;
impl R {
    #[doc = "Bits 0:7 - desc STAT"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::svd::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::svd::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::svd::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
