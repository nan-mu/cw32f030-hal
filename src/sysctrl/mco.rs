#[doc = "Register `MCO` reader"]
pub type R = crate::R<MCO_SPEC>;
#[doc = "Register `MCO` writer"]
pub type W = crate::W<MCO_SPEC>;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SOURCE_R = crate::FieldReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SOURCE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DIV_R = crate::FieldReader;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:3 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<MCO_SPEC, 0> {
        SOURCE_W::new(self)
    }
    #[doc = "Bits 4:6 - desc DIV"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<MCO_SPEC, 4> {
        DIV_W::new(self)
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
#[doc = "Master Clock Output Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mco::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mco::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCO_SPEC;
impl crate::RegisterSpec for MCO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mco::R`](R) reader structure"]
impl crate::Readable for MCO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mco::W`](W) writer structure"]
impl crate::Writable for MCO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCO to value 0"]
impl crate::Resettable for MCO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
