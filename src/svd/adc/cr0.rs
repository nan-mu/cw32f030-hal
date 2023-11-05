#[doc = "Register `CR0` reader"]
pub type R = crate::svd::R<CR0_SPEC>;
#[doc = "Register `CR0` writer"]
pub type W = crate::svd::W<CR0_SPEC>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::svd::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::svd::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BGREN` reader - desc BGREN"]
pub type BGREN_R = crate::svd::BitReader;
#[doc = "Field `BGREN` writer - desc BGREN"]
pub type BGREN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `TSEN` reader - desc TSEN"]
pub type TSEN_R = crate::svd::BitReader;
#[doc = "Field `TSEN` writer - desc TSEN"]
pub type TSEN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `REF` reader - desc REF"]
pub type REF_R = crate::svd::FieldReader;
#[doc = "Field `REF` writer - desc REF"]
pub type REF_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CLK` reader - desc CLK"]
pub type CLK_R = crate::svd::FieldReader;
#[doc = "Field `CLK` writer - desc CLK"]
pub type CLK_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SAM` reader - desc SAM"]
pub type SAM_R = crate::svd::FieldReader;
#[doc = "Field `SAM` writer - desc SAM"]
pub type SAM_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BUF` reader - desc BUF"]
pub type BUF_R = crate::svd::BitReader;
#[doc = "Field `BUF` writer - desc BUF"]
pub type BUF_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `BIAS` reader - desc BIAS"]
pub type BIAS_R = crate::svd::FieldReader;
#[doc = "Field `BIAS` writer - desc BIAS"]
pub type BIAS_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - desc BGREN"]
    #[inline(always)]
    pub fn bgren(&self) -> BGREN_R {
        BGREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TSEN"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc REF"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - desc CLK"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - desc SAM"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - desc BUF"]
    #[inline(always)]
    pub fn buf(&self) -> BUF_R {
        BUF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - desc BIAS"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CR0_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:3 - desc MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CR0_SPEC, 1> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - desc BGREN"]
    #[inline(always)]
    #[must_use]
    pub fn bgren(&mut self) -> BGREN_W<CR0_SPEC, 4> {
        BGREN_W::new(self)
    }
    #[doc = "Bit 5 - desc TSEN"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<CR0_SPEC, 5> {
        TSEN_W::new(self)
    }
    #[doc = "Bits 6:7 - desc REF"]
    #[inline(always)]
    #[must_use]
    pub fn ref_(&mut self) -> REF_W<CR0_SPEC, 6> {
        REF_W::new(self)
    }
    #[doc = "Bits 8:10 - desc CLK"]
    #[inline(always)]
    #[must_use]
    pub fn clk(&mut self) -> CLK_W<CR0_SPEC, 8> {
        CLK_W::new(self)
    }
    #[doc = "Bits 11:12 - desc SAM"]
    #[inline(always)]
    #[must_use]
    pub fn sam(&mut self) -> SAM_W<CR0_SPEC, 11> {
        SAM_W::new(self)
    }
    #[doc = "Bit 13 - desc BUF"]
    #[inline(always)]
    #[must_use]
    pub fn buf(&mut self) -> BUF_W<CR0_SPEC, 13> {
        BUF_W::new(self)
    }
    #[doc = "Bits 14:15 - desc BIAS"]
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BIAS_W<CR0_SPEC, 14> {
        BIAS_W::new(self)
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
