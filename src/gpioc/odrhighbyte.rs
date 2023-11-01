#[doc = "Register `ODRHIGHBYTE` reader"]
pub type R = crate::R<ODRHIGHBYTE_SPEC>;
#[doc = "Register `ODRHIGHBYTE` writer"]
pub type W = crate::W<ODRHIGHBYTE_SPEC>;
#[doc = "Field `HIGHBYTE` reader - desc HIGHBYTE"]
pub type HIGHBYTE_R = crate::FieldReader;
#[doc = "Field `HIGHBYTE` writer - desc HIGHBYTE"]
pub type HIGHBYTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - desc HIGHBYTE"]
    #[inline(always)]
    pub fn highbyte(&self) -> HIGHBYTE_R {
        HIGHBYTE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc HIGHBYTE"]
    #[inline(always)]
    #[must_use]
    pub fn highbyte(&mut self) -> HIGHBYTE_W<ODRHIGHBYTE_SPEC, 0> {
        HIGHBYTE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "desc ODRHIGHBYTE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odrhighbyte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odrhighbyte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODRHIGHBYTE_SPEC;
impl crate::RegisterSpec for ODRHIGHBYTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`odrhighbyte::R`](R) reader structure"]
impl crate::Readable for ODRHIGHBYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`odrhighbyte::W`](W) writer structure"]
impl crate::Writable for ODRHIGHBYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ODRHIGHBYTE to value 0"]
impl crate::Resettable for ODRHIGHBYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
