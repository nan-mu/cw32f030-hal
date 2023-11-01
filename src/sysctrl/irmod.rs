#[doc = "Register `IRMOD` reader"]
pub type R = crate::R<IRMOD_SPEC>;
#[doc = "Register `IRMOD` writer"]
pub type W = crate::W<IRMOD_SPEC>;
#[doc = "Field `MOD` reader - desc MOD"]
pub type MOD_R = crate::FieldReader;
#[doc = "Field `MOD` writer - desc MOD"]
pub type MOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - desc MOD"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc MOD"]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> MOD_W<IRMOD_SPEC, 0> {
        MOD_W::new(self)
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
#[doc = "IR MOD Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irmod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irmod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRMOD_SPEC;
impl crate::RegisterSpec for IRMOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irmod::R`](R) reader structure"]
impl crate::Readable for IRMOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irmod::W`](W) writer structure"]
impl crate::Writable for IRMOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRMOD to value 0"]
impl crate::Resettable for IRMOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
