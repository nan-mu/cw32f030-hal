#[doc = "Register `DATE` reader"]
pub type R = crate::svd::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::svd::W<DATE_SPEC>;
#[doc = "Field `DAY` reader - desc DAY"]
pub type DAY_R = crate::svd::FieldReader;
#[doc = "Field `DAY` writer - desc DAY"]
pub type DAY_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `MONTH` reader - desc MONTH"]
pub type MONTH_R = crate::svd::FieldReader;
#[doc = "Field `MONTH` writer - desc MONTH"]
pub type MONTH_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `YEAR` reader - desc YEAR"]
pub type YEAR_R = crate::svd::FieldReader;
#[doc = "Field `YEAR` writer - desc YEAR"]
pub type YEAR_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WEEK` reader - desc WEEK"]
pub type WEEK_R = crate::svd::FieldReader;
#[doc = "Field `WEEK` writer - desc WEEK"]
pub type WEEK_W<'a, REG, const O: u8> = crate::svd::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:7 - desc DAY"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - desc MONTH"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - desc YEAR"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - desc WEEK"]
    #[inline(always)]
    pub fn week(&self) -> WEEK_R {
        WEEK_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DAY"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<DATE_SPEC, 0> {
        DAY_W::new(self)
    }
    #[doc = "Bits 8:15 - desc MONTH"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MONTH_W<DATE_SPEC, 8> {
        MONTH_W::new(self)
    }
    #[doc = "Bits 16:23 - desc YEAR"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<DATE_SPEC, 16> {
        YEAR_W::new(self)
    }
    #[doc = "Bits 24:26 - desc WEEK"]
    #[inline(always)]
    #[must_use]
    pub fn week(&mut self) -> WEEK_W<DATE_SPEC, 24> {
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
#[doc = "Time.Second register\n\nYou can [`read`](crate::svd::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::svd::generic::Reg::reset), [`write`](crate::svd::generic::Reg::write), [`write_with_zero`](crate::svd::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::svd::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::svd::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::svd::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::svd::Writable for DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATE to value 0"]
impl crate::svd::Resettable for DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
