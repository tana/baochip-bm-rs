#[doc = "Register `SFR_CFG_SLEW_CR_CFG_SLEWSLOW5` reader"]
pub type R = crate::R<SfrCfgSlewCrCfgSlewslow5Spec>;
#[doc = "Register `SFR_CFG_SLEW_CR_CFG_SLEWSLOW5` writer"]
pub type W = crate::W<SfrCfgSlewCrCfgSlewslow5Spec>;
#[doc = "Field `cr_cfg_slewslow5` reader - cr_cfg_slewslow read/write control register"]
pub type CrCfgSlewslow5R = crate::FieldReader<u16>;
#[doc = "Field `cr_cfg_slewslow5` writer - cr_cfg_slewslow read/write control register"]
pub type CrCfgSlewslow5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cr_cfg_slewslow read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_slewslow5(&self) -> CrCfgSlewslow5R {
        CrCfgSlewslow5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cr_cfg_slewslow read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_slewslow5(&mut self) -> CrCfgSlewslow5W<'_, SfrCfgSlewCrCfgSlewslow5Spec> {
        CrCfgSlewslow5W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_slew_cr_cfg_slewslow5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_slew_cr_cfg_slewslow5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfgSlewCrCfgSlewslow5Spec;
impl crate::RegisterSpec for SfrCfgSlewCrCfgSlewslow5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg_slew_cr_cfg_slewslow5::R`](R) reader structure"]
impl crate::Readable for SfrCfgSlewCrCfgSlewslow5Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg_slew_cr_cfg_slewslow5::W`](W) writer structure"]
impl crate::Writable for SfrCfgSlewCrCfgSlewslow5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG_SLEW_CR_CFG_SLEWSLOW5 to value 0"]
impl crate::Resettable for SfrCfgSlewCrCfgSlewslow5Spec {}
