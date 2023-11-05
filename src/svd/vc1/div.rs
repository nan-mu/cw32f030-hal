#[doc = "Register `DIV` reader"]
pub type R = crate::svd::R<DIV_SPEC>;
#[doc = "Register `DIV` writer"]
pub type W = crate::svd::W<DIV_SPEC>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DIV_R = crate::svd::FieldReader;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DIV_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::svd::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `VIN` reader - desc VIN"]
pub type VIN_R = crate::svd::BitReader;
#[doc = "Field `VIN` writer - desc VIN"]
pub type VIN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc VIN"]
    #[inline(always)]
    pub fn vin(&self) -> VIN_R {
        VIN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc DIV"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<DIV_SPEC, 0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DIV_SPEC, 6> {
        EN_W::new(self)
    }
    #[doc = "Bit 7 - desc VIN"]
    #[inline(always)]
    #[must_use]
    pub fn vin(&mut self) -> VIN_W<DIV_SPEC, 7> {
        VIN_W::new(self)
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
#[doc = "desc DIV\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`div::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SPEC;
impl crate::svd::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::svd::Readable for DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::svd::Writable for DIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::svd::Resettable for DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
