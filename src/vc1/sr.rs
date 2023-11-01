#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `INTF` reader - desc INTF"]
pub type INTF_R = crate::BitReader;
#[doc = "Field `INTF` writer - desc INTF"]
pub type INTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLTV` reader - desc FLTV"]
pub type FLTV_R = crate::BitReader;
#[doc = "Field `READY` reader - desc READY"]
pub type READY_R = crate::BitReader;
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
    #[doc = "Bit 2 - desc READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 2) & 1) != 0)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
