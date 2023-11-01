#[doc = "Register `CH2CR` reader"]
pub type R = crate::R<CH2CR_SPEC>;
#[doc = "Register `CH2CR` writer"]
pub type W = crate::W<CH2CR_SPEC>;
#[doc = "Field `BKSA` reader - desc BKSA"]
pub type BKSA_R = crate::FieldReader;
#[doc = "Field `BKSA` writer - desc BKSA"]
pub type BKSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BKSB` reader - desc BKSB"]
pub type BKSB_R = crate::FieldReader;
#[doc = "Field `BKSB` writer - desc BKSB"]
pub type BKSB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CSA` reader - desc CSA"]
pub type CSA_R = crate::BitReader;
#[doc = "Field `CSA` writer - desc CSA"]
pub type CSA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSB` reader - desc CSB"]
pub type CSB_R = crate::BitReader;
#[doc = "Field `CSB` writer - desc CSB"]
pub type CSB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUFEA` reader - desc BUFEA"]
pub type BUFEA_R = crate::BitReader;
#[doc = "Field `BUFEA` writer - desc BUFEA"]
pub type BUFEA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUFEB` reader - desc BUFEB"]
pub type BUFEB_R = crate::BitReader;
#[doc = "Field `BUFEB` writer - desc BUFEB"]
pub type BUFEB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CIEA` reader - desc CIEA"]
pub type CIEA_R = crate::BitReader;
#[doc = "Field `CIEA` writer - desc CIEA"]
pub type CIEA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CIEB` reader - desc CIEB"]
pub type CIEB_R = crate::BitReader;
#[doc = "Field `CIEB` writer - desc CIEB"]
pub type CIEB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDEA` reader - desc CDEA"]
pub type CDEA_R = crate::BitReader;
#[doc = "Field `CDEA` writer - desc CDEA"]
pub type CDEA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDEB` reader - desc CDEB"]
pub type CDEB_R = crate::BitReader;
#[doc = "Field `CDEB` writer - desc CDEB"]
pub type CDEB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CISB` reader - desc CISB"]
pub type CISB_R = crate::FieldReader;
#[doc = "Field `CISB` writer - desc CISB"]
pub type CISB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CCGA` reader - desc CCGA"]
pub type CCGA_R = crate::BitReader;
#[doc = "Field `CCGA` writer - desc CCGA"]
pub type CCGA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCGB` reader - desc CCGB"]
pub type CCGB_R = crate::BitReader;
#[doc = "Field `CCGB` writer - desc CCGB"]
pub type CCGB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - desc BKSA"]
    #[inline(always)]
    pub fn bksa(&self) -> BKSA_R {
        BKSA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc BKSB"]
    #[inline(always)]
    pub fn bksb(&self) -> BKSB_R {
        BKSB_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - desc CSA"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CSB"]
    #[inline(always)]
    pub fn csb(&self) -> CSB_R {
        CSB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc BUFEA"]
    #[inline(always)]
    pub fn bufea(&self) -> BUFEA_R {
        BUFEA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BUFEB"]
    #[inline(always)]
    pub fn bufeb(&self) -> BUFEB_R {
        BUFEB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CIEA"]
    #[inline(always)]
    pub fn ciea(&self) -> CIEA_R {
        CIEA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CIEB"]
    #[inline(always)]
    pub fn cieb(&self) -> CIEB_R {
        CIEB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CDEA"]
    #[inline(always)]
    pub fn cdea(&self) -> CDEA_R {
        CDEA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CDEB"]
    #[inline(always)]
    pub fn cdeb(&self) -> CDEB_R {
        CDEB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - desc CISB"]
    #[inline(always)]
    pub fn cisb(&self) -> CISB_R {
        CISB_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc CCGA"]
    #[inline(always)]
    pub fn ccga(&self) -> CCGA_R {
        CCGA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc CCGB"]
    #[inline(always)]
    pub fn ccgb(&self) -> CCGB_R {
        CCGB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc BKSA"]
    #[inline(always)]
    #[must_use]
    pub fn bksa(&mut self) -> BKSA_W<CH2CR_SPEC, 0> {
        BKSA_W::new(self)
    }
    #[doc = "Bits 2:3 - desc BKSB"]
    #[inline(always)]
    #[must_use]
    pub fn bksb(&mut self) -> BKSB_W<CH2CR_SPEC, 2> {
        BKSB_W::new(self)
    }
    #[doc = "Bit 4 - desc CSA"]
    #[inline(always)]
    #[must_use]
    pub fn csa(&mut self) -> CSA_W<CH2CR_SPEC, 4> {
        CSA_W::new(self)
    }
    #[doc = "Bit 5 - desc CSB"]
    #[inline(always)]
    #[must_use]
    pub fn csb(&mut self) -> CSB_W<CH2CR_SPEC, 5> {
        CSB_W::new(self)
    }
    #[doc = "Bit 6 - desc BUFEA"]
    #[inline(always)]
    #[must_use]
    pub fn bufea(&mut self) -> BUFEA_W<CH2CR_SPEC, 6> {
        BUFEA_W::new(self)
    }
    #[doc = "Bit 7 - desc BUFEB"]
    #[inline(always)]
    #[must_use]
    pub fn bufeb(&mut self) -> BUFEB_W<CH2CR_SPEC, 7> {
        BUFEB_W::new(self)
    }
    #[doc = "Bit 8 - desc CIEA"]
    #[inline(always)]
    #[must_use]
    pub fn ciea(&mut self) -> CIEA_W<CH2CR_SPEC, 8> {
        CIEA_W::new(self)
    }
    #[doc = "Bit 9 - desc CIEB"]
    #[inline(always)]
    #[must_use]
    pub fn cieb(&mut self) -> CIEB_W<CH2CR_SPEC, 9> {
        CIEB_W::new(self)
    }
    #[doc = "Bit 10 - desc CDEA"]
    #[inline(always)]
    #[must_use]
    pub fn cdea(&mut self) -> CDEA_W<CH2CR_SPEC, 10> {
        CDEA_W::new(self)
    }
    #[doc = "Bit 11 - desc CDEB"]
    #[inline(always)]
    #[must_use]
    pub fn cdeb(&mut self) -> CDEB_W<CH2CR_SPEC, 11> {
        CDEB_W::new(self)
    }
    #[doc = "Bits 12:13 - desc CISB"]
    #[inline(always)]
    #[must_use]
    pub fn cisb(&mut self) -> CISB_W<CH2CR_SPEC, 12> {
        CISB_W::new(self)
    }
    #[doc = "Bit 14 - desc CCGA"]
    #[inline(always)]
    #[must_use]
    pub fn ccga(&mut self) -> CCGA_W<CH2CR_SPEC, 14> {
        CCGA_W::new(self)
    }
    #[doc = "Bit 15 - desc CCGB"]
    #[inline(always)]
    #[must_use]
    pub fn ccgb(&mut self) -> CCGB_W<CH2CR_SPEC, 15> {
        CCGB_W::new(self)
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
#[doc = "desc CH2CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2CR_SPEC;
impl crate::RegisterSpec for CH2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2cr::R`](R) reader structure"]
impl crate::Readable for CH2CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2cr::W`](W) writer structure"]
impl crate::Writable for CH2CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2CR to value 0"]
impl crate::Resettable for CH2CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
