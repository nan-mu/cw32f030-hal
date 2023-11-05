#[doc = "Register `CR2` reader"]
pub type R = crate::svd::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::svd::W<CR2_SPEC>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CNT_R = crate::svd::FieldReader;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CNT_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ACCEN` reader - desc ACCEN"]
pub type ACCEN_R = crate::svd::BitReader;
#[doc = "Field `ACCEN` writer - desc ACCEN"]
pub type ACCEN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `ACCRST` reader - desc ACCRST"]
pub type ACCRST_R = crate::svd::BitReader;
#[doc = "Field `ACCRST` writer - desc ACCRST"]
pub type ACCRST_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - desc ACCEN"]
    #[inline(always)]
    pub fn accen(&self) -> ACCEN_R {
        ACCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ACCRST"]
    #[inline(always)]
    pub fn accrst(&self) -> ACCRST_R {
        ACCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc CNT"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CR2_SPEC, 0> {
        CNT_W::new(self)
    }
    #[doc = "Bit 8 - desc ACCEN"]
    #[inline(always)]
    #[must_use]
    pub fn accen(&mut self) -> ACCEN_W<CR2_SPEC, 8> {
        ACCEN_W::new(self)
    }
    #[doc = "Bit 9 - desc ACCRST"]
    #[inline(always)]
    #[must_use]
    pub fn accrst(&mut self) -> ACCRST_W<CR2_SPEC, 9> {
        ACCRST_W::new(self)
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
