#[doc = "Register `ICR` reader"]
pub type R = crate::svd::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::svd::W<ICR_SPEC>;
#[doc = "Field `PC` reader - desc PC"]
pub type PC_R = crate::svd::BitReader;
#[doc = "Field `PC` writer - desc PC"]
pub type PC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PAGELOCK` reader - desc PAGELOCK"]
pub type PAGELOCK_R = crate::svd::BitReader;
#[doc = "Field `PAGELOCK` writer - desc PAGELOCK"]
pub type PAGELOCK_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PROG` reader - desc PROG"]
pub type PROG_R = crate::svd::BitReader;
#[doc = "Field `PROG` writer - desc PROG"]
pub type PROG_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc PC"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PAGELOCK"]
    #[inline(always)]
    pub fn pagelock(&self) -> PAGELOCK_R {
        PAGELOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PROG"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PC"]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PC_W<ICR_SPEC, 0> {
        PC_W::new(self)
    }
    #[doc = "Bit 1 - desc PAGELOCK"]
    #[inline(always)]
    #[must_use]
    pub fn pagelock(&mut self) -> PAGELOCK_W<ICR_SPEC, 1> {
        PAGELOCK_W::new(self)
    }
    #[doc = "Bit 4 - desc PROG"]
    #[inline(always)]
    #[must_use]
    pub fn prog(&mut self) -> PROG_W<ICR_SPEC, 4> {
        PROG_W::new(self)
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
#[doc = "Interrupt flag clear register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
