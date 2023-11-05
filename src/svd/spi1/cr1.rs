#[doc = "Register `CR1` reader"]
pub type R = crate::svd::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::svd::W<CR1_SPEC>;
#[doc = "Field `CPHA` reader - desc CPHA"]
pub type CPHA_R = crate::svd::BitReader;
#[doc = "Field `CPHA` writer - desc CPHA"]
pub type CPHA_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `CPOL` reader - desc CPOL"]
pub type CPOL_R = crate::svd::BitReader;
#[doc = "Field `CPOL` writer - desc CPOL"]
pub type CPOL_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `MSTR` reader - desc MSTR"]
pub type MSTR_R = crate::svd::BitReader;
#[doc = "Field `MSTR` writer - desc MSTR"]
pub type MSTR_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BR` reader - desc BR"]
pub type BR_R = crate::svd::FieldReader;
#[doc = "Field `BR` writer - desc BR"]
pub type BR_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::svd::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `LSBF` reader - desc LSBF"]
pub type LSBF_R = crate::svd::BitReader;
#[doc = "Field `LSBF` writer - desc LSBF"]
pub type LSBF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SMP` reader - desc SMP"]
pub type SMP_R = crate::svd::BitReader;
#[doc = "Field `SMP` writer - desc SMP"]
pub type SMP_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `SSM` reader - desc SSM"]
pub type SSM_R = crate::svd::BitReader;
#[doc = "Field `SSM` writer - desc SSM"]
pub type SSM_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `WIDTH` reader - desc WIDTH"]
pub type WIDTH_R = crate::svd::FieldReader;
#[doc = "Field `WIDTH` writer - desc WIDTH"]
pub type WIDTH_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::svd::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DMARX` reader - desc DMARX"]
pub type DMARX_R = crate::svd::BitReader;
#[doc = "Field `DMARX` writer - desc DMARX"]
pub type DMARX_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `DMATX` reader - desc DMATX"]
pub type DMATX_R = crate::svd::BitReader;
#[doc = "Field `DMATX` writer - desc DMATX"]
pub type DMATX_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `MISOHD` reader - desc MISOHD"]
pub type MISOHD_R = crate::svd::BitReader;
#[doc = "Field `MISOHD` writer - desc MISOHD"]
pub type MISOHD_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc CPHA"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CPOL"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc MSTR"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - desc BR"]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc LSBF"]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SMP"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SSM"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - desc WIDTH"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - desc DMARX"]
    #[inline(always)]
    pub fn dmarx(&self) -> DMARX_R {
        DMARX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc DMATX"]
    #[inline(always)]
    pub fn dmatx(&self) -> DMATX_R {
        DMATX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc MISOHD"]
    #[inline(always)]
    pub fn misohd(&self) -> MISOHD_R {
        MISOHD_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CPHA"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CR1_SPEC, 0> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 1 - desc CPOL"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CR1_SPEC, 1> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 2 - desc MSTR"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<CR1_SPEC, 2> {
        MSTR_W::new(self)
    }
    #[doc = "Bits 3:5 - desc BR"]
    #[inline(always)]
    #[must_use]
    pub fn br(&mut self) -> BR_W<CR1_SPEC, 3> {
        BR_W::new(self)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CR1_SPEC, 6> {
        EN_W::new(self)
    }
    #[doc = "Bit 7 - desc LSBF"]
    #[inline(always)]
    #[must_use]
    pub fn lsbf(&mut self) -> LSBF_W<CR1_SPEC, 7> {
        LSBF_W::new(self)
    }
    #[doc = "Bit 8 - desc SMP"]
    #[inline(always)]
    #[must_use]
    pub fn smp(&mut self) -> SMP_W<CR1_SPEC, 8> {
        SMP_W::new(self)
    }
    #[doc = "Bit 9 - desc SSM"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<CR1_SPEC, 9> {
        SSM_W::new(self)
    }
    #[doc = "Bits 10:13 - desc WIDTH"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<CR1_SPEC, 10> {
        WIDTH_W::new(self)
    }
    #[doc = "Bits 14:15 - desc MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CR1_SPEC, 14> {
        MODE_W::new(self)
    }
    #[doc = "Bit 16 - desc DMARX"]
    #[inline(always)]
    #[must_use]
    pub fn dmarx(&mut self) -> DMARX_W<CR1_SPEC, 16> {
        DMARX_W::new(self)
    }
    #[doc = "Bit 17 - desc DMATX"]
    #[inline(always)]
    #[must_use]
    pub fn dmatx(&mut self) -> DMATX_W<CR1_SPEC, 17> {
        DMATX_W::new(self)
    }
    #[doc = "Bit 18 - desc MISOHD"]
    #[inline(always)]
    #[must_use]
    pub fn misohd(&mut self) -> MISOHD_W<CR1_SPEC, 18> {
        MISOHD_W::new(self)
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
