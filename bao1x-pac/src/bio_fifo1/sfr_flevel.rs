#[doc = "Register `SFR_FLEVEL` reader"]
pub type R = crate::R<SfrFlevelSpec>;
#[doc = "Register `SFR_FLEVEL` writer"]
pub type W = crate::W<SfrFlevelSpec>;
#[doc = "Field `pclk_regfifo_level0` reader - pclk_regfifo_level\\[0\\] read only status register"]
pub type PclkRegfifoLevel0R = crate::FieldReader;
#[doc = "Field `pclk_regfifo_level0` writer - pclk_regfifo_level\\[0\\] read only status register"]
pub type PclkRegfifoLevel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pclk_regfifo_level1` reader - pclk_regfifo_level\\[1\\] read only status register"]
pub type PclkRegfifoLevel1R = crate::FieldReader;
#[doc = "Field `pclk_regfifo_level1` writer - pclk_regfifo_level\\[1\\] read only status register"]
pub type PclkRegfifoLevel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pclk_regfifo_level2` reader - pclk_regfifo_level\\[2\\] read only status register"]
pub type PclkRegfifoLevel2R = crate::FieldReader;
#[doc = "Field `pclk_regfifo_level2` writer - pclk_regfifo_level\\[2\\] read only status register"]
pub type PclkRegfifoLevel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pclk_regfifo_level3` reader - pclk_regfifo_level\\[3\\] read only status register"]
pub type PclkRegfifoLevel3R = crate::FieldReader;
#[doc = "Field `pclk_regfifo_level3` writer - pclk_regfifo_level\\[3\\] read only status register"]
pub type PclkRegfifoLevel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - pclk_regfifo_level\\[0\\] read only status register"]
    #[inline(always)]
    pub fn pclk_regfifo_level0(&self) -> PclkRegfifoLevel0R {
        PclkRegfifoLevel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - pclk_regfifo_level\\[1\\] read only status register"]
    #[inline(always)]
    pub fn pclk_regfifo_level1(&self) -> PclkRegfifoLevel1R {
        PclkRegfifoLevel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - pclk_regfifo_level\\[2\\] read only status register"]
    #[inline(always)]
    pub fn pclk_regfifo_level2(&self) -> PclkRegfifoLevel2R {
        PclkRegfifoLevel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - pclk_regfifo_level\\[3\\] read only status register"]
    #[inline(always)]
    pub fn pclk_regfifo_level3(&self) -> PclkRegfifoLevel3R {
        PclkRegfifoLevel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - pclk_regfifo_level\\[0\\] read only status register"]
    #[inline(always)]
    pub fn pclk_regfifo_level0(&mut self) -> PclkRegfifoLevel0W<'_, SfrFlevelSpec> {
        PclkRegfifoLevel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - pclk_regfifo_level\\[1\\] read only status register"]
    #[inline(always)]
    pub fn pclk_regfifo_level1(&mut self) -> PclkRegfifoLevel1W<'_, SfrFlevelSpec> {
        PclkRegfifoLevel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - pclk_regfifo_level\\[2\\] read only status register"]
    #[inline(always)]
    pub fn pclk_regfifo_level2(&mut self) -> PclkRegfifoLevel2W<'_, SfrFlevelSpec> {
        PclkRegfifoLevel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - pclk_regfifo_level\\[3\\] read only status register"]
    #[inline(always)]
    pub fn pclk_regfifo_level3(&mut self) -> PclkRegfifoLevel3W<'_, SfrFlevelSpec> {
        PclkRegfifoLevel3W::new(self, 12)
    }
}
#[doc = "See `bio_bdma.sv#L492 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L492>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_flevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_flevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFlevelSpec;
impl crate::RegisterSpec for SfrFlevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_flevel::R`](R) reader structure"]
impl crate::Readable for SfrFlevelSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_flevel::W`](W) writer structure"]
impl crate::Writable for SfrFlevelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FLEVEL to value 0"]
impl crate::Resettable for SfrFlevelSpec {}
