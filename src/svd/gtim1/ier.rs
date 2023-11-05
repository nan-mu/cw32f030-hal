#[doc = "Register `IER` reader"]
pub type R = crate::svd::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::svd::W<IER_SPEC>;
#[doc = "Field `OV` reader - desc OV"]
pub type OV_R = crate::svd::BitReader;
#[doc = "Field `OV` writer - desc OV"]
pub type OV_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TI` reader - desc TI"]
pub type TI_R = crate::svd::BitReader;
#[doc = "Field `TI` writer - desc TI"]
pub type TI_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `UD` reader - desc UD"]
pub type UD_R = crate::svd::BitReader;
#[doc = "Field `UD` writer - desc UD"]
pub type UD_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CC1` reader - desc CC1"]
pub type CC1_R = crate::svd::BitReader;
#[doc = "Field `CC1` writer - desc CC1"]
pub type CC1_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CC2` reader - desc CC2"]
pub type CC2_R = crate::svd::BitReader;
#[doc = "Field `CC2` writer - desc CC2"]
pub type CC2_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CC3` reader - desc CC3"]
pub type CC3_R = crate::svd::BitReader;
#[doc = "Field `CC3` writer - desc CC3"]
pub type CC3_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CC4` reader - desc CC4"]
pub type CC4_R = crate::svd::BitReader;
#[doc = "Field `CC4` writer - desc CC4"]
pub type CC4_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `DIRCHANGE` reader - desc DIRCHANGE"]
pub type DIRCHANGE_R = crate::svd::BitReader;
#[doc = "Field `DIRCHANGE` writer - desc DIRCHANGE"]
pub type DIRCHANGE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TI"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc UD"]
    #[inline(always)]
    pub fn ud(&self) -> UD_R {
        UD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CC1"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CC2"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CC3"]
    #[inline(always)]
    pub fn cc3(&self) -> CC3_R {
        CC3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CC4"]
    #[inline(always)]
    pub fn cc4(&self) -> CC4_R {
        CC4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DIRCHANGE"]
    #[inline(always)]
    pub fn dirchange(&self) -> DIRCHANGE_R {
        DIRCHANGE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OV"]
    #[inline(always)]
    #[must_use]
    pub fn ov(&mut self) -> OV_W<IER_SPEC, 0> {
        OV_W::new(self)
    }
    #[doc = "Bit 1 - desc TI"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<IER_SPEC, 1> {
        TI_W::new(self)
    }
    #[doc = "Bit 2 - desc UD"]
    #[inline(always)]
    #[must_use]
    pub fn ud(&mut self) -> UD_W<IER_SPEC, 2> {
        UD_W::new(self)
    }
    #[doc = "Bit 3 - desc CC1"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> CC1_W<IER_SPEC, 3> {
        CC1_W::new(self)
    }
    #[doc = "Bit 4 - desc CC2"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> CC2_W<IER_SPEC, 4> {
        CC2_W::new(self)
    }
    #[doc = "Bit 5 - desc CC3"]
    #[inline(always)]
    #[must_use]
    pub fn cc3(&mut self) -> CC3_W<IER_SPEC, 5> {
        CC3_W::new(self)
    }
    #[doc = "Bit 6 - desc CC4"]
    #[inline(always)]
    #[must_use]
    pub fn cc4(&mut self) -> CC4_W<IER_SPEC, 6> {
        CC4_W::new(self)
    }
    #[doc = "Bit 9 - desc DIRCHANGE"]
    #[inline(always)]
    #[must_use]
    pub fn dirchange(&mut self) -> DIRCHANGE_W<IER_SPEC, 9> {
        DIRCHANGE_W::new(self)
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
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::svd::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::svd::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::svd::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::svd::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
