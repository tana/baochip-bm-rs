#[doc = "Register `MAP_LO` reader"]
pub type R = crate::R<MapLoSpec>;
#[doc = "Register `MAP_LO` writer"]
pub type W = crate::W<MapLoSpec>;
#[doc = "Field `lut0` reader - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
pub type Lut0R = crate::FieldReader;
#[doc = "Field `lut0` writer - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
pub type Lut0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lut1` reader - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
pub type Lut1R = crate::FieldReader;
#[doc = "Field `lut1` writer - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
pub type Lut1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lut2` reader - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
pub type Lut2R = crate::FieldReader;
#[doc = "Field `lut2` writer - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
pub type Lut2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lut3` reader - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
pub type Lut3R = crate::FieldReader;
#[doc = "Field `lut3` writer - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
pub type Lut3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
    #[inline(always)]
    pub fn lut0(&self) -> Lut0R {
        Lut0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
    #[inline(always)]
    pub fn lut1(&self) -> Lut1R {
        Lut1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
    #[inline(always)]
    pub fn lut2(&self) -> Lut2R {
        Lut2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
    #[inline(always)]
    pub fn lut3(&self) -> Lut3R {
        Lut3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
    #[inline(always)]
    pub fn lut0(&mut self) -> Lut0W<'_, MapLoSpec> {
        Lut0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
    #[inline(always)]
    pub fn lut1(&mut self) -> Lut1W<'_, MapLoSpec> {
        Lut1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
    #[inline(always)]
    pub fn lut2(&mut self) -> Lut2W<'_, MapLoSpec> {
        Lut2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Mapping of `CoreUser` ASID to coreuser dense conding bit 0"]
    #[inline(always)]
    pub fn lut3(&mut self) -> Lut3W<'_, MapLoSpec> {
        Lut3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`map_lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`map_lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MapLoSpec;
impl crate::RegisterSpec for MapLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`map_lo::R`](R) reader structure"]
impl crate::Readable for MapLoSpec {}
#[doc = "`write(|w| ..)` method takes [`map_lo::W`](W) writer structure"]
impl crate::Writable for MapLoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAP_LO to value 0"]
impl crate::Resettable for MapLoSpec {}
