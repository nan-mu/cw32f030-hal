#[doc = "Register `CR2` reader"]
pub type R = crate::svd::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::svd::W<CR2_SPEC>;
#[doc = "Field `HDOE` reader - desc HDOE"]
pub type HDOE_R = crate::svd::BitReader;
#[doc = "Field `HDOE` writer - desc HDOE"]
pub type HDOE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc HDOE"]
    #[inline(always)]
    pub fn hdoe(&self) -> HDOE_R {
        HDOE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HDOE"]
    #[inline(always)]
    #[must_use]
    pub fn hdoe(&mut self) -> HDOE_W<CR2_SPEC, 0> {
        HDOE_W::new(self)
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
#[doc = "Control register2\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::svd::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::svd::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::svd::Writable for CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::svd::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
