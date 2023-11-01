#[doc = "Register `SSI` reader"]
pub type R = crate::R<SSI_SPEC>;
#[doc = "Register `SSI` writer"]
pub type W = crate::W<SSI_SPEC>;
#[doc = "Field `SSI` reader - desc SSI"]
pub type SSI_R = crate::BitReader;
#[doc = "Field `SSI` writer - desc SSI"]
pub type SSI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc SSI"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SSI"]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<SSI_SPEC, 0> {
        SSI_W::new(self)
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
#[doc = "Slave slect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSI_SPEC;
impl crate::RegisterSpec for SSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssi::R`](R) reader structure"]
impl crate::Readable for SSI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssi::W`](W) writer structure"]
impl crate::Writable for SSI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSI to value 0"]
impl crate::Resettable for SSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
