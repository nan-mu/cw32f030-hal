#[doc = "Register `CR1` reader"]
pub type R = crate::svd::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::svd::W<CR1_SPEC>;
#[doc = "Field `TXEN` reader - desc TXEN"]
pub type TXEN_R = crate::svd::BitReader;
#[doc = "Field `TXEN` writer - desc TXEN"]
pub type TXEN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `RXEN` reader - desc RXEN"]
pub type RXEN_R = crate::svd::BitReader;
#[doc = "Field `RXEN` writer - desc RXEN"]
pub type RXEN_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `PARITY` reader - desc PARITY"]
pub type PARITY_R = crate::svd::FieldReader;
#[doc = "Field `PARITY` writer - desc PARITY"]
pub type PARITY_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type STOP_R = crate::svd::FieldReader;
#[doc = "Field `STOP` writer - desc STOP"]
pub type STOP_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SYNC` reader - desc SYNC"]
pub type SYNC_R = crate::svd::BitReader;
#[doc = "Field `SYNC` writer - desc SYNC"]
pub type SYNC_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `START` reader - desc START"]
pub type START_R = crate::svd::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type START_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `OVER` reader - desc OVER"]
pub type OVER_R = crate::svd::FieldReader;
#[doc = "Field `OVER` writer - desc OVER"]
pub type OVER_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - desc TXEN"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXEN"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc PARITY"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc SYNC"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - desc OVER"]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc TXEN"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<CR1_SPEC, 0> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 1 - desc RXEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<CR1_SPEC, 1> {
        RXEN_W::new(self)
    }
    #[doc = "Bits 2:3 - desc PARITY"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<CR1_SPEC, 2> {
        PARITY_W::new(self)
    }
    #[doc = "Bits 4:5 - desc STOP"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR1_SPEC, 4> {
        STOP_W::new(self)
    }
    #[doc = "Bit 6 - desc SYNC"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<CR1_SPEC, 6> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CR1_SPEC, 8> {
        START_W::new(self)
    }
    #[doc = "Bits 9:10 - desc OVER"]
    #[inline(always)]
    #[must_use]
    pub fn over(&mut self) -> OVER_W<CR1_SPEC, 9> {
        OVER_W::new(self)
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
