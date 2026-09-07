#[doc = "Register `MAP_HI` reader"]
pub type R = crate::R<MapHiSpec>;
#[doc = "Register `MAP_HI` writer"]
pub type W = crate::W<MapHiSpec>;
#[doc = "Field `lut4` reader - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
pub type Lut4R = crate::FieldReader;
#[doc = "Field `lut4` writer - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
pub type Lut4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lut5` reader - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
pub type Lut5R = crate::FieldReader;
#[doc = "Field `lut5` writer - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
pub type Lut5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lut6` reader - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
pub type Lut6R = crate::FieldReader;
#[doc = "Field `lut6` writer - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
pub type Lut6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lut7` reader - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
pub type Lut7R = crate::FieldReader;
#[doc = "Field `lut7` writer - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
pub type Lut7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
    #[inline(always)]
    pub fn lut4(&self) -> Lut4R {
        Lut4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
    #[inline(always)]
    pub fn lut5(&self) -> Lut5R {
        Lut5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
    #[inline(always)]
    pub fn lut6(&self) -> Lut6R {
        Lut6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
    #[inline(always)]
    pub fn lut7(&self) -> Lut7R {
        Lut7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
    #[inline(always)]
    pub fn lut4(&mut self) -> Lut4W<'_, MapHiSpec> {
        Lut4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
    #[inline(always)]
    pub fn lut5(&mut self) -> Lut5W<'_, MapHiSpec> {
        Lut5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
    #[inline(always)]
    pub fn lut6(&mut self) -> Lut6W<'_, MapHiSpec> {
        Lut6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Mapping of `CoreUser` ASID to coreuser dense coding bit 1"]
    #[inline(always)]
    pub fn lut7(&mut self) -> Lut7W<'_, MapHiSpec> {
        Lut7W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`map_hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`map_hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MapHiSpec;
impl crate::RegisterSpec for MapHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`map_hi::R`](R) reader structure"]
impl crate::Readable for MapHiSpec {}
#[doc = "`write(|w| ..)` method takes [`map_hi::W`](W) writer structure"]
impl crate::Writable for MapHiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAP_HI to value 0"]
impl crate::Resettable for MapHiSpec {}
