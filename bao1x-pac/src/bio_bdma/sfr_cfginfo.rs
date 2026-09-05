#[doc = "Register `SFR_CFGINFO` reader"]
pub type R = crate::R<SfrCfginfoSpec>;
#[doc = "Register `SFR_CFGINFO` writer"]
pub type W = crate::W<SfrCfginfoSpec>;
#[doc = "Field `constant0` reader - constant value of 8"]
pub type Constant0R = crate::FieldReader;
#[doc = "Field `constant0` writer - constant value of 8"]
pub type Constant0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `constant1` reader - constant value of 4"]
pub type Constant1R = crate::FieldReader;
#[doc = "Field `constant1` writer - constant value of 4"]
pub type Constant1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `constant2` reader - constant value of 4096"]
pub type Constant2R = crate::FieldReader<u16>;
#[doc = "Field `constant2` writer - constant value of 4096"]
pub type Constant2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - constant value of 8"]
    #[inline(always)]
    pub fn constant0(&self) -> Constant0R {
        Constant0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - constant value of 4"]
    #[inline(always)]
    pub fn constant1(&self) -> Constant1R {
        Constant1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - constant value of 4096"]
    #[inline(always)]
    pub fn constant2(&self) -> Constant2R {
        Constant2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - constant value of 8"]
    #[inline(always)]
    pub fn constant0(&mut self) -> Constant0W<'_, SfrCfginfoSpec> {
        Constant0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - constant value of 4"]
    #[inline(always)]
    pub fn constant1(&mut self) -> Constant1W<'_, SfrCfginfoSpec> {
        Constant1W::new(self, 8)
    }
    #[doc = "Bits 16:31 - constant value of 4096"]
    #[inline(always)]
    pub fn constant2(&mut self) -> Constant2W<'_, SfrCfginfoSpec> {
        Constant2W::new(self, 16)
    }
}
#[doc = "See `bio_bdma.sv#L489 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L489>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfginfo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfginfo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfginfoSpec;
impl crate::RegisterSpec for SfrCfginfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfginfo::R`](R) reader structure"]
impl crate::Readable for SfrCfginfoSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfginfo::W`](W) writer structure"]
impl crate::Writable for SfrCfginfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFGINFO to value 0"]
impl crate::Resettable for SfrCfginfoSpec {}
