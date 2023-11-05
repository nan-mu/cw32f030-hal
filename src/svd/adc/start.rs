#[doc = "Register `START` reader"]
pub type R = crate::svd::R<START_SPEC>;
#[doc = "Register `START` writer"]
pub type W = crate::svd::W<START_SPEC>;
#[doc = "Field `START` reader - desc START"]
pub type START_R = crate::svd::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type START_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOSTOP` reader - desc AUTOSTOP"]
pub type AUTOSTOP_R = crate::svd::BitReader;
#[doc = "Field `AUTOSTOP` writer - desc AUTOSTOP"]
pub type AUTOSTOP_W<'a, REG, const O: u8> = crate::svd::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc AUTOSTOP"]
    #[inline(always)]
    pub fn autostop(&self) -> AUTOSTOP_R {
        AUTOSTOP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc START"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<START_SPEC, 0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - desc AUTOSTOP"]
    #[inline(always)]
    #[must_use]
    pub fn autostop(&mut self) -> AUTOSTOP_W<START_SPEC, 1> {
        AUTOSTOP_W::new(self)
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
#[doc = "desc START\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`start::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`start::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct START_SPEC;
impl crate::svd::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start::R`](R) reader structure"]
impl crate::svd::Readable for START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::svd::Writable for START_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::svd::Resettable for START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
