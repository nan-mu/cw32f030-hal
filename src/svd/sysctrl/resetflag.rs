#[doc = "Register `RESETFLAG` reader"]
pub type R = crate::svd::R<RESETFLAG_SPEC>;
#[doc = "Register `RESETFLAG` writer"]
pub type W = crate::svd::W<RESETFLAG_SPEC>;
#[doc = "Field `POR` reader - desc POR"]
pub type POR_R = crate::svd::BitReader;
#[doc = "Field `POR` writer - desc POR"]
pub type POR_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LVD` reader - desc LVD"]
pub type LVD_R = crate::svd::BitReader;
#[doc = "Field `LVD` writer - desc LVD"]
pub type LVD_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `IWDT` reader - desc IWDT"]
pub type IWDT_R = crate::svd::BitReader;
#[doc = "Field `IWDT` writer - desc IWDT"]
pub type IWDT_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `WWDT` reader - desc WWDT"]
pub type WWDT_R = crate::svd::BitReader;
#[doc = "Field `WWDT` writer - desc WWDT"]
pub type WWDT_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `RSTB` reader - desc RSTB"]
pub type RSTB_R = crate::svd::BitReader;
#[doc = "Field `RSTB` writer - desc RSTB"]
pub type RSTB_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKUP` reader - desc LOCKUP"]
pub type LOCKUP_R = crate::svd::BitReader;
#[doc = "Field `LOCKUP` writer - desc LOCKUP"]
pub type LOCKUP_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SYSRESETREQ` reader - desc SYSRESETREQ"]
pub type SYSRESETREQ_R = crate::svd::BitReader;
#[doc = "Field `SYSRESETREQ` writer - desc SYSRESETREQ"]
pub type SYSRESETREQ_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc POR"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - desc LVD"]
    #[inline(always)]
    pub fn lvd(&self) -> LVD_R {
        LVD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&self) -> IWDT_R {
        IWDT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc WWDT"]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RSTB"]
    #[inline(always)]
    pub fn rstb(&self) -> RSTB_R {
        RSTB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc LOCKUP"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SYSRESETREQ"]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SYSRESETREQ_R {
        SYSRESETREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POR"]
    #[inline(always)]
    #[must_use]
    pub fn por(&mut self) -> POR_W<RESETFLAG_SPEC, 0> {
        POR_W::new(self)
    }
    #[doc = "Bit 3 - desc LVD"]
    #[inline(always)]
    #[must_use]
    pub fn lvd(&mut self) -> LVD_W<RESETFLAG_SPEC, 3> {
        LVD_W::new(self)
    }
    #[doc = "Bit 4 - desc IWDT"]
    #[inline(always)]
    #[must_use]
    pub fn iwdt(&mut self) -> IWDT_W<RESETFLAG_SPEC, 4> {
        IWDT_W::new(self)
    }
    #[doc = "Bit 5 - desc WWDT"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt(&mut self) -> WWDT_W<RESETFLAG_SPEC, 5> {
        WWDT_W::new(self)
    }
    #[doc = "Bit 6 - desc RSTB"]
    #[inline(always)]
    #[must_use]
    pub fn rstb(&mut self) -> RSTB_W<RESETFLAG_SPEC, 6> {
        RSTB_W::new(self)
    }
    #[doc = "Bit 8 - desc LOCKUP"]
    #[inline(always)]
    #[must_use]
    pub fn lockup(&mut self) -> LOCKUP_W<RESETFLAG_SPEC, 8> {
        LOCKUP_W::new(self)
    }
    #[doc = "Bit 9 - desc SYSRESETREQ"]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W<RESETFLAG_SPEC, 9> {
        SYSRESETREQ_W::new(self)
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
#[doc = "Reset Status Reg\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`resetflag::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`resetflag::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESETFLAG_SPEC;
impl crate::svd::RegisterSpec for RESETFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetflag::R`](R) reader structure"]
impl crate::svd::Readable for RESETFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`resetflag::W`](W) writer structure"]
impl crate::svd::Writable for RESETFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESETFLAG to value 0"]
impl crate::svd::Resettable for RESETFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
