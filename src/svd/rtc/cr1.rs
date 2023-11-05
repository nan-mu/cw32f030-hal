#[doc = "Register `CR1` reader"]
pub type R = crate::svd::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::svd::W<CR1_SPEC>;
#[doc = "Field `ACCESS` reader - desc ACCESS"]
pub type ACCESS_R = crate::svd::BitReader;
#[doc = "Field `ACCESS` writer - desc ACCESS"]
pub type ACCESS_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `WINDOW` reader - desc WINDOW"]
pub type WINDOW_R = crate::svd::BitReader;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SOURCE_R = crate::svd::FieldReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SOURCE_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - desc ACCESS"]
    #[inline(always)]
    pub fn access(&self) -> ACCESS_R {
        ACCESS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WINDOW"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc ACCESS"]
    #[inline(always)]
    #[must_use]
    pub fn access(&mut self) -> ACCESS_W<CR1_SPEC, 0> {
        ACCESS_W::new(self)
    }
    #[doc = "Bits 8:10 - desc SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<CR1_SPEC, 8> {
        SOURCE_W::new(self)
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
#[doc = "Control register1\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::svd::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::svd::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::svd::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::svd::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
