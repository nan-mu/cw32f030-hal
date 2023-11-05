#[doc = "Register `ADDR1` reader"]
pub type R = crate::svd::R<ADDR1_SPEC>;
#[doc = "Register `ADDR1` writer"]
pub type W = crate::svd::W<ADDR1_SPEC>;
#[doc = "Field `ADDR1` reader - desc ADDR1"]
pub type ADDR1_R = crate::svd::FieldReader;
#[doc = "Field `ADDR1` writer - desc ADDR1"]
pub type ADDR1_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 1:7 - desc ADDR1"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - desc ADDR1"]
    #[inline(always)]
    #[must_use]
    pub fn addr1(&mut self) -> ADDR1_W<ADDR1_SPEC, 1> {
        ADDR1_W::new(self)
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
#[doc = "Slave Addrress1\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`addr1::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`addr1::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR1_SPEC;
impl crate::svd::RegisterSpec for ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr1::R`](R) reader structure"]
impl crate::svd::Readable for ADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr1::W`](W) writer structure"]
impl crate::svd::Writable for ADDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR1 to value 0"]
impl crate::svd::Resettable for ADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
