#[doc = "Register `COMPEN` reader"]
pub type R = crate::svd::R<COMPEN_SPEC>;
#[doc = "Register `COMPEN` writer"]
pub type W = crate::svd::W<COMPEN_SPEC>;
#[doc = "Field `COMP` reader - desc COMP"]
pub type COMP_R = crate::svd::FieldReader<u16>;
#[doc = "Field `COMP` writer - desc COMP"]
pub type COMP_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `STEP` reader - desc STEP"]
pub type STEP_R = crate::svd::FieldReader;
#[doc = "Field `STEP` writer - desc STEP"]
pub type STEP_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SIGN` reader - desc SIGN"]
pub type SIGN_R = crate::svd::BitReader;
#[doc = "Field `SIGN` writer - desc SIGN"]
pub type SIGN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::svd::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `FREQ` reader - desc FREQ"]
pub type FREQ_R = crate::svd::FieldReader;
#[doc = "Field `FREQ` writer - desc FREQ"]
pub type FREQ_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:11 - desc COMP"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - desc STEP"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc SIGN"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - desc FREQ"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc COMP"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<COMPEN_SPEC, 0> {
        COMP_W::new(self)
    }
    #[doc = "Bits 12:13 - desc STEP"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<COMPEN_SPEC, 12> {
        STEP_W::new(self)
    }
    #[doc = "Bit 14 - desc SIGN"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SIGN_W<COMPEN_SPEC, 14> {
        SIGN_W::new(self)
    }
    #[doc = "Bit 15 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<COMPEN_SPEC, 15> {
        EN_W::new(self)
    }
    #[doc = "Bits 16:19 - desc FREQ"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<COMPEN_SPEC, 16> {
        FREQ_W::new(self)
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
#[doc = "Compen register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`compen::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`compen::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMPEN_SPEC;
impl crate::svd::RegisterSpec for COMPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compen::R`](R) reader structure"]
impl crate::svd::Readable for COMPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`compen::W`](W) writer structure"]
impl crate::svd::Writable for COMPEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPEN to value 0"]
impl crate::svd::Resettable for COMPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
