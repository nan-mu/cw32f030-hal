#[doc = "Register `ICR` reader"]
pub type R = crate::svd::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::svd::W<ICR_SPEC>;
#[doc = "Field `HSIRDY` reader - desc HSIRDY"]
pub type HSIRDY_R = crate::svd::BitReader;
#[doc = "Field `HSIRDY` writer - desc HSIRDY"]
pub type HSIRDY_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `HSERDY` reader - desc HSERDY"]
pub type HSERDY_R = crate::svd::BitReader;
#[doc = "Field `HSERDY` writer - desc HSERDY"]
pub type HSERDY_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PLLRDY` reader - desc PLLRDY"]
pub type PLLRDY_R = crate::svd::BitReader;
#[doc = "Field `PLLRDY` writer - desc PLLRDY"]
pub type PLLRDY_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LSIRDY` reader - desc LSIRDY"]
pub type LSIRDY_R = crate::svd::BitReader;
#[doc = "Field `LSIRDY` writer - desc LSIRDY"]
pub type LSIRDY_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LSERDY` reader - desc LSERDY"]
pub type LSERDY_R = crate::svd::BitReader;
#[doc = "Field `LSERDY` writer - desc LSERDY"]
pub type LSERDY_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LSEFAIL` reader - desc LSEFAIL"]
pub type LSEFAIL_R = crate::svd::BitReader;
#[doc = "Field `LSEFAIL` writer - desc LSEFAIL"]
pub type LSEFAIL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `HSEFAIL` reader - desc HSEFAIL"]
pub type HSEFAIL_R = crate::svd::BitReader;
#[doc = "Field `HSEFAIL` writer - desc HSEFAIL"]
pub type HSEFAIL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LSEFAULT` reader - desc LSEFAULT"]
pub type LSEFAULT_R = crate::svd::BitReader;
#[doc = "Field `LSEFAULT` writer - desc LSEFAULT"]
pub type LSEFAULT_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `HSEFAULT` reader - desc HSEFAULT"]
pub type HSEFAULT_R = crate::svd::BitReader;
#[doc = "Field `HSEFAULT` writer - desc HSEFAULT"]
pub type HSEFAULT_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSERDY"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PLLRDY"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc LSERDY"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc LSEFAIL"]
    #[inline(always)]
    pub fn lsefail(&self) -> LSEFAIL_R {
        LSEFAIL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HSEFAIL"]
    #[inline(always)]
    pub fn hsefail(&self) -> HSEFAIL_R {
        HSEFAIL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc LSEFAULT"]
    #[inline(always)]
    pub fn lsefault(&self) -> LSEFAULT_R {
        LSEFAULT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HSEFAULT"]
    #[inline(always)]
    pub fn hsefault(&self) -> HSEFAULT_R {
        HSEFAULT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HSIRDY"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdy(&mut self) -> HSIRDY_W<ICR_SPEC, 0> {
        HSIRDY_W::new(self)
    }
    #[doc = "Bit 1 - desc HSERDY"]
    #[inline(always)]
    #[must_use]
    pub fn hserdy(&mut self) -> HSERDY_W<ICR_SPEC, 1> {
        HSERDY_W::new(self)
    }
    #[doc = "Bit 2 - desc PLLRDY"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdy(&mut self) -> PLLRDY_W<ICR_SPEC, 2> {
        PLLRDY_W::new(self)
    }
    #[doc = "Bit 3 - desc LSIRDY"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdy(&mut self) -> LSIRDY_W<ICR_SPEC, 3> {
        LSIRDY_W::new(self)
    }
    #[doc = "Bit 4 - desc LSERDY"]
    #[inline(always)]
    #[must_use]
    pub fn lserdy(&mut self) -> LSERDY_W<ICR_SPEC, 4> {
        LSERDY_W::new(self)
    }
    #[doc = "Bit 5 - desc LSEFAIL"]
    #[inline(always)]
    #[must_use]
    pub fn lsefail(&mut self) -> LSEFAIL_W<ICR_SPEC, 5> {
        LSEFAIL_W::new(self)
    }
    #[doc = "Bit 6 - desc HSEFAIL"]
    #[inline(always)]
    #[must_use]
    pub fn hsefail(&mut self) -> HSEFAIL_W<ICR_SPEC, 6> {
        HSEFAIL_W::new(self)
    }
    #[doc = "Bit 7 - desc LSEFAULT"]
    #[inline(always)]
    #[must_use]
    pub fn lsefault(&mut self) -> LSEFAULT_W<ICR_SPEC, 7> {
        LSEFAULT_W::new(self)
    }
    #[doc = "Bit 8 - desc HSEFAULT"]
    #[inline(always)]
    #[must_use]
    pub fn hsefault(&mut self) -> HSEFAULT_W<ICR_SPEC, 8> {
        HSEFAULT_W::new(self)
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
#[doc = "Interupt Clear Reg\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::svd::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::svd::Readable for ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::svd::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::svd::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
