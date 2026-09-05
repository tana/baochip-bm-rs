#[doc = "Register `SFR_CFG_SLEW_CR_CFG_SLEWSLOW3` reader"]
pub type R = crate::R<SfrCfgSlewCrCfgSlewslow3Spec>;
#[doc = "Register `SFR_CFG_SLEW_CR_CFG_SLEWSLOW3` writer"]
pub type W = crate::W<SfrCfgSlewCrCfgSlewslow3Spec>;
#[doc = "Field `cr_cfg_slewslow3` reader - cr_cfg_slewslow read/write control register"]
pub type CrCfgSlewslow3R = crate::FieldReader<u16>;
#[doc = "Field `cr_cfg_slewslow3` writer - cr_cfg_slewslow read/write control register"]
pub type CrCfgSlewslow3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cr_cfg_slewslow read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_slewslow3(&self) -> CrCfgSlewslow3R {
        CrCfgSlewslow3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cr_cfg_slewslow read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_slewslow3(&mut self) -> CrCfgSlewslow3W<'_, SfrCfgSlewCrCfgSlewslow3Spec> {
        CrCfgSlewslow3W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_slew_cr_cfg_slewslow3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_slew_cr_cfg_slewslow3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfgSlewCrCfgSlewslow3Spec;
impl crate::RegisterSpec for SfrCfgSlewCrCfgSlewslow3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg_slew_cr_cfg_slewslow3::R`](R) reader structure"]
impl crate::Readable for SfrCfgSlewCrCfgSlewslow3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg_slew_cr_cfg_slewslow3::W`](W) writer structure"]
impl crate::Writable for SfrCfgSlewCrCfgSlewslow3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG_SLEW_CR_CFG_SLEWSLOW3 to value 0"]
impl crate::Resettable for SfrCfgSlewCrCfgSlewslow3Spec {}
