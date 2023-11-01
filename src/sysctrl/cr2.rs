#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Field `SWDIO` reader - desc SWDIO"]
pub type SWDIO_R = crate::BitReader;
#[doc = "Field `SWDIO` writer - desc SWDIO"]
pub type SWDIO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKUP` reader - desc LOCKUP"]
pub type LOCKUP_R = crate::BitReader;
#[doc = "Field `LOCKUP` writer - desc LOCKUP"]
pub type LOCKUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPCLK` reader - desc WAKEUPCLK"]
pub type WAKEUPCLK_R = crate::BitReader;
#[doc = "Field `WAKEUPCLK` writer - desc WAKEUPCLK"]
pub type WAKEUPCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 1 - desc SWDIO"]
    #[inline(always)]
    pub fn swdio(&self) -> SWDIO_R {
        SWDIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc LOCKUP"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc WAKEUPCLK"]
    #[inline(always)]
    pub fn wakeupclk(&self) -> WAKEUPCLK_R {
        WAKEUPCLK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc SWDIO"]
    #[inline(always)]
    #[must_use]
    pub fn swdio(&mut self) -> SWDIO_W<CR2_SPEC, 1> {
        SWDIO_W::new(self)
    }
    #[doc = "Bit 2 - desc LOCKUP"]
    #[inline(always)]
    #[must_use]
    pub fn lockup(&mut self) -> LOCKUP_W<CR2_SPEC, 2> {
        LOCKUP_W::new(self)
    }
    #[doc = "Bit 3 - desc WAKEUPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupclk(&mut self) -> WAKEUPCLK_W<CR2_SPEC, 3> {
        WAKEUPCLK_W::new(self)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CR2_SPEC, 16> {
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
#[doc = "Control Reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
