#[doc = "Register `USERVALUE` reader"]
pub type R = crate::R<UservalueSpec>;
#[doc = "Register `USERVALUE` writer"]
pub type W = crate::W<UservalueSpec>;
#[doc = "Field `user0` reader - Value of `CoreUser` for lut0 match"]
pub type User0R = crate::FieldReader;
#[doc = "Field `user0` writer - Value of `CoreUser` for lut0 match"]
pub type User0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `user1` reader - Value of `CoreUser` for lut1 match"]
pub type User1R = crate::FieldReader;
#[doc = "Field `user1` writer - Value of `CoreUser` for lut1 match"]
pub type User1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `user2` reader - Value of `CoreUser` for lut2 match"]
pub type User2R = crate::FieldReader;
#[doc = "Field `user2` writer - Value of `CoreUser` for lut2 match"]
pub type User2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `user3` reader - Value of `CoreUser` for lut3 match"]
pub type User3R = crate::FieldReader;
#[doc = "Field `user3` writer - Value of `CoreUser` for lut3 match"]
pub type User3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `user4` reader - Value of `CoreUser` for lut4 match"]
pub type User4R = crate::FieldReader;
#[doc = "Field `user4` writer - Value of `CoreUser` for lut4 match"]
pub type User4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `user5` reader - Value of `CoreUser` for lut5 match"]
pub type User5R = crate::FieldReader;
#[doc = "Field `user5` writer - Value of `CoreUser` for lut5 match"]
pub type User5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `user6` reader - Value of `CoreUser` for lut6 match"]
pub type User6R = crate::FieldReader;
#[doc = "Field `user6` writer - Value of `CoreUser` for lut6 match"]
pub type User6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `user7` reader - Value of `CoreUser` for lut7 match"]
pub type User7R = crate::FieldReader;
#[doc = "Field `user7` writer - Value of `CoreUser` for lut7 match"]
pub type User7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `default` reader - Default value of `CoreUser`, for when none of the others match"]
pub type DefaultR = crate::FieldReader;
#[doc = "Field `default` writer - Default value of `CoreUser`, for when none of the others match"]
pub type DefaultW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Value of `CoreUser` for lut0 match"]
    #[inline(always)]
    pub fn user0(&self) -> User0R {
        User0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Value of `CoreUser` for lut1 match"]
    #[inline(always)]
    pub fn user1(&self) -> User1R {
        User1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Value of `CoreUser` for lut2 match"]
    #[inline(always)]
    pub fn user2(&self) -> User2R {
        User2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Value of `CoreUser` for lut3 match"]
    #[inline(always)]
    pub fn user3(&self) -> User3R {
        User3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Value of `CoreUser` for lut4 match"]
    #[inline(always)]
    pub fn user4(&self) -> User4R {
        User4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Value of `CoreUser` for lut5 match"]
    #[inline(always)]
    pub fn user5(&self) -> User5R {
        User5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Value of `CoreUser` for lut6 match"]
    #[inline(always)]
    pub fn user6(&self) -> User6R {
        User6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Value of `CoreUser` for lut7 match"]
    #[inline(always)]
    pub fn user7(&self) -> User7R {
        User7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Default value of `CoreUser`, for when none of the others match"]
    #[inline(always)]
    pub fn default(&self) -> DefaultR {
        DefaultR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Value of `CoreUser` for lut0 match"]
    #[inline(always)]
    pub fn user0(&mut self) -> User0W<'_, UservalueSpec> {
        User0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Value of `CoreUser` for lut1 match"]
    #[inline(always)]
    pub fn user1(&mut self) -> User1W<'_, UservalueSpec> {
        User1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Value of `CoreUser` for lut2 match"]
    #[inline(always)]
    pub fn user2(&mut self) -> User2W<'_, UservalueSpec> {
        User2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Value of `CoreUser` for lut3 match"]
    #[inline(always)]
    pub fn user3(&mut self) -> User3W<'_, UservalueSpec> {
        User3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Value of `CoreUser` for lut4 match"]
    #[inline(always)]
    pub fn user4(&mut self) -> User4W<'_, UservalueSpec> {
        User4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Value of `CoreUser` for lut5 match"]
    #[inline(always)]
    pub fn user5(&mut self) -> User5W<'_, UservalueSpec> {
        User5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Value of `CoreUser` for lut6 match"]
    #[inline(always)]
    pub fn user6(&mut self) -> User6W<'_, UservalueSpec> {
        User6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Value of `CoreUser` for lut7 match"]
    #[inline(always)]
    pub fn user7(&mut self) -> User7W<'_, UservalueSpec> {
        User7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Default value of `CoreUser`, for when none of the others match"]
    #[inline(always)]
    pub fn default(&mut self) -> DefaultW<'_, UservalueSpec> {
        DefaultW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`uservalue::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uservalue::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UservalueSpec;
impl crate::RegisterSpec for UservalueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uservalue::R`](R) reader structure"]
impl crate::Readable for UservalueSpec {}
#[doc = "`write(|w| ..)` method takes [`uservalue::W`](W) writer structure"]
impl crate::Writable for UservalueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USERVALUE to value 0"]
impl crate::Resettable for UservalueSpec {}
