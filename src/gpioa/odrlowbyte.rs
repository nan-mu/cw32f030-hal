#[doc = "Register `ODRLOWBYTE` reader"]
pub type R = crate::R<ODRLOWBYTE_SPEC>;
#[doc = "Register `ODRLOWBYTE` writer"]
pub type W = crate::W<ODRLOWBYTE_SPEC>;
#[doc = "Field `LOWBYTE` reader - desc LOWBYTE"]
pub type LOWBYTE_R = crate::FieldReader;
#[doc = "Field `LOWBYTE` writer - desc LOWBYTE"]
pub type LOWBYTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - desc LOWBYTE"]
    #[inline(always)]
    pub fn lowbyte(&self) -> LOWBYTE_R {
        LOWBYTE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc LOWBYTE"]
    #[inline(always)]
    #[must_use]
    pub fn lowbyte(&mut self) -> LOWBYTE_W<ODRLOWBYTE_SPEC, 0> {
        LOWBYTE_W::new(self)
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
#[doc = "desc ODRLOWBYTE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odrlowbyte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odrlowbyte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODRLOWBYTE_SPEC;
impl crate::RegisterSpec for ODRLOWBYTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`odrlowbyte::R`](R) reader structure"]
impl crate::Readable for ODRLOWBYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`odrlowbyte::W`](W) writer structure"]
impl crate::Writable for ODRLOWBYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ODRLOWBYTE to value 0"]
impl crate::Resettable for ODRLOWBYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
