#[doc = "Register `TRIG5` reader"]
pub type R = crate::R<TRIG5_SPEC>;
#[doc = "Register `TRIG5` writer"]
pub type W = crate::W<TRIG5_SPEC>;
#[doc = "Field `TYPE` reader - desc TYPE"]
pub type TYPE_R = crate::BitReader;
#[doc = "Field `TYPE` writer - desc TYPE"]
pub type TYPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTSRC` reader - desc SOFTSRC"]
pub type SOFTSRC_R = crate::BitReader;
#[doc = "Field `SOFTSRC` writer - desc SOFTSRC"]
pub type SOFTSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HARDSRC` reader - desc HARDSRC"]
pub type HARDSRC_R = crate::FieldReader;
#[doc = "Field `HARDSRC` writer - desc HARDSRC"]
pub type HARDSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bit 0 - desc TYPE"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SOFTSRC"]
    #[inline(always)]
    pub fn softsrc(&self) -> SOFTSRC_R {
        SOFTSRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - desc HARDSRC"]
    #[inline(always)]
    pub fn hardsrc(&self) -> HARDSRC_R {
        HARDSRC_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc TYPE"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<TRIG5_SPEC, 0> {
        TYPE_W::new(self)
    }
    #[doc = "Bit 1 - desc SOFTSRC"]
    #[inline(always)]
    #[must_use]
    pub fn softsrc(&mut self) -> SOFTSRC_W<TRIG5_SPEC, 1> {
        SOFTSRC_W::new(self)
    }
    #[doc = "Bits 2:7 - desc HARDSRC"]
    #[inline(always)]
    #[must_use]
    pub fn hardsrc(&mut self) -> HARDSRC_W<TRIG5_SPEC, 2> {
        HARDSRC_W::new(self)
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
#[doc = "Channel5 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trig5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trig5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIG5_SPEC;
impl crate::RegisterSpec for TRIG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trig5::R`](R) reader structure"]
impl crate::Readable for TRIG5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trig5::W`](W) writer structure"]
impl crate::Writable for TRIG5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIG5 to value 0"]
impl crate::Resettable for TRIG5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
