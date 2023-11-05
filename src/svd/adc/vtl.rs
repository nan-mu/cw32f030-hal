#[doc = "Register `VTL` reader"]
pub type R = crate::svd::R<VTL_SPEC>;
#[doc = "Register `VTL` writer"]
pub type W = crate::svd::W<VTL_SPEC>;
#[doc = "Field `VTL` reader - desc VTL"]
pub type VTL_R = crate::svd::FieldReader<u16>;
#[doc = "Field `VTL` writer - desc VTL"]
pub type VTL_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - desc VTL"]
    #[inline(always)]
    pub fn vtl(&self) -> VTL_R {
        VTL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc VTL"]
    #[inline(always)]
    #[must_use]
    pub fn vtl(&mut self) -> VTL_W<VTL_SPEC, 0> {
        VTL_W::new(self)
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
#[doc = "desc VTL\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`vtl::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`vtl::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VTL_SPEC;
impl crate::svd::RegisterSpec for VTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtl::R`](R) reader structure"]
impl crate::svd::Readable for VTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vtl::W`](W) writer structure"]
impl crate::svd::Writable for VTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTL to value 0"]
impl crate::svd::Resettable for VTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
