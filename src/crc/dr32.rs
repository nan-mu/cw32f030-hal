#[doc = "Register `DR32` reader"]
pub type R = crate::R<DR32_SPEC>;
#[doc = "Register `DR32` writer"]
pub type W = crate::W<DR32_SPEC>;
#[doc = "Field `DR32` reader - desc DR32"]
pub type DR32_R = crate::FieldReader<u32>;
#[doc = "Field `DR32` writer - desc DR32"]
pub type DR32_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - desc DR32"]
    #[inline(always)]
    pub fn dr32(&self) -> DR32_R {
        DR32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc DR32"]
    #[inline(always)]
    #[must_use]
    pub fn dr32(&mut self) -> DR32_W<DR32_SPEC, 0> {
        DR32_W::new(self)
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
#[doc = "Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR32_SPEC;
impl crate::RegisterSpec for DR32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr32::R`](R) reader structure"]
impl crate::Readable for DR32_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr32::W`](W) writer structure"]
impl crate::Writable for DR32_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR32 to value 0"]
impl crate::Resettable for DR32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
