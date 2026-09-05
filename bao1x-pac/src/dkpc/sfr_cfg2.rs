#[doc = "Register `SFR_CFG2` reader"]
pub type R = crate::R<SfrCfg2Spec>;
#[doc = "Register `SFR_CFG2` writer"]
pub type W = crate::W<SfrCfg2Spec>;
#[doc = "Field `cfg_cnt` reader - cfg_cnt read/write control register"]
pub type CfgCntR = crate::FieldReader<u32>;
#[doc = "Field `cfg_cnt` writer - cfg_cnt read/write control register"]
pub type CfgCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cfg_cnt read/write control register"]
    #[inline(always)]
    pub fn cfg_cnt(&self) -> CfgCntR {
        CfgCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cfg_cnt read/write control register"]
    #[inline(always)]
    pub fn cfg_cnt(&mut self) -> CfgCntW<'_, SfrCfg2Spec> {
        CfgCntW::new(self, 0)
    }
}
#[doc = "See `dkpc.sv#L169 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L169>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfg2Spec;
impl crate::RegisterSpec for SfrCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg2::R`](R) reader structure"]
impl crate::Readable for SfrCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg2::W`](W) writer structure"]
impl crate::Writable for SfrCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG2 to value 0"]
impl crate::Resettable for SfrCfg2Spec {}
