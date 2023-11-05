#[doc = "Register `ADDR0` reader"]
pub type R = crate::svd::R<ADDR0_SPEC>;
#[doc = "Register `ADDR0` writer"]
pub type W = crate::svd::W<ADDR0_SPEC>;
#[doc = "Field `GC` reader - desc GC"]
pub type GC_R = crate::svd::BitReader;
#[doc = "Field `GC` writer - desc GC"]
pub type GC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `ADDR0` reader - desc ADDR0"]
pub type ADDR0_R = crate::svd::FieldReader;
#[doc = "Field `ADDR0` writer - desc ADDR0"]
pub type ADDR0_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bit 0 - desc GC"]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - desc ADDR0"]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc GC"]
    #[inline(always)]
    #[must_use]
    pub fn gc(&mut self) -> GC_W<ADDR0_SPEC, 0> {
        GC_W::new(self)
    }
    #[doc = "Bits 1:7 - desc ADDR0"]
    #[inline(always)]
    #[must_use]
    pub fn addr0(&mut self) -> ADDR0_W<ADDR0_SPEC, 1> {
        ADDR0_W::new(self)
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
#[doc = "Slave Addrress0\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`addr0::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`addr0::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR0_SPEC;
impl crate::svd::RegisterSpec for ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr0::R`](R) reader structure"]
impl crate::svd::Readable for ADDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr0::W`](W) writer structure"]
impl crate::svd::Writable for ADDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR0 to value 0"]
impl crate::svd::Resettable for ADDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
