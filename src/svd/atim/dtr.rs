#[doc = "Register `DTR` reader"]
pub type R = crate::svd::R<DTR_SPEC>;
#[doc = "Register `DTR` writer"]
pub type W = crate::svd::W<DTR_SPEC>;
#[doc = "Field `DTR` reader - desc DTR"]
pub type DTR_R = crate::svd::FieldReader;
#[doc = "Field `DTR` writer - desc DTR"]
pub type DTR_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DTEN` reader - desc DTEN"]
pub type DTEN_R = crate::svd::BitReader;
#[doc = "Field `DTEN` writer - desc DTEN"]
pub type DTEN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BKE` reader - desc BKE"]
pub type BKE_R = crate::svd::BitReader;
#[doc = "Field `BKE` writer - desc BKE"]
pub type BKE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `AOE` reader - desc AOE"]
pub type AOE_R = crate::svd::BitReader;
#[doc = "Field `AOE` writer - desc AOE"]
pub type AOE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `MOE` reader - desc MOE"]
pub type MOE_R = crate::svd::BitReader;
#[doc = "Field `MOE` writer - desc MOE"]
pub type MOE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SAFEEN` reader - desc SAFEEN"]
pub type SAFEEN_R = crate::svd::BitReader;
#[doc = "Field `SAFEEN` writer - desc SAFEEN"]
pub type SAFEEN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `VCE` reader - desc VCE"]
pub type VCE_R = crate::svd::BitReader;
#[doc = "Field `VCE` writer - desc VCE"]
pub type VCE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - desc DTR"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - desc DTEN"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BKE"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc AOE"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc MOE"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SAFEEN"]
    #[inline(always)]
    pub fn safeen(&self) -> SAFEEN_R {
        SAFEEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc VCE"]
    #[inline(always)]
    pub fn vce(&self) -> VCE_R {
        VCE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DTR"]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DTR_W<DTR_SPEC, 0> {
        DTR_W::new(self)
    }
    #[doc = "Bit 9 - desc DTEN"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<DTR_SPEC, 9> {
        DTEN_W::new(self)
    }
    #[doc = "Bit 10 - desc BKE"]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<DTR_SPEC, 10> {
        BKE_W::new(self)
    }
    #[doc = "Bit 11 - desc AOE"]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<DTR_SPEC, 11> {
        AOE_W::new(self)
    }
    #[doc = "Bit 12 - desc MOE"]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<DTR_SPEC, 12> {
        MOE_W::new(self)
    }
    #[doc = "Bit 13 - desc SAFEEN"]
    #[inline(always)]
    #[must_use]
    pub fn safeen(&mut self) -> SAFEEN_W<DTR_SPEC, 13> {
        SAFEEN_W::new(self)
    }
    #[doc = "Bit 14 - desc VCE"]
    #[inline(always)]
    #[must_use]
    pub fn vce(&mut self) -> VCE_W<DTR_SPEC, 14> {
        VCE_W::new(self)
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
#[doc = "desc DTR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`dtr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`dtr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTR_SPEC;
impl crate::svd::RegisterSpec for DTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtr::R`](R) reader structure"]
impl crate::svd::Readable for DTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtr::W`](W) writer structure"]
impl crate::svd::Writable for DTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTR to value 0"]
impl crate::svd::Resettable for DTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
