#[doc = "Register `HSE` reader"]
pub type R = crate::R<HSE_SPEC>;
#[doc = "Register `HSE` writer"]
pub type W = crate::W<HSE_SPEC>;
#[doc = "Field `DRIVER` reader - desc DRIVER"]
pub type DRIVER_R = crate::FieldReader;
#[doc = "Field `DRIVER` writer - desc DRIVER"]
pub type DRIVER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FREQ` reader - desc FREQ"]
pub type FREQ_R = crate::FieldReader;
#[doc = "Field `FREQ` writer - desc FREQ"]
pub type FREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `WAITCYCLE` reader - desc WAITCYCLE"]
pub type WAITCYCLE_R = crate::FieldReader;
#[doc = "Field `WAITCYCLE` writer - desc WAITCYCLE"]
pub type WAITCYCLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT` reader - desc FLT"]
pub type FLT_R = crate::BitReader;
#[doc = "Field `FLT` writer - desc FLT"]
pub type FLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DETCNT` reader - desc DETCNT"]
pub type DETCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DETCNT` writer - desc DETCNT"]
pub type DETCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type STABLE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - desc DRIVER"]
    #[inline(always)]
    pub fn driver(&self) -> DRIVER_R {
        DRIVER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc FREQ"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&self) -> WAITCYCLE_R {
        WAITCYCLE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc FLT"]
    #[inline(always)]
    pub fn flt(&self) -> FLT_R {
        FLT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:18 - desc DETCNT"]
    #[inline(always)]
    pub fn detcnt(&self) -> DETCNT_R {
        DETCNT_R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bit 19 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc DRIVER"]
    #[inline(always)]
    #[must_use]
    pub fn driver(&mut self) -> DRIVER_W<HSE_SPEC, 0> {
        DRIVER_W::new(self)
    }
    #[doc = "Bits 2:3 - desc FREQ"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<HSE_SPEC, 2> {
        FREQ_W::new(self)
    }
    #[doc = "Bits 4:5 - desc WAITCYCLE"]
    #[inline(always)]
    #[must_use]
    pub fn waitcycle(&mut self) -> WAITCYCLE_W<HSE_SPEC, 4> {
        WAITCYCLE_W::new(self)
    }
    #[doc = "Bit 6 - desc MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<HSE_SPEC, 6> {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - desc FLT"]
    #[inline(always)]
    #[must_use]
    pub fn flt(&mut self) -> FLT_W<HSE_SPEC, 7> {
        FLT_W::new(self)
    }
    #[doc = "Bits 8:18 - desc DETCNT"]
    #[inline(always)]
    #[must_use]
    pub fn detcnt(&mut self) -> DETCNT_W<HSE_SPEC, 8> {
        DETCNT_W::new(self)
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
#[doc = "HSE Control Reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSE_SPEC;
impl crate::RegisterSpec for HSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hse::R`](R) reader structure"]
impl crate::Readable for HSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hse::W`](W) writer structure"]
impl crate::Writable for HSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSE to value 0"]
impl crate::Resettable for HSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
