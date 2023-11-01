#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `HSIEN` reader - desc HSIEN"]
pub type HSIEN_R = crate::BitReader;
#[doc = "Field `HSIEN` writer - desc HSIEN"]
pub type HSIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSEEN` reader - desc HSEEN"]
pub type HSEEN_R = crate::BitReader;
#[doc = "Field `HSEEN` writer - desc HSEEN"]
pub type HSEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLEN` reader - desc PLLEN"]
pub type PLLEN_R = crate::BitReader;
#[doc = "Field `PLLEN` writer - desc PLLEN"]
pub type PLLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSIEN` reader - desc LSIEN"]
pub type LSIEN_R = crate::BitReader;
#[doc = "Field `LSIEN` writer - desc LSIEN"]
pub type LSIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSEEN` reader - desc LSEEN"]
pub type LSEEN_R = crate::BitReader;
#[doc = "Field `LSEEN` writer - desc LSEEN"]
pub type LSEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSELOCK` reader - desc LSELOCK"]
pub type LSELOCK_R = crate::BitReader;
#[doc = "Field `LSELOCK` writer - desc LSELOCK"]
pub type LSELOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSECCS` reader - desc LSECCS"]
pub type LSECCS_R = crate::BitReader;
#[doc = "Field `LSECCS` writer - desc LSECCS"]
pub type LSECCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSECCS` reader - desc HSECCS"]
pub type HSECCS_R = crate::BitReader;
#[doc = "Field `HSECCS` writer - desc HSECCS"]
pub type HSECCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKCCS` reader - desc CLKCCS"]
pub type CLKCCS_R = crate::BitReader;
#[doc = "Field `CLKCCS` writer - desc CLKCCS"]
pub type CLKCCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - desc HSIEN"]
    #[inline(always)]
    pub fn hsien(&self) -> HSIEN_R {
        HSIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSEEN"]
    #[inline(always)]
    pub fn hseen(&self) -> HSEEN_R {
        HSEEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc LSIEN"]
    #[inline(always)]
    pub fn lsien(&self) -> LSIEN_R {
        LSIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc LSEEN"]
    #[inline(always)]
    pub fn lseen(&self) -> LSEEN_R {
        LSEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc LSELOCK"]
    #[inline(always)]
    pub fn lselock(&self) -> LSELOCK_R {
        LSELOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc LSECCS"]
    #[inline(always)]
    pub fn lseccs(&self) -> LSECCS_R {
        LSECCS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HSECCS"]
    #[inline(always)]
    pub fn hseccs(&self) -> HSECCS_R {
        HSECCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CLKCCS"]
    #[inline(always)]
    pub fn clkccs(&self) -> CLKCCS_R {
        CLKCCS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HSIEN"]
    #[inline(always)]
    #[must_use]
    pub fn hsien(&mut self) -> HSIEN_W<CR1_SPEC, 0> {
        HSIEN_W::new(self)
    }
    #[doc = "Bit 1 - desc HSEEN"]
    #[inline(always)]
    #[must_use]
    pub fn hseen(&mut self) -> HSEEN_W<CR1_SPEC, 1> {
        HSEEN_W::new(self)
    }
    #[doc = "Bit 2 - desc PLLEN"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<CR1_SPEC, 2> {
        PLLEN_W::new(self)
    }
    #[doc = "Bit 3 - desc LSIEN"]
    #[inline(always)]
    #[must_use]
    pub fn lsien(&mut self) -> LSIEN_W<CR1_SPEC, 3> {
        LSIEN_W::new(self)
    }
    #[doc = "Bit 4 - desc LSEEN"]
    #[inline(always)]
    #[must_use]
    pub fn lseen(&mut self) -> LSEEN_W<CR1_SPEC, 4> {
        LSEEN_W::new(self)
    }
    #[doc = "Bit 5 - desc LSELOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lselock(&mut self) -> LSELOCK_W<CR1_SPEC, 5> {
        LSELOCK_W::new(self)
    }
    #[doc = "Bit 6 - desc LSECCS"]
    #[inline(always)]
    #[must_use]
    pub fn lseccs(&mut self) -> LSECCS_W<CR1_SPEC, 6> {
        LSECCS_W::new(self)
    }
    #[doc = "Bit 7 - desc HSECCS"]
    #[inline(always)]
    #[must_use]
    pub fn hseccs(&mut self) -> HSECCS_W<CR1_SPEC, 7> {
        HSECCS_W::new(self)
    }
    #[doc = "Bit 8 - desc CLKCCS"]
    #[inline(always)]
    #[must_use]
    pub fn clkccs(&mut self) -> CLKCCS_W<CR1_SPEC, 8> {
        CLKCCS_W::new(self)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CR1_SPEC, 16> {
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
#[doc = "Control Reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
