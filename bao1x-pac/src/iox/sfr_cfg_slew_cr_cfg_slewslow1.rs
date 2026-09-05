#[doc = "Register `SFR_CFG_SLEW_CR_CFG_SLEWSLOW1` reader"]
pub type R = crate::R<SfrCfgSlewCrCfgSlewslow1Spec>;
#[doc = "Register `SFR_CFG_SLEW_CR_CFG_SLEWSLOW1` writer"]
pub type W = crate::W<SfrCfgSlewCrCfgSlewslow1Spec>;
#[doc = "Field `cr_cfg_slewslow1` reader - cr_cfg_slewslow read/write control register"]
pub type CrCfgSlewslow1R = crate::FieldReader<u16>;
#[doc = "Field `cr_cfg_slewslow1` writer - cr_cfg_slewslow read/write control register"]
pub type CrCfgSlewslow1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cr_cfg_slewslow read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_slewslow1(&self) -> CrCfgSlewslow1R {
        CrCfgSlewslow1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cr_cfg_slewslow read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_slewslow1(&mut self) -> CrCfgSlewslow1W<'_, SfrCfgSlewCrCfgSlewslow1Spec> {
        CrCfgSlewslow1W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_slew_cr_cfg_slewslow1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_slew_cr_cfg_slewslow1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfgSlewCrCfgSlewslow1Spec;
impl crate::RegisterSpec for SfrCfgSlewCrCfgSlewslow1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg_slew_cr_cfg_slewslow1::R`](R) reader structure"]
impl crate::Readable for SfrCfgSlewCrCfgSlewslow1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg_slew_cr_cfg_slewslow1::W`](W) writer structure"]
impl crate::Writable for SfrCfgSlewCrCfgSlewslow1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG_SLEW_CR_CFG_SLEWSLOW1 to value 0"]
impl crate::Resettable for SfrCfgSlewCrCfgSlewslow1Spec {}
