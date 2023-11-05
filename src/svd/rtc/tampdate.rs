#[doc = "Register `TAMPDATE` reader"]
pub type R = crate::svd::R<TAMPDATE_SPEC>;
#[doc = "Register `TAMPDATE` writer"]
pub type W = crate::svd::W<TAMPDATE_SPEC>;
#[doc = "Field `DAY` reader - desc DAY"]
pub type DAY_R = crate::svd::FieldReader;
#[doc = "Field `DAY` writer - desc DAY"]
pub type DAY_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `MONTH` reader - desc MONTH"]
pub type MONTH_R = crate::svd::FieldReader;
#[doc = "Field `MONTH` writer - desc MONTH"]
pub type MONTH_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `WEEK` reader - desc WEEK"]
pub type WEEK_R = crate::svd::FieldReader;
#[doc = "Field `WEEK` writer - desc WEEK"]
pub type WEEK_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:5 - desc DAY"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - desc MONTH"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - desc WEEK"]
    #[inline(always)]
    pub fn week(&self) -> WEEK_R {
        WEEK_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc DAY"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<TAMPDATE_SPEC, 0> {
        DAY_W::new(self)
    }
    #[doc = "Bits 8:12 - desc MONTH"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MONTH_W<TAMPDATE_SPEC, 8> {
        MONTH_W::new(self)
    }
    #[doc = "Bits 13:15 - desc WEEK"]
    #[inline(always)]
    #[must_use]
    pub fn week(&mut self) -> WEEK_W<TAMPDATE_SPEC, 13> {
        WEEK_W::new(self)
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
#[doc = "desc TAMPDATE\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`tampdate::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`tampdate::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMPDATE_SPEC;
impl crate::svd::RegisterSpec for TAMPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tampdate::R`](R) reader structure"]
impl crate::svd::Readable for TAMPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tampdate::W`](W) writer structure"]
impl crate::svd::Writable for TAMPDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMPDATE to value 0"]
impl crate::svd::Resettable for TAMPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
