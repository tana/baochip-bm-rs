#[doc = "Register `SFR_CFG_SLEW_CR_CFG_SLEWSLOW2` reader"]
pub type R = crate::R<SfrCfgSlewCrCfgSlewslow2Spec>;
#[doc = "Register `SFR_CFG_SLEW_CR_CFG_SLEWSLOW2` writer"]
pub type W = crate::W<SfrCfgSlewCrCfgSlewslow2Spec>;
#[doc = "Field `cr_cfg_slewslow2` reader - cr_cfg_slewslow read/write control register"]
pub type CrCfgSlewslow2R = crate::FieldReader<u16>;
#[doc = "Field `cr_cfg_slewslow2` writer - cr_cfg_slewslow read/write control register"]
pub type CrCfgSlewslow2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cr_cfg_slewslow read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_slewslow2(&self) -> CrCfgSlewslow2R {
        CrCfgSlewslow2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cr_cfg_slewslow read/write control register"]
    #[inline(always)]
    pub fn cr_cfg_slewslow2(&mut self) -> CrCfgSlewslow2W<'_, SfrCfgSlewCrCfgSlewslow2Spec> {
        CrCfgSlewslow2W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_slew_cr_cfg_slewslow2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_slew_cr_cfg_slewslow2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfgSlewCrCfgSlewslow2Spec;
impl crate::RegisterSpec for SfrCfgSlewCrCfgSlewslow2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg_slew_cr_cfg_slewslow2::R`](R) reader structure"]
impl crate::Readable for SfrCfgSlewCrCfgSlewslow2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg_slew_cr_cfg_slewslow2::W`](W) writer structure"]
impl crate::Writable for SfrCfgSlewCrCfgSlewslow2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG_SLEW_CR_CFG_SLEWSLOW2 to value 0"]
impl crate::Resettable for SfrCfgSlewCrCfgSlewslow2Spec {}
