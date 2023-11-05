#[doc = "Register `CR0` reader"]
pub type R = crate::svd::R<CR0_SPEC>;
#[doc = "Register `CR0` writer"]
pub type W = crate::svd::W<CR0_SPEC>;
#[doc = "Field `SYSCLK` reader - desc SYSCLK"]
pub type SYSCLK_R = crate::svd::FieldReader;
#[doc = "Field `SYSCLK` writer - desc SYSCLK"]
pub type SYSCLK_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PCLKPRS` reader - desc PCLKPRS"]
pub type PCLKPRS_R = crate::svd::FieldReader;
#[doc = "Field `PCLKPRS` writer - desc PCLKPRS"]
pub type PCLKPRS_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `HCLKPRS` reader - desc HCLKPRS"]
pub type HCLKPRS_R = crate::svd::FieldReader;
#[doc = "Field `HCLKPRS` writer - desc HCLKPRS"]
pub type HCLKPRS_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KEY_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:2 - desc SYSCLK"]
    #[inline(always)]
    pub fn sysclk(&self) -> SYSCLK_R {
        SYSCLK_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - desc PCLKPRS"]
    #[inline(always)]
    pub fn pclkprs(&self) -> PCLKPRS_R {
        PCLKPRS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - desc HCLKPRS"]
    #[inline(always)]
    pub fn hclkprs(&self) -> HCLKPRS_R {
        HCLKPRS_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc SYSCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sysclk(&mut self) -> SYSCLK_W<CR0_SPEC, 0> {
        SYSCLK_W::new(self)
    }
    #[doc = "Bits 3:4 - desc PCLKPRS"]
    #[inline(always)]
    #[must_use]
    pub fn pclkprs(&mut self) -> PCLKPRS_W<CR0_SPEC, 3> {
        PCLKPRS_W::new(self)
    }
    #[doc = "Bits 5:7 - desc HCLKPRS"]
    #[inline(always)]
    #[must_use]
    pub fn hclkprs(&mut self) -> HCLKPRS_W<CR0_SPEC, 5> {
        HCLKPRS_W::new(self)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CR0_SPEC, 16> {
        KEY_W::new(self)
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
#[doc = "Control Reg0\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
