#[doc = "Register `DSTADDR4` reader"]
pub type R = crate::R<DSTADDR4_SPEC>;
#[doc = "Register `DSTADDR4` writer"]
pub type W = crate::W<DSTADDR4_SPEC>;
#[doc = "Field `DSTADDR` reader - desc DSTADDR"]
pub type DSTADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DSTADDR` writer - desc DSTADDR"]
pub type DSTADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
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
    pub fn dstaddr(&mut self) -> DSTADDR_W<DSTADDR4_SPEC, 0> {
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
#[doc = "Channel4 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstaddr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstaddr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTADDR4_SPEC;
impl crate::RegisterSpec for DSTADDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstaddr4::R`](R) reader structure"]
impl crate::Readable for DSTADDR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dstaddr4::W`](W) writer structure"]
impl crate::Writable for DSTADDR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSTADDR4 to value 0"]
impl crate::Resettable for DSTADDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
