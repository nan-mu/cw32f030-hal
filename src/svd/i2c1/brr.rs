#[doc = "Register `BRR` reader"]
pub type R = crate::svd::R<BRR_SPEC>;
#[doc = "Register `BRR` writer"]
pub type W = crate::svd::W<BRR_SPEC>;
#[doc = "Field `BRR` reader - fSCL = fPCLK / 8 / (BRR+1)"]
pub type BRR_R = crate::svd::FieldReader;
#[doc = "Field `BRR` writer - fSCL = fPCLK / 8 / (BRR+1)"]
pub type BRR_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - fSCL = fPCLK / 8 / (BRR+1)"]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - fSCL = fPCLK / 8 / (BRR+1)"]
    #[inline(always)]
    #[must_use]
    pub fn brr(&mut self) -> BRR_W<BRR_SPEC, 0> {
        BRR_W::new(self)
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
#[doc = "desc BRR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`brr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRR_SPEC;
impl crate::svd::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::svd::Readable for BRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::svd::Writable for BRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::svd::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
