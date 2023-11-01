#[doc = "Register `SRCADDR3` reader"]
pub type R = crate::R<SRCADDR3_SPEC>;
#[doc = "Register `SRCADDR3` writer"]
pub type W = crate::W<SRCADDR3_SPEC>;
#[doc = "Field `SRCADDR` reader - desc SRCADDR"]
pub type SRCADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SRCADDR` writer - desc SRCADDR"]
pub type SRCADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - desc SRCADDR"]
    #[inline(always)]
    pub fn srcaddr(&self) -> SRCADDR_R {
        SRCADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc SRCADDR"]
    #[inline(always)]
    #[must_use]
    pub fn srcaddr(&mut self) -> SRCADDR_W<SRCADDR3_SPEC, 0> {
        SRCADDR_W::new(self)
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
#[doc = "Channel3 source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcaddr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcaddr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCADDR3_SPEC;
impl crate::RegisterSpec for SRCADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcaddr3::R`](R) reader structure"]
impl crate::Readable for SRCADDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcaddr3::W`](W) writer structure"]
impl crate::Writable for SRCADDR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCADDR3 to value 0"]
impl crate::Resettable for SRCADDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
