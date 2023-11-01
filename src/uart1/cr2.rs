#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Field `ADDREN` reader - desc ADDREN"]
pub type ADDREN_R = crate::BitReader;
#[doc = "Field `ADDREN` writer - desc ADDREN"]
pub type ADDREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SIGNAL` reader - desc SIGNAL"]
pub type SIGNAL_R = crate::BitReader;
#[doc = "Field `SIGNAL` writer - desc SIGNAL"]
pub type SIGNAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTSEN` reader - desc CTSEN"]
pub type CTSEN_R = crate::BitReader;
#[doc = "Field `CTSEN` writer - desc CTSEN"]
pub type CTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTSEN` reader - desc RTSEN"]
pub type RTSEN_R = crate::BitReader;
#[doc = "Field `RTSEN` writer - desc RTSEN"]
pub type RTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXINV` reader - desc RXINV"]
pub type RXINV_R = crate::BitReader;
#[doc = "Field `RXINV` writer - desc RXINV"]
pub type RXINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXINV` reader - desc TXINV"]
pub type TXINV_R = crate::BitReader;
#[doc = "Field `TXINV` writer - desc TXINV"]
pub type TXINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMARX` reader - desc DMARX"]
pub type DMARX_R = crate::BitReader;
#[doc = "Field `DMARX` writer - desc DMARX"]
pub type DMARX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMATX` reader - desc DMATX"]
pub type DMATX_R = crate::BitReader;
#[doc = "Field `DMATX` writer - desc DMATX"]
pub type DMATX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SOURCE_R = crate::FieldReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SOURCE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - desc ADDREN"]
    #[inline(always)]
    pub fn addren(&self) -> ADDREN_R {
        ADDREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SIGNAL"]
    #[inline(always)]
    pub fn signal(&self) -> SIGNAL_R {
        SIGNAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CTSEN"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc RTSEN"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RXINV"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TXINV"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DMARX"]
    #[inline(always)]
    pub fn dmarx(&self) -> DMARX_R {
        DMARX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DMATX"]
    #[inline(always)]
    pub fn dmatx(&self) -> DMATX_R {
        DMATX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADDREN"]
    #[inline(always)]
    #[must_use]
    pub fn addren(&mut self) -> ADDREN_W<CR2_SPEC, 0> {
        ADDREN_W::new(self)
    }
    #[doc = "Bit 1 - desc SIGNAL"]
    #[inline(always)]
    #[must_use]
    pub fn signal(&mut self) -> SIGNAL_W<CR2_SPEC, 1> {
        SIGNAL_W::new(self)
    }
    #[doc = "Bit 2 - desc CTSEN"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CTSEN_W<CR2_SPEC, 2> {
        CTSEN_W::new(self)
    }
    #[doc = "Bit 3 - desc RTSEN"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RTSEN_W<CR2_SPEC, 3> {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 4 - desc RXINV"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<CR2_SPEC, 4> {
        RXINV_W::new(self)
    }
    #[doc = "Bit 5 - desc TXINV"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<CR2_SPEC, 5> {
        TXINV_W::new(self)
    }
    #[doc = "Bit 6 - desc DMARX"]
    #[inline(always)]
    #[must_use]
    pub fn dmarx(&mut self) -> DMARX_W<CR2_SPEC, 6> {
        DMARX_W::new(self)
    }
    #[doc = "Bit 7 - desc DMATX"]
    #[inline(always)]
    #[must_use]
    pub fn dmatx(&mut self) -> DMATX_W<CR2_SPEC, 7> {
        DMATX_W::new(self)
    }
    #[doc = "Bits 8:9 - desc SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<CR2_SPEC, 8> {
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
#[doc = "Control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
