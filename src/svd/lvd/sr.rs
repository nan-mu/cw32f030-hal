#[doc = "Register `SR` reader"]
pub type R = crate::svd::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::svd::W<SR_SPEC>;
#[doc = "Field `INTF` reader - desc INTF"]
pub type INTF_R = crate::svd::BitReader;
#[doc = "Field `INTF` writer - desc INTF"]
pub type INTF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `FLTV` reader - desc FLTV"]
pub type FLTV_R = crate::svd::BitReader;
impl R {
    #[doc = "Bit 0 - desc INTF"]
    #[inline(always)]
    pub fn intf(&self) -> INTF_R {
        INTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FLTV"]
    #[inline(always)]
    pub fn fltv(&self) -> FLTV_R {
        FLTV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc INTF"]
    #[inline(always)]
    #[must_use]
    pub fn intf(&mut self) -> INTF_W<SR_SPEC, 0> {
        INTF_W::new(self)
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
#[doc = "status register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::svd::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::svd::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::svd::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::svd::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
