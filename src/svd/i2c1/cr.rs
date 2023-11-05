#[doc = "Register `CR` reader"]
pub type R = crate::svd::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::svd::W<CR_SPEC>;
#[doc = "Field `FLT` reader - desc FLT"]
pub type FLT_R = crate::svd::BitReader;
#[doc = "Field `FLT` writer - desc FLT"]
pub type FLT_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `AA` reader - desc AA"]
pub type AA_R = crate::svd::BitReader;
#[doc = "Field `AA` writer - desc AA"]
pub type AA_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SI` reader - desc SI"]
pub type SI_R = crate::svd::BitReader;
#[doc = "Field `SI` writer - desc SI"]
pub type SI_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `STO` reader - desc STO"]
pub type STO_R = crate::svd::BitReader;
#[doc = "Field `STO` writer - desc STO"]
pub type STO_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `STA` reader - desc STA"]
pub type STA_R = crate::svd::BitReader;
#[doc = "Field `STA` writer - desc STA"]
pub type STA_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::svd::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc FLT"]
    #[inline(always)]
    pub fn flt(&self) -> FLT_R {
        FLT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc AA"]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SI"]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc STO"]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc STA"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc FLT"]
    #[inline(always)]
    #[must_use]
    pub fn flt(&mut self) -> FLT_W<CR_SPEC, 0> {
        FLT_W::new(self)
    }
    #[doc = "Bit 2 - desc AA"]
    #[inline(always)]
    #[must_use]
    pub fn aa(&mut self) -> AA_W<CR_SPEC, 2> {
        AA_W::new(self)
    }
    #[doc = "Bit 3 - desc SI"]
    #[inline(always)]
    #[must_use]
    pub fn si(&mut self) -> SI_W<CR_SPEC, 3> {
        SI_W::new(self)
    }
    #[doc = "Bit 4 - desc STO"]
    #[inline(always)]
    #[must_use]
    pub fn sto(&mut self) -> STO_W<CR_SPEC, 4> {
        STO_W::new(self)
    }
    #[doc = "Bit 5 - desc STA"]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> STA_W<CR_SPEC, 5> {
        STA_W::new(self)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CR_SPEC, 6> {
        EN_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::svd::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::svd::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::svd::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::svd::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
