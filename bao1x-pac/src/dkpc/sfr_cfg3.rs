#[doc = "Register `SFR_CFG3` reader"]
pub type R = crate::R<SfrCfg3Spec>;
#[doc = "Register `SFR_CFG3` writer"]
pub type W = crate::W<SfrCfg3Spec>;
#[doc = "Field `kpnoderiseen` reader - kpnoderiseen read/write control register"]
pub type KpnoderiseenR = crate::BitReader;
#[doc = "Field `kpnoderiseen` writer - kpnoderiseen read/write control register"]
pub type KpnoderiseenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `kpnodefallen` reader - kpnodefallen read/write control register"]
pub type KpnodefallenR = crate::BitReader;
#[doc = "Field `kpnodefallen` writer - kpnodefallen read/write control register"]
pub type KpnodefallenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - kpnoderiseen read/write control register"]
    #[inline(always)]
    pub fn kpnoderiseen(&self) -> KpnoderiseenR {
        KpnoderiseenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - kpnodefallen read/write control register"]
    #[inline(always)]
    pub fn kpnodefallen(&self) -> KpnodefallenR {
        KpnodefallenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - kpnoderiseen read/write control register"]
    #[inline(always)]
    pub fn kpnoderiseen(&mut self) -> KpnoderiseenW<'_, SfrCfg3Spec> {
        KpnoderiseenW::new(self, 0)
    }
    #[doc = "Bit 1 - kpnodefallen read/write control register"]
    #[inline(always)]
    pub fn kpnodefallen(&mut self) -> KpnodefallenW<'_, SfrCfg3Spec> {
        KpnodefallenW::new(self, 1)
    }
}
#[doc = "See `dkpc.sv#L170 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L170>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfg3Spec;
impl crate::RegisterSpec for SfrCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg3::R`](R) reader structure"]
impl crate::Readable for SfrCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg3::W`](W) writer structure"]
impl crate::Writable for SfrCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG3 to value 0"]
impl crate::Resettable for SfrCfg3Spec {}
