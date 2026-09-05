#[doc = "Register `SFR_SR0` reader"]
pub type R = crate::R<SfrSr0Spec>;
#[doc = "Register `SFR_SR0` writer"]
pub type W = crate::W<SfrSr0Spec>;
#[doc = "Field `kpnodereg` reader - kpnodereg read only status register"]
pub type KpnoderegR = crate::BitReader;
#[doc = "Field `kpnodereg` writer - kpnodereg read only status register"]
pub type KpnoderegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `kpi0_pi` reader - kpi\\[0\\].pi read only status register"]
pub type Kpi0PiR = crate::BitReader;
#[doc = "Field `kpi0_pi` writer - kpi\\[0\\].pi read only status register"]
pub type Kpi0PiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `kpi1_pi` reader - kpi\\[1\\].pi read only status register"]
pub type Kpi1PiR = crate::BitReader;
#[doc = "Field `kpi1_pi` writer - kpi\\[1\\].pi read only status register"]
pub type Kpi1PiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `kpi2_pi` reader - kpi\\[2\\].pi read only status register"]
pub type Kpi2PiR = crate::BitReader;
#[doc = "Field `kpi2_pi` writer - kpi\\[2\\].pi read only status register"]
pub type Kpi2PiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `kpi3_pi` reader - kpi\\[3\\].pi read only status register"]
pub type Kpi3PiR = crate::BitReader;
#[doc = "Field `kpi3_pi` writer - kpi\\[3\\].pi read only status register"]
pub type Kpi3PiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - kpnodereg read only status register"]
    #[inline(always)]
    pub fn kpnodereg(&self) -> KpnoderegR {
        KpnoderegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - kpi\\[0\\].pi read only status register"]
    #[inline(always)]
    pub fn kpi0_pi(&self) -> Kpi0PiR {
        Kpi0PiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - kpi\\[1\\].pi read only status register"]
    #[inline(always)]
    pub fn kpi1_pi(&self) -> Kpi1PiR {
        Kpi1PiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - kpi\\[2\\].pi read only status register"]
    #[inline(always)]
    pub fn kpi2_pi(&self) -> Kpi2PiR {
        Kpi2PiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - kpi\\[3\\].pi read only status register"]
    #[inline(always)]
    pub fn kpi3_pi(&self) -> Kpi3PiR {
        Kpi3PiR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - kpnodereg read only status register"]
    #[inline(always)]
    pub fn kpnodereg(&mut self) -> KpnoderegW<'_, SfrSr0Spec> {
        KpnoderegW::new(self, 0)
    }
    #[doc = "Bit 1 - kpi\\[0\\].pi read only status register"]
    #[inline(always)]
    pub fn kpi0_pi(&mut self) -> Kpi0PiW<'_, SfrSr0Spec> {
        Kpi0PiW::new(self, 1)
    }
    #[doc = "Bit 2 - kpi\\[1\\].pi read only status register"]
    #[inline(always)]
    pub fn kpi1_pi(&mut self) -> Kpi1PiW<'_, SfrSr0Spec> {
        Kpi1PiW::new(self, 2)
    }
    #[doc = "Bit 3 - kpi\\[2\\].pi read only status register"]
    #[inline(always)]
    pub fn kpi2_pi(&mut self) -> Kpi2PiW<'_, SfrSr0Spec> {
        Kpi2PiW::new(self, 3)
    }
    #[doc = "Bit 4 - kpi\\[3\\].pi read only status register"]
    #[inline(always)]
    pub fn kpi3_pi(&mut self) -> Kpi3PiW<'_, SfrSr0Spec> {
        Kpi3PiW::new(self, 4)
    }
}
#[doc = "See `dkpc.sv#L173 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L173>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSr0Spec;
impl crate::RegisterSpec for SfrSr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sr0::R`](R) reader structure"]
impl crate::Readable for SfrSr0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sr0::W`](W) writer structure"]
impl crate::Writable for SfrSr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SR0 to value 0"]
impl crate::Resettable for SfrSr0Spec {}
