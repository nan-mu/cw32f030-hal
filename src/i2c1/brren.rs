#[doc = "Register `BRREN` reader"]
pub type R = crate::R<BRREN_SPEC>;
#[doc = "Register `BRREN` writer"]
pub type W = crate::W<BRREN_SPEC>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<BRREN_SPEC, 0> {
        EN_W::new(self)
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
#[doc = "desc BRREN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRREN_SPEC;
impl crate::RegisterSpec for BRREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brren::R`](R) reader structure"]
impl crate::Readable for BRREN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brren::W`](W) writer structure"]
impl crate::Writable for BRREN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRREN to value 0"]
impl crate::Resettable for BRREN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
