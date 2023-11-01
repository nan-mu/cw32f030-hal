#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PRS_R = crate::FieldReader;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PRS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ACTION` reader - desc ACTION"]
pub type ACTION_R = crate::BitReader;
#[doc = "Field `ACTION` writer - desc ACTION"]
pub type ACTION_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE` reader - desc IE"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - desc IE"]
pub type IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAUSE` reader - desc PAUSE"]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `PAUSE` writer - desc PAUSE"]
pub type PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc ACTION"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PAUSE"]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc PRS"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<CR_SPEC, 0> {
        PRS_W::new(self)
    }
    #[doc = "Bit 3 - desc ACTION"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<CR_SPEC, 3> {
        ACTION_W::new(self)
    }
    #[doc = "Bit 4 - desc IE"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CR_SPEC, 4> {
        IE_W::new(self)
    }
    #[doc = "Bit 5 - desc PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<CR_SPEC, 5> {
        PAUSE_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
