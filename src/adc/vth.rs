#[doc = "Register `VTH` reader"]
pub type R = crate::R<VTH_SPEC>;
#[doc = "Register `VTH` writer"]
pub type W = crate::W<VTH_SPEC>;
#[doc = "Field `VTH` reader - desc VTH"]
pub type VTH_R = crate::FieldReader<u16>;
#[doc = "Field `VTH` writer - desc VTH"]
pub type VTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - desc VTH"]
    #[inline(always)]
    pub fn vth(&self) -> VTH_R {
        VTH_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc VTH"]
    #[inline(always)]
    #[must_use]
    pub fn vth(&mut self) -> VTH_W<VTH_SPEC, 0> {
        VTH_W::new(self)
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
#[doc = "desc VTH\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VTH_SPEC;
impl crate::RegisterSpec for VTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vth::R`](R) reader structure"]
impl crate::Readable for VTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vth::W`](W) writer structure"]
impl crate::Writable for VTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTH to value 0"]
impl crate::Resettable for VTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
