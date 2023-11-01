#[doc = "Register `AWTARR` reader"]
pub type R = crate::R<AWTARR_SPEC>;
#[doc = "Register `AWTARR` writer"]
pub type W = crate::W<AWTARR_SPEC>;
#[doc = "Field `ARR` reader - desc ARR"]
pub type ARR_R = crate::FieldReader<u16>;
#[doc = "Field `ARR` writer - desc ARR"]
pub type ARR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - desc ARR"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc ARR"]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<AWTARR_SPEC, 0> {
        ARR_W::new(self)
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
#[doc = "Auto Wakeup Timer Auto Reload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awtarr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awtarr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWTARR_SPEC;
impl crate::RegisterSpec for AWTARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awtarr::R`](R) reader structure"]
impl crate::Readable for AWTARR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awtarr::W`](W) writer structure"]
impl crate::Writable for AWTARR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWTARR to value 0"]
impl crate::Resettable for AWTARR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
