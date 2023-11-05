#[doc = "Register `ICR` reader"]
pub type R = crate::svd::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::svd::W<ICR_SPEC>;
#[doc = "Field `UIF` reader - desc UIF"]
pub type UIF_R = crate::svd::BitReader;
#[doc = "Field `UIF` writer - desc UIF"]
pub type UIF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C1AF` reader - desc C1AF"]
pub type C1AF_R = crate::svd::BitReader;
#[doc = "Field `C1AF` writer - desc C1AF"]
pub type C1AF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C2AF` reader - desc C2AF"]
pub type C2AF_R = crate::svd::BitReader;
#[doc = "Field `C2AF` writer - desc C2AF"]
pub type C2AF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C3AF` reader - desc C3AF"]
pub type C3AF_R = crate::svd::BitReader;
#[doc = "Field `C3AF` writer - desc C3AF"]
pub type C3AF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C1BF` reader - desc C1BF"]
pub type C1BF_R = crate::svd::BitReader;
#[doc = "Field `C1BF` writer - desc C1BF"]
pub type C1BF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C2BF` reader - desc C2BF"]
pub type C2BF_R = crate::svd::BitReader;
#[doc = "Field `C2BF` writer - desc C2BF"]
pub type C2BF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C3BF` reader - desc C3BF"]
pub type C3BF_R = crate::svd::BitReader;
#[doc = "Field `C3BF` writer - desc C3BF"]
pub type C3BF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C1AE` reader - desc C1AE"]
pub type C1AE_R = crate::svd::BitReader;
#[doc = "Field `C1AE` writer - desc C1AE"]
pub type C1AE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C2AE` reader - desc C2AE"]
pub type C2AE_R = crate::svd::BitReader;
#[doc = "Field `C2AE` writer - desc C2AE"]
pub type C2AE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C3AE` reader - desc C3AE"]
pub type C3AE_R = crate::svd::BitReader;
#[doc = "Field `C3AE` writer - desc C3AE"]
pub type C3AE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C1BE` reader - desc C1BE"]
pub type C1BE_R = crate::svd::BitReader;
#[doc = "Field `C1BE` writer - desc C1BE"]
pub type C1BE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C2BE` reader - desc C2BE"]
pub type C2BE_R = crate::svd::BitReader;
#[doc = "Field `C2BE` writer - desc C2BE"]
pub type C2BE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C3BE` reader - desc C3BE"]
pub type C3BE_R = crate::svd::BitReader;
#[doc = "Field `C3BE` writer - desc C3BE"]
pub type C3BE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BIF` reader - desc BIF"]
pub type BIF_R = crate::svd::BitReader;
#[doc = "Field `BIF` writer - desc BIF"]
pub type BIF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TIF` reader - desc TIF"]
pub type TIF_R = crate::svd::BitReader;
#[doc = "Field `TIF` writer - desc TIF"]
pub type TIF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `OVF` reader - desc OVF"]
pub type OVF_R = crate::svd::BitReader;
#[doc = "Field `OVF` writer - desc OVF"]
pub type OVF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `UNDF` reader - desc UNDF"]
pub type UNDF_R = crate::svd::BitReader;
#[doc = "Field `UNDF` writer - desc UNDF"]
pub type UNDF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `C4AF` reader - desc C4AF"]
pub type C4AF_R = crate::svd::BitReader;
#[doc = "Field `C4AF` writer - desc C4AF"]
pub type C4AF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc UIF"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc C1AF"]
    #[inline(always)]
    pub fn c1af(&self) -> C1AF_R {
        C1AF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc C2AF"]
    #[inline(always)]
    pub fn c2af(&self) -> C2AF_R {
        C2AF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc C3AF"]
    #[inline(always)]
    pub fn c3af(&self) -> C3AF_R {
        C3AF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc C1BF"]
    #[inline(always)]
    pub fn c1bf(&self) -> C1BF_R {
        C1BF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc C2BF"]
    #[inline(always)]
    pub fn c2bf(&self) -> C2BF_R {
        C2BF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc C3BF"]
    #[inline(always)]
    pub fn c3bf(&self) -> C3BF_R {
        C3BF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc C1AE"]
    #[inline(always)]
    pub fn c1ae(&self) -> C1AE_R {
        C1AE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc C2AE"]
    #[inline(always)]
    pub fn c2ae(&self) -> C2AE_R {
        C2AE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc C3AE"]
    #[inline(always)]
    pub fn c3ae(&self) -> C3AE_R {
        C3AE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc C1BE"]
    #[inline(always)]
    pub fn c1be(&self) -> C1BE_R {
        C1BE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc C2BE"]
    #[inline(always)]
    pub fn c2be(&self) -> C2BE_R {
        C2BE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc C3BE"]
    #[inline(always)]
    pub fn c3be(&self) -> C3BE_R {
        C3BE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc BIF"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TIF"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc OVF"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc UNDF"]
    #[inline(always)]
    pub fn undf(&self) -> UNDF_R {
        UNDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc C4AF"]
    #[inline(always)]
    pub fn c4af(&self) -> C4AF_R {
        C4AF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UIF"]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<ICR_SPEC, 0> {
        UIF_W::new(self)
    }
    #[doc = "Bit 2 - desc C1AF"]
    #[inline(always)]
    #[must_use]
    pub fn c1af(&mut self) -> C1AF_W<ICR_SPEC, 2> {
        C1AF_W::new(self)
    }
    #[doc = "Bit 3 - desc C2AF"]
    #[inline(always)]
    #[must_use]
    pub fn c2af(&mut self) -> C2AF_W<ICR_SPEC, 3> {
        C2AF_W::new(self)
    }
    #[doc = "Bit 4 - desc C3AF"]
    #[inline(always)]
    #[must_use]
    pub fn c3af(&mut self) -> C3AF_W<ICR_SPEC, 4> {
        C3AF_W::new(self)
    }
    #[doc = "Bit 5 - desc C1BF"]
    #[inline(always)]
    #[must_use]
    pub fn c1bf(&mut self) -> C1BF_W<ICR_SPEC, 5> {
        C1BF_W::new(self)
    }
    #[doc = "Bit 6 - desc C2BF"]
    #[inline(always)]
    #[must_use]
    pub fn c2bf(&mut self) -> C2BF_W<ICR_SPEC, 6> {
        C2BF_W::new(self)
    }
    #[doc = "Bit 7 - desc C3BF"]
    #[inline(always)]
    #[must_use]
    pub fn c3bf(&mut self) -> C3BF_W<ICR_SPEC, 7> {
        C3BF_W::new(self)
    }
    #[doc = "Bit 8 - desc C1AE"]
    #[inline(always)]
    #[must_use]
    pub fn c1ae(&mut self) -> C1AE_W<ICR_SPEC, 8> {
        C1AE_W::new(self)
    }
    #[doc = "Bit 9 - desc C2AE"]
    #[inline(always)]
    #[must_use]
    pub fn c2ae(&mut self) -> C2AE_W<ICR_SPEC, 9> {
        C2AE_W::new(self)
    }
    #[doc = "Bit 10 - desc C3AE"]
    #[inline(always)]
    #[must_use]
    pub fn c3ae(&mut self) -> C3AE_W<ICR_SPEC, 10> {
        C3AE_W::new(self)
    }
    #[doc = "Bit 11 - desc C1BE"]
    #[inline(always)]
    #[must_use]
    pub fn c1be(&mut self) -> C1BE_W<ICR_SPEC, 11> {
        C1BE_W::new(self)
    }
    #[doc = "Bit 12 - desc C2BE"]
    #[inline(always)]
    #[must_use]
    pub fn c2be(&mut self) -> C2BE_W<ICR_SPEC, 12> {
        C2BE_W::new(self)
    }
    #[doc = "Bit 13 - desc C3BE"]
    #[inline(always)]
    #[must_use]
    pub fn c3be(&mut self) -> C3BE_W<ICR_SPEC, 13> {
        C3BE_W::new(self)
    }
    #[doc = "Bit 14 - desc BIF"]
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<ICR_SPEC, 14> {
        BIF_W::new(self)
    }
    #[doc = "Bit 15 - desc TIF"]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<ICR_SPEC, 15> {
        TIF_W::new(self)
    }
    #[doc = "Bit 16 - desc OVF"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<ICR_SPEC, 16> {
        OVF_W::new(self)
    }
    #[doc = "Bit 17 - desc UNDF"]
    #[inline(always)]
    #[must_use]
    pub fn undf(&mut self) -> UNDF_W<ICR_SPEC, 17> {
        UNDF_W::new(self)
    }
    #[doc = "Bit 18 - desc C4AF"]
    #[inline(always)]
    #[must_use]
    pub fn c4af(&mut self) -> C4AF_W<ICR_SPEC, 18> {
        C4AF_W::new(self)
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
#[doc = "desc ICR\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
