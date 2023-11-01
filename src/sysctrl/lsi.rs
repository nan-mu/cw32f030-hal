#[doc = "Register `LSI` reader"]
pub type R = crate::R<LSI_SPEC>;
#[doc = "Register `LSI` writer"]
pub type W = crate::W<LSI_SPEC>;
#[doc = "Field `TRIM` reader - desc TRIM"]
pub type TRIM_R = crate::FieldReader<u16>;
#[doc = "Field `TRIM` writer - desc TRIM"]
pub type TRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `WAITCYCLE` reader - desc WAITCYCLE"]
pub type WAITCYCLE_R = crate::FieldReader;
#[doc = "Field `WAITCYCLE` writer - desc WAITCYCLE"]
pub type WAITCYCLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type STABLE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:11 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&self) -> WAITCYCLE_R {
        WAITCYCLE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 15 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - desc TRIM"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<LSI_SPEC, 0> {
        TRIM_W::new(self)
    }
    #[doc = "Bits 10:11 - desc WAITCYCLE"]
    #[inline(always)]
    #[must_use]
    pub fn waitcycle(&mut self) -> WAITCYCLE_W<LSI_SPEC, 10> {
        WAITCYCLE_W::new(self)
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
#[doc = "LSI Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSI_SPEC;
impl crate::RegisterSpec for LSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsi::R`](R) reader structure"]
impl crate::Readable for LSI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lsi::W`](W) writer structure"]
impl crate::Writable for LSI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSI to value 0"]
impl crate::Resettable for LSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
