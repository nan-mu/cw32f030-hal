#[doc = "Register `CR0` reader"]
pub type R = crate::svd::R<CR0_SPEC>;
#[doc = "Register `CR0` writer"]
pub type W = crate::svd::W<CR0_SPEC>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::svd::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::svd::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TRS` reader - desc TRS"]
pub type TRS_R = crate::svd::BitReader;
#[doc = "Field `TRS` writer - desc TRS"]
pub type TRS_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `POL` reader - desc POL"]
pub type POL_R = crate::svd::BitReader;
#[doc = "Field `POL` writer - desc POL"]
pub type POL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `ONESHOT` reader - desc ONESHOT"]
pub type ONESHOT_R = crate::svd::BitReader;
#[doc = "Field `ONESHOT` writer - desc ONESHOT"]
pub type ONESHOT_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TOGEN` reader - desc TOGEN"]
pub type TOGEN_R = crate::svd::BitReader;
#[doc = "Field `TOGEN` writer - desc TOGEN"]
pub type TOGEN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PRS_R = crate::svd::FieldReader;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PRS_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PRSSTATUS` reader - desc PRSSTATUS"]
pub type PRSSTATUS_R = crate::svd::FieldReader;
#[doc = "Field `ENCMODE` reader - desc ENCMODE"]
pub type ENCMODE_R = crate::svd::FieldReader;
#[doc = "Field `ENCMODE` writer - desc ENCMODE"]
pub type ENCMODE_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ENCRESET` reader - desc ENCRESET"]
pub type ENCRESET_R = crate::svd::FieldReader;
#[doc = "Field `ENCRESET` writer - desc ENCRESET"]
pub type ENCRESET_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ENCRELOAD` reader - desc ENCRELOAD"]
pub type ENCRELOAD_R = crate::svd::FieldReader;
#[doc = "Field `ENCRELOAD` writer - desc ENCRELOAD"]
pub type ENCRELOAD_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - desc TRS"]
    #[inline(always)]
    pub fn trs(&self) -> TRS_R {
        TRS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc POL"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ONESHOT"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TOGEN"]
    #[inline(always)]
    pub fn togen(&self) -> TOGEN_R {
        TOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - desc PRSSTATUS"]
    #[inline(always)]
    pub fn prsstatus(&self) -> PRSSTATUS_R {
        PRSSTATUS_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:16 - desc ENCMODE"]
    #[inline(always)]
    pub fn encmode(&self) -> ENCMODE_R {
        ENCMODE_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18 - desc ENCRESET"]
    #[inline(always)]
    pub fn encreset(&self) -> ENCRESET_R {
        ENCRESET_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - desc ENCRELOAD"]
    #[inline(always)]
    pub fn encreload(&self) -> ENCRELOAD_R {
        ENCRELOAD_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CR0_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - desc MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CR0_SPEC, 1> {
        MODE_W::new(self)
    }
    #[doc = "Bit 3 - desc TRS"]
    #[inline(always)]
    #[must_use]
    pub fn trs(&mut self) -> TRS_W<CR0_SPEC, 3> {
        TRS_W::new(self)
    }
    #[doc = "Bit 4 - desc POL"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CR0_SPEC, 4> {
        POL_W::new(self)
    }
    #[doc = "Bit 5 - desc ONESHOT"]
    #[inline(always)]
    #[must_use]
    pub fn oneshot(&mut self) -> ONESHOT_W<CR0_SPEC, 5> {
        ONESHOT_W::new(self)
    }
    #[doc = "Bit 6 - desc TOGEN"]
    #[inline(always)]
    #[must_use]
    pub fn togen(&mut self) -> TOGEN_W<CR0_SPEC, 6> {
        TOGEN_W::new(self)
    }
    #[doc = "Bits 7:10 - desc PRS"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<CR0_SPEC, 7> {
        PRS_W::new(self)
    }
    #[doc = "Bits 15:16 - desc ENCMODE"]
    #[inline(always)]
    #[must_use]
    pub fn encmode(&mut self) -> ENCMODE_W<CR0_SPEC, 15> {
        ENCMODE_W::new(self)
    }
    #[doc = "Bits 17:18 - desc ENCRESET"]
    #[inline(always)]
    #[must_use]
    pub fn encreset(&mut self) -> ENCRESET_W<CR0_SPEC, 17> {
        ENCRESET_W::new(self)
    }
    #[doc = "Bits 19:20 - desc ENCRELOAD"]
    #[inline(always)]
    #[must_use]
    pub fn encreload(&mut self) -> ENCRELOAD_W<CR0_SPEC, 19> {
        ENCRELOAD_W::new(self)
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
#[doc = "Control register0\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR0_SPEC;
impl crate::svd::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::svd::Readable for CR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::svd::Writable for CR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::svd::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
