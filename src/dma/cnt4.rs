#[doc = "Register `CNT4` reader"]
pub type R = crate::R<CNT4_SPEC>;
#[doc = "Register `CNT4` writer"]
pub type W = crate::W<CNT4_SPEC>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `REPEAT` reader - desc REPEAT"]
pub type REPEAT_R = crate::FieldReader;
#[doc = "Field `REPEAT` writer - desc REPEAT"]
pub type REPEAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
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
    pub fn cnt(&mut self) -> CNT_W<CNT4_SPEC, 0> {
        CNT_W::new(self)
    }
    #[doc = "Bits 16:19 - desc REPEAT"]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> REPEAT_W<CNT4_SPEC, 16> {
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
#[doc = "Channel4 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT4_SPEC;
impl crate::RegisterSpec for CNT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt4::R`](R) reader structure"]
impl crate::Readable for CNT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnt4::W`](W) writer structure"]
impl crate::Writable for CNT4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNT4 to value 0"]
impl crate::Resettable for CNT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
