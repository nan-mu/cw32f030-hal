#[doc = "Register `CNT` reader"]
pub type R = crate::svd::R<CNT_SPEC>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CNT_R = crate::svd::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "counter\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_SPEC;
impl crate::svd::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::svd::Readable for CNT_SPEC {}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::svd::Resettable for CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
