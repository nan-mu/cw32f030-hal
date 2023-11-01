#[doc = "Register `ADDR2` reader"]
pub type R = crate::R<ADDR2_SPEC>;
#[doc = "Register `ADDR2` writer"]
pub type W = crate::W<ADDR2_SPEC>;
#[doc = "Field `ADDR2` reader - desc ADDR2"]
pub type ADDR2_R = crate::FieldReader;
#[doc = "Field `ADDR2` writer - desc ADDR2"]
pub type ADDR2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 1:7 - desc ADDR2"]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - desc ADDR2"]
    #[inline(always)]
    #[must_use]
    pub fn addr2(&mut self) -> ADDR2_W<ADDR2_SPEC, 1> {
        ADDR2_W::new(self)
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
#[doc = "Slave Addrress2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR2_SPEC;
impl crate::RegisterSpec for ADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr2::R`](R) reader structure"]
impl crate::Readable for ADDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr2::W`](W) writer structure"]
impl crate::Writable for ADDR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR2 to value 0"]
impl crate::Resettable for ADDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
