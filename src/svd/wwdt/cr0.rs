#[doc = "Register `CR0` reader"]
pub type R = crate::svd::R<CR0_SPEC>;
#[doc = "Register `CR0` writer"]
pub type W = crate::svd::W<CR0_SPEC>;
#[doc = "Field `WCNT` reader - desc WCNT"]
pub type WCNT_R = crate::svd::FieldReader;
#[doc = "Field `WCNT` writer - desc WCNT"]
pub type WCNT_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::svd::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - desc WCNT"]
    #[inline(always)]
    pub fn wcnt(&self) -> WCNT_R {
        WCNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc WCNT"]
    #[inline(always)]
    #[must_use]
    pub fn wcnt(&mut self) -> WCNT_W<CR0_SPEC, 0> {
        WCNT_W::new(self)
    }
    #[doc = "Bit 7 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CR0_SPEC, 7> {
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
#[doc = "Control register1\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR0_SPEC;
impl crate::svd::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::svd::Readable for CR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::svd::Writable for CR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::svd::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
