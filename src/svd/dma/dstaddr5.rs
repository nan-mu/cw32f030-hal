#[doc = "Register `DSTADDR5` reader"]
pub type R = crate::svd::R<DSTADDR5_SPEC>;
#[doc = "Register `DSTADDR5` writer"]
pub type W = crate::svd::W<DSTADDR5_SPEC>;
#[doc = "Field `DSTADDR` reader - desc DSTADDR"]
pub type DSTADDR_R = crate::svd::FieldReader<u32>;
#[doc = "Field `DSTADDR` writer - desc DSTADDR"]
pub type DSTADDR_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - desc DSTADDR"]
    #[inline(always)]
    pub fn dstaddr(&self) -> DSTADDR_R {
        DSTADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc DSTADDR"]
    #[inline(always)]
    #[must_use]
    pub fn dstaddr(&mut self) -> DSTADDR_W<DSTADDR5_SPEC, 0> {
        DSTADDR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel5 destination address register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dstaddr5::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dstaddr5::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTADDR5_SPEC;
impl crate::svd::RegisterSpec for DSTADDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstaddr5::R`](R) reader structure"]
impl crate::svd::Readable for DSTADDR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dstaddr5::W`](W) writer structure"]
impl crate::svd::Writable for DSTADDR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSTADDR5 to value 0"]
impl crate::svd::Resettable for DSTADDR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
