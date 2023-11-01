#[doc = "Register `DSTADDR3` reader"]
pub type R = crate::R<DSTADDR3_SPEC>;
#[doc = "Register `DSTADDR3` writer"]
pub type W = crate::W<DSTADDR3_SPEC>;
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
    pub fn dstaddr(&mut self) -> DSTADDR_W<DSTADDR3_SPEC, 0> {
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
#[doc = "Channel3 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstaddr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstaddr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTADDR3_SPEC;
impl crate::RegisterSpec for DSTADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstaddr3::R`](R) reader structure"]
impl crate::Readable for DSTADDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dstaddr3::W`](W) writer structure"]
impl crate::Writable for DSTADDR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSTADDR3 to value 0"]
impl crate::Resettable for DSTADDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
