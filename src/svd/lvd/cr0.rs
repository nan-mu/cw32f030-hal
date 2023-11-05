#[doc = "Register `CR0` reader"]
pub type R = crate::svd::R<CR0_SPEC>;
#[doc = "Register `CR0` writer"]
pub type W = crate::svd::W<CR0_SPEC>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::svd::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `ACTION` reader - desc ACTION"]
pub type ACTION_R = crate::svd::BitReader;
#[doc = "Field `ACTION` writer - desc ACTION"]
pub type ACTION_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SOURCE_R = crate::svd::FieldReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SOURCE_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `VTH` reader - desc VTH"]
pub type VTH_R = crate::svd::FieldReader;
#[doc = "Field `VTH` writer - desc VTH"]
pub type VTH_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `IE` reader - desc IE"]
pub type IE_R = crate::svd::BitReader;
#[doc = "Field `IE` writer - desc IE"]
pub type IE_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ACTION"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc VTH"]
    #[inline(always)]
    pub fn vth(&self) -> VTH_R {
        VTH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CR0_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - desc ACTION"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<CR0_SPEC, 1> {
        ACTION_W::new(self)
    }
    #[doc = "Bits 2:3 - desc SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<CR0_SPEC, 2> {
        SOURCE_W::new(self)
    }
    #[doc = "Bits 4:7 - desc VTH"]
    #[inline(always)]
    #[must_use]
    pub fn vth(&mut self) -> VTH_W<CR0_SPEC, 4> {
        VTH_W::new(self)
    }
    #[doc = "Bit 9 - desc IE"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CR0_SPEC, 9> {
        IE_W::new(self)
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
#[doc = "Control register0\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR0_SPEC;
impl crate::svd::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::svd::Readable for CR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::svd::Writable for CR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::svd::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
