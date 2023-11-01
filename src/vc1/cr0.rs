#[doc = "Register `CR0` reader"]
pub type R = crate::R<CR0_SPEC>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<CR0_SPEC>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESP` reader - desc RESP"]
pub type RESP_R = crate::FieldReader;
#[doc = "Field `RESP` writer - desc RESP"]
pub type RESP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `HYS` reader - desc HYS"]
pub type HYS_R = crate::FieldReader;
#[doc = "Field `HYS` writer - desc HYS"]
pub type HYS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IE` reader - desc IE"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - desc IE"]
pub type IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POL` reader - desc POL"]
pub type POL_R = crate::BitReader;
#[doc = "Field `POL` writer - desc POL"]
pub type POL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WINDOW` reader - desc WINDOW"]
pub type WINDOW_R = crate::BitReader;
#[doc = "Field `WINDOW` writer - desc WINDOW"]
pub type WINDOW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INP` reader - desc INP"]
pub type INP_R = crate::FieldReader;
#[doc = "Field `INP` writer - desc INP"]
pub type INP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `INN` reader - desc INN"]
pub type INN_R = crate::FieldReader;
#[doc = "Field `INN` writer - desc INN"]
pub type INN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc RESP"]
    #[inline(always)]
    pub fn resp(&self) -> RESP_R {
        RESP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - desc HYS"]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc POL"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc WINDOW"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - desc INP"]
    #[inline(always)]
    pub fn inp(&self) -> INP_R {
        INP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc INN"]
    #[inline(always)]
    pub fn inn(&self) -> INN_R {
        INN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CR0_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - desc RESP"]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self) -> RESP_W<CR0_SPEC, 1> {
        RESP_W::new(self)
    }
    #[doc = "Bits 3:4 - desc HYS"]
    #[inline(always)]
    #[must_use]
    pub fn hys(&mut self) -> HYS_W<CR0_SPEC, 3> {
        HYS_W::new(self)
    }
    #[doc = "Bit 5 - desc IE"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CR0_SPEC, 5> {
        IE_W::new(self)
    }
    #[doc = "Bit 6 - desc POL"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CR0_SPEC, 6> {
        POL_W::new(self)
    }
    #[doc = "Bit 7 - desc WINDOW"]
    #[inline(always)]
    #[must_use]
    pub fn window(&mut self) -> WINDOW_W<CR0_SPEC, 7> {
        WINDOW_W::new(self)
    }
    #[doc = "Bits 8:11 - desc INP"]
    #[inline(always)]
    #[must_use]
    pub fn inp(&mut self) -> INP_W<CR0_SPEC, 8> {
        INP_W::new(self)
    }
    #[doc = "Bits 12:15 - desc INN"]
    #[inline(always)]
    #[must_use]
    pub fn inn(&mut self) -> INN_W<CR0_SPEC, 12> {
        INN_W::new(self)
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
