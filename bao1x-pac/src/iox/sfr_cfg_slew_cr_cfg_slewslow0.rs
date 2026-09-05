#[doc = "Register `SFR_CFG_SLEW_CR_CFG_SLEWSLOW0` reader"]
pub type R = crate::R<SfrCfgSlewCrCfgSlewslow0Spec>;
#[doc = "Register `SFR_CFG_SLEW_CR_CFG_SLEWSLOW0` writer"]
pub type W = crate::W<SfrCfgSlewCrCfgSlewslow0Spec>;
#[doc = "Field `cr_cfg_slewslow0` reader - cr_cfg_slewslow read/write control register"]
pub type CrCfgSlewslow0R = crate::FieldReader<u16>;
#[doc = "Field `cr_cfg_slewslow0` writer - cr_cfg_slewslow read/write control register"]
pub type CrCfgSlewslow0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cr_cfg_slewslow read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_slewslow0(&self) -> CrCfgSlewslow0R {
        CrCfgSlewslow0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cr_cfg_slewslow read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_slewslow0(&mut self) -> CrCfgSlewslow0W<'_, SfrCfgSlewCrCfgSlewslow0Spec> {
        CrCfgSlewslow0W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_slew_cr_cfg_slewslow0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_slew_cr_cfg_slewslow0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfgSlewCrCfgSlewslow0Spec;
impl crate::RegisterSpec for SfrCfgSlewCrCfgSlewslow0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg_slew_cr_cfg_slewslow0::R`](R) reader structure"]
impl crate::Readable for SfrCfgSlewCrCfgSlewslow0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg_slew_cr_cfg_slewslow0::W`](W) writer structure"]
impl crate::Writable for SfrCfgSlewCrCfgSlewslow0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG_SLEW_CR_CFG_SLEWSLOW0 to value 0"]
impl crate::Resettable for SfrCfgSlewCrCfgSlewslow0Spec {}
