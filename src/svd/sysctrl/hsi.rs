#[doc = "Register `HSI` reader"]
pub type R = crate::svd::R<HSI_SPEC>;
#[doc = "Register `HSI` writer"]
pub type W = crate::svd::W<HSI_SPEC>;
#[doc = "Field `TRIM` reader - desc TRIM"]
pub type TRIM_R = crate::svd::FieldReader<u16>;
#[doc = "Field `TRIM` writer - desc TRIM"]
pub type TRIM_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DIV_R = crate::svd::FieldReader;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DIV_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type STABLE_R = crate::svd::BitReader;
impl R {
    #[doc = "Bits 0:10 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - desc TRIM"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<HSI_SPEC, 0> {
        TRIM_W::new(self)
    }
    #[doc = "Bits 11:14 - desc DIV"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<HSI_SPEC, 11> {
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
#[doc = "HSI Control Reg\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`hsi::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`hsi::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSI_SPEC;
impl crate::svd::RegisterSpec for HSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsi::R`](R) reader structure"]
impl crate::svd::Readable for HSI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsi::W`](W) writer structure"]
impl crate::svd::Writable for HSI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSI to value 0"]
impl crate::svd::Resettable for HSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
