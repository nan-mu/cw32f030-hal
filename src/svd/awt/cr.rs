#[doc = "Register `CR` reader"]
pub type R = crate::svd::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::svd::W<CR_SPEC>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::svd::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `MD` reader - desc MD"]
pub type MD_R = crate::svd::FieldReader;
#[doc = "Field `MD` writer - desc MD"]
pub type MD_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PRS_R = crate::svd::FieldReader;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PRS_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SRC` reader - desc SRC"]
pub type SRC_R = crate::svd::FieldReader;
#[doc = "Field `SRC` writer - desc SRC"]
pub type SRC_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc MD"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - desc SRC"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CR_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - desc MD"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<CR_SPEC, 1> {
        MD_W::new(self)
    }
    #[doc = "Bits 4:7 - desc PRS"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<CR_SPEC, 4> {
        PRS_W::new(self)
    }
    #[doc = "Bits 8:10 - desc SRC"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<CR_SPEC, 8> {
        SRC_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::svd::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::svd::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::svd::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::svd::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
