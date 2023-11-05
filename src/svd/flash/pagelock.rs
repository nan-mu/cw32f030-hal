#[doc = "Register `PAGELOCK` reader"]
pub type R = crate::svd::R<PAGELOCK_SPEC>;
#[doc = "Register `PAGELOCK` writer"]
pub type W = crate::svd::W<PAGELOCK_SPEC>;
#[doc = "Field `LOCK0` reader - Page0 - 7"]
pub type LOCK0_R = crate::svd::BitReader;
#[doc = "Field `LOCK0` writer - Page0 - 7"]
pub type LOCK0_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK1` reader - Page8 - 15"]
pub type LOCK1_R = crate::svd::BitReader;
#[doc = "Field `LOCK1` writer - Page8 - 15"]
pub type LOCK1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK2` reader - Page16 - 23"]
pub type LOCK2_R = crate::svd::BitReader;
#[doc = "Field `LOCK2` writer - Page16 - 23"]
pub type LOCK2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK3` reader - Page24 - 31"]
pub type LOCK3_R = crate::svd::BitReader;
#[doc = "Field `LOCK3` writer - Page24 - 31"]
pub type LOCK3_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK4` reader - Page32 - 39"]
pub type LOCK4_R = crate::svd::BitReader;
#[doc = "Field `LOCK4` writer - Page32 - 39"]
pub type LOCK4_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK5` reader - Page40 - 47"]
pub type LOCK5_R = crate::svd::BitReader;
#[doc = "Field `LOCK5` writer - Page40 - 47"]
pub type LOCK5_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK6` reader - Page48 - 55"]
pub type LOCK6_R = crate::svd::BitReader;
#[doc = "Field `LOCK6` writer - Page48 - 55"]
pub type LOCK6_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK7` reader - Page56 - 63"]
pub type LOCK7_R = crate::svd::BitReader;
#[doc = "Field `LOCK7` writer - Page56 - 63"]
pub type LOCK7_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK8` reader - Page64 - 71"]
pub type LOCK8_R = crate::svd::BitReader;
#[doc = "Field `LOCK8` writer - Page64 - 71"]
pub type LOCK8_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK9` reader - Page72 - 79"]
pub type LOCK9_R = crate::svd::BitReader;
#[doc = "Field `LOCK9` writer - Page72 - 79"]
pub type LOCK9_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK10` reader - Page80 - 87"]
pub type LOCK10_R = crate::svd::BitReader;
#[doc = "Field `LOCK10` writer - Page80 - 87"]
pub type LOCK10_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK11` reader - Page88 - 95"]
pub type LOCK11_R = crate::svd::BitReader;
#[doc = "Field `LOCK11` writer - Page88 - 95"]
pub type LOCK11_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK12` reader - Page96 - 103"]
pub type LOCK12_R = crate::svd::BitReader;
#[doc = "Field `LOCK12` writer - Page96 - 103"]
pub type LOCK12_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK13` reader - Page104 - 111"]
pub type LOCK13_R = crate::svd::BitReader;
#[doc = "Field `LOCK13` writer - Page104 - 111"]
pub type LOCK13_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK14` reader - Page112 - 119"]
pub type LOCK14_R = crate::svd::BitReader;
#[doc = "Field `LOCK14` writer - Page112 - 119"]
pub type LOCK14_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK15` reader - Page120 - 127"]
pub type LOCK15_R = crate::svd::BitReader;
#[doc = "Field `LOCK15` writer - Page120 - 127"]
pub type LOCK15_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KEY_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Page0 - 7"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page8 - 15"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Page16 - 23"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Page24 - 31"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Page32 - 39"]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Page40 - 47"]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Page48 - 55"]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Page56 - 63"]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Page64 - 71"]
    #[inline(always)]
    pub fn lock8(&self) -> LOCK8_R {
        LOCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Page72 - 79"]
    #[inline(always)]
    pub fn lock9(&self) -> LOCK9_R {
        LOCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Page80 - 87"]
    #[inline(always)]
    pub fn lock10(&self) -> LOCK10_R {
        LOCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Page88 - 95"]
    #[inline(always)]
    pub fn lock11(&self) -> LOCK11_R {
        LOCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Page96 - 103"]
    #[inline(always)]
    pub fn lock12(&self) -> LOCK12_R {
        LOCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Page104 - 111"]
    #[inline(always)]
    pub fn lock13(&self) -> LOCK13_R {
        LOCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Page112 - 119"]
    #[inline(always)]
    pub fn lock14(&self) -> LOCK14_R {
        LOCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Page120 - 127"]
    #[inline(always)]
    pub fn lock15(&self) -> LOCK15_R {
        LOCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Page0 - 7"]
    #[inline(always)]
    #[must_use]
    pub fn lock0(&mut self) -> LOCK0_W<PAGELOCK_SPEC, 0> {
        LOCK0_W::new(self)
    }
    #[doc = "Bit 1 - Page8 - 15"]
    #[inline(always)]
    #[must_use]
    pub fn lock1(&mut self) -> LOCK1_W<PAGELOCK_SPEC, 1> {
        LOCK1_W::new(self)
    }
    #[doc = "Bit 2 - Page16 - 23"]
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> LOCK2_W<PAGELOCK_SPEC, 2> {
        LOCK2_W::new(self)
    }
    #[doc = "Bit 3 - Page24 - 31"]
    #[inline(always)]
    #[must_use]
    pub fn lock3(&mut self) -> LOCK3_W<PAGELOCK_SPEC, 3> {
        LOCK3_W::new(self)
    }
    #[doc = "Bit 4 - Page32 - 39"]
    #[inline(always)]
    #[must_use]
    pub fn lock4(&mut self) -> LOCK4_W<PAGELOCK_SPEC, 4> {
        LOCK4_W::new(self)
    }
    #[doc = "Bit 5 - Page40 - 47"]
    #[inline(always)]
    #[must_use]
    pub fn lock5(&mut self) -> LOCK5_W<PAGELOCK_SPEC, 5> {
        LOCK5_W::new(self)
    }
    #[doc = "Bit 6 - Page48 - 55"]
    #[inline(always)]
    #[must_use]
    pub fn lock6(&mut self) -> LOCK6_W<PAGELOCK_SPEC, 6> {
        LOCK6_W::new(self)
    }
    #[doc = "Bit 7 - Page56 - 63"]
    #[inline(always)]
    #[must_use]
    pub fn lock7(&mut self) -> LOCK7_W<PAGELOCK_SPEC, 7> {
        LOCK7_W::new(self)
    }
    #[doc = "Bit 8 - Page64 - 71"]
    #[inline(always)]
    #[must_use]
    pub fn lock8(&mut self) -> LOCK8_W<PAGELOCK_SPEC, 8> {
        LOCK8_W::new(self)
    }
    #[doc = "Bit 9 - Page72 - 79"]
    #[inline(always)]
    #[must_use]
    pub fn lock9(&mut self) -> LOCK9_W<PAGELOCK_SPEC, 9> {
        LOCK9_W::new(self)
    }
    #[doc = "Bit 10 - Page80 - 87"]
    #[inline(always)]
    #[must_use]
    pub fn lock10(&mut self) -> LOCK10_W<PAGELOCK_SPEC, 10> {
        LOCK10_W::new(self)
    }
    #[doc = "Bit 11 - Page88 - 95"]
    #[inline(always)]
    #[must_use]
    pub fn lock11(&mut self) -> LOCK11_W<PAGELOCK_SPEC, 11> {
        LOCK11_W::new(self)
    }
    #[doc = "Bit 12 - Page96 - 103"]
    #[inline(always)]
    #[must_use]
    pub fn lock12(&mut self) -> LOCK12_W<PAGELOCK_SPEC, 12> {
        LOCK12_W::new(self)
    }
    #[doc = "Bit 13 - Page104 - 111"]
    #[inline(always)]
    #[must_use]
    pub fn lock13(&mut self) -> LOCK13_W<PAGELOCK_SPEC, 13> {
        LOCK13_W::new(self)
    }
    #[doc = "Bit 14 - Page112 - 119"]
    #[inline(always)]
    #[must_use]
    pub fn lock14(&mut self) -> LOCK14_W<PAGELOCK_SPEC, 14> {
        LOCK14_W::new(self)
    }
    #[doc = "Bit 15 - Page120 - 127"]
    #[inline(always)]
    #[must_use]
    pub fn lock15(&mut self) -> LOCK15_W<PAGELOCK_SPEC, 15> {
        LOCK15_W::new(self)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<PAGELOCK_SPEC, 16> {
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
#[doc = "Page Write Erase Lock\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`pagelock::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`pagelock::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAGELOCK_SPEC;
impl crate::svd::RegisterSpec for PAGELOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pagelock::R`](R) reader structure"]
impl crate::svd::Readable for PAGELOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pagelock::W`](W) writer structure"]
impl crate::svd::Writable for PAGELOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAGELOCK to value 0"]
impl crate::svd::Resettable for PAGELOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
