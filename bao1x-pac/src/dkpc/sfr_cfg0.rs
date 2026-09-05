#[doc = "Register `SFR_CFG0` reader"]
pub type R = crate::R<SfrCfg0Spec>;
#[doc = "Register `SFR_CFG0` writer"]
pub type W = crate::W<SfrCfg0Spec>;
#[doc = "Field `KPOPO0` reader - KPOPO0 read/write control register"]
pub type Kpopo0R = crate::BitReader;
#[doc = "Field `KPOPO0` writer - KPOPO0 read/write control register"]
pub type Kpopo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KPOPO1` reader - KPOPO1 read/write control register"]
pub type Kpopo1R = crate::BitReader;
#[doc = "Field `KPOPO1` writer - KPOPO1 read/write control register"]
pub type Kpopo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KPOOE0` reader - KPOOE0 read/write control register"]
pub type Kpooe0R = crate::BitReader;
#[doc = "Field `KPOOE0` writer - KPOOE0 read/write control register"]
pub type Kpooe0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KPOOE1` reader - KPOOE1 read/write control register"]
pub type Kpooe1R = crate::BitReader;
#[doc = "Field `KPOOE1` writer - KPOOE1 read/write control register"]
pub type Kpooe1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dkpcen` reader - dkpcen read/write control register"]
pub type DkpcenR = crate::BitReader;
#[doc = "Field `dkpcen` writer - dkpcen read/write control register"]
pub type DkpcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `autosleepen` reader - autosleepen read/write control register"]
pub type AutosleepenR = crate::BitReader;
#[doc = "Field `autosleepen` writer - autosleepen read/write control register"]
pub type AutosleepenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - KPOPO0 read/write control register"]
    #[inline(always)]
    pub fn kpopo0(&self) -> Kpopo0R {
        Kpopo0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - KPOPO1 read/write control register"]
    #[inline(always)]
    pub fn kpopo1(&self) -> Kpopo1R {
        Kpopo1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - KPOOE0 read/write control register"]
    #[inline(always)]
    pub fn kpooe0(&self) -> Kpooe0R {
        Kpooe0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - KPOOE1 read/write control register"]
    #[inline(always)]
    pub fn kpooe1(&self) -> Kpooe1R {
        Kpooe1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - dkpcen read/write control register"]
    #[inline(always)]
    pub fn dkpcen(&self) -> DkpcenR {
        DkpcenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - autosleepen read/write control register"]
    #[inline(always)]
    pub fn autosleepen(&self) -> AutosleepenR {
        AutosleepenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - KPOPO0 read/write control register"]
    #[inline(always)]
    pub fn kpopo0(&mut self) -> Kpopo0W<'_, SfrCfg0Spec> {
        Kpopo0W::new(self, 0)
    }
    #[doc = "Bit 1 - KPOPO1 read/write control register"]
    #[inline(always)]
    pub fn kpopo1(&mut self) -> Kpopo1W<'_, SfrCfg0Spec> {
        Kpopo1W::new(self, 1)
    }
    #[doc = "Bit 2 - KPOOE0 read/write control register"]
    #[inline(always)]
    pub fn kpooe0(&mut self) -> Kpooe0W<'_, SfrCfg0Spec> {
        Kpooe0W::new(self, 2)
    }
    #[doc = "Bit 3 - KPOOE1 read/write control register"]
    #[inline(always)]
    pub fn kpooe1(&mut self) -> Kpooe1W<'_, SfrCfg0Spec> {
        Kpooe1W::new(self, 3)
    }
    #[doc = "Bit 4 - dkpcen read/write control register"]
    #[inline(always)]
    pub fn dkpcen(&mut self) -> DkpcenW<'_, SfrCfg0Spec> {
        DkpcenW::new(self, 4)
    }
    #[doc = "Bit 5 - autosleepen read/write control register"]
    #[inline(always)]
    pub fn autosleepen(&mut self) -> AutosleepenW<'_, SfrCfg0Spec> {
        AutosleepenW::new(self, 5)
    }
}
#[doc = "See `dkpc.sv#L167 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L167>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfg0Spec;
impl crate::RegisterSpec for SfrCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg0::R`](R) reader structure"]
impl crate::Readable for SfrCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg0::W`](W) writer structure"]
impl crate::Writable for SfrCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG0 to value 0"]
impl crate::Resettable for SfrCfg0Spec {}
