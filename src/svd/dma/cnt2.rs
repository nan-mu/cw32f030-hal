#[doc = "Register `CNT2` reader"]
pub type R = crate::svd::R<CNT2_SPEC>;
#[doc = "Register `CNT2` writer"]
pub type W = crate::svd::W<CNT2_SPEC>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CNT_R = crate::svd::FieldReader<u16>;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CNT_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `REPEAT` reader - desc REPEAT"]
pub type REPEAT_R = crate::svd::FieldReader;
#[doc = "Field `REPEAT` writer - desc REPEAT"]
pub type REPEAT_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:15 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - desc REPEAT"]
    #[inline(always)]
    pub fn repeat(&self) -> REPEAT_R {
        REPEAT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CNT"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CNT2_SPEC, 0> {
        CNT_W::new(self)
    }
    #[doc = "Bits 16:19 - desc REPEAT"]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> REPEAT_W<CNT2_SPEC, 16> {
        REPEAT_W::new(self)
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
#[doc = "Channel2 counter register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cnt2::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cnt2::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT2_SPEC;
impl crate::svd::RegisterSpec for CNT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt2::R`](R) reader structure"]
impl crate::svd::Readable for CNT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnt2::W`](W) writer structure"]
impl crate::svd::Writable for CNT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNT2 to value 0"]
impl crate::svd::Resettable for CNT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
