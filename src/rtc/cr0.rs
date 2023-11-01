#[doc = "Register `CR0` reader"]
pub type R = crate::R<CR0_SPEC>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<CR0_SPEC>;
#[doc = "Field `INTERVAL` reader - desc INTERVAL"]
pub type INTERVAL_R = crate::FieldReader;
#[doc = "Field `INTERVAL` writer - desc INTERVAL"]
pub type INTERVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `H24` reader - desc H24"]
pub type H24_R = crate::BitReader;
#[doc = "Field `H24` writer - desc H24"]
pub type H24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTC1HZ` reader - desc RTC1HZ"]
pub type RTC1HZ_R = crate::FieldReader;
#[doc = "Field `RTC1HZ` writer - desc RTC1HZ"]
pub type RTC1HZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `START` reader - desc START"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - desc INTERVAL"]
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc H24"]
    #[inline(always)]
    pub fn h24(&self) -> H24_R {
        H24_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:6 - desc RTC1HZ"]
    #[inline(always)]
    pub fn rtc1hz(&self) -> RTC1HZ_R {
        RTC1HZ_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc INTERVAL"]
    #[inline(always)]
    #[must_use]
    pub fn interval(&mut self) -> INTERVAL_W<CR0_SPEC, 0> {
        INTERVAL_W::new(self)
    }
    #[doc = "Bit 3 - desc H24"]
    #[inline(always)]
    #[must_use]
    pub fn h24(&mut self) -> H24_W<CR0_SPEC, 3> {
        H24_W::new(self)
    }
    #[doc = "Bits 5:6 - desc RTC1HZ"]
    #[inline(always)]
    #[must_use]
    pub fn rtc1hz(&mut self) -> RTC1HZ_W<CR0_SPEC, 5> {
        RTC1HZ_W::new(self)
    }
    #[doc = "Bit 7 - desc START"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CR0_SPEC, 7> {
        START_W::new(self)
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
#[doc = "Control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for CR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
