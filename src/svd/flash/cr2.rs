#[doc = "Register `CR2` reader"]
pub type R = crate::svd::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::svd::W<CR2_SPEC>;
#[doc = "Field `WAIT` reader - desc WAIT"]
pub type WAIT_R = crate::svd::FieldReader;
#[doc = "Field `WAIT` writer - desc WAIT"]
pub type WAIT_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `FETCH` reader - desc FETCH"]
pub type FETCH_R = crate::svd::BitReader;
#[doc = "Field `FETCH` writer - desc FETCH"]
pub type FETCH_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CACHE` reader - desc CACHE"]
pub type CACHE_R = crate::svd::BitReader;
#[doc = "Field `CACHE` writer - desc CACHE"]
pub type CACHE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KEY_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:2 - desc WAIT"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc FETCH"]
    #[inline(always)]
    pub fn fetch(&self) -> FETCH_R {
        FETCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CACHE"]
    #[inline(always)]
    pub fn cache(&self) -> CACHE_R {
        CACHE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc WAIT"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<CR2_SPEC, 0> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 3 - desc FETCH"]
    #[inline(always)]
    #[must_use]
    pub fn fetch(&mut self) -> FETCH_W<CR2_SPEC, 3> {
        FETCH_W::new(self)
    }
    #[doc = "Bit 4 - desc CACHE"]
    #[inline(always)]
    #[must_use]
    pub fn cache(&mut self) -> CACHE_W<CR2_SPEC, 4> {
        CACHE_W::new(self)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CR2_SPEC, 16> {
        KEY_W::new(self)
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
