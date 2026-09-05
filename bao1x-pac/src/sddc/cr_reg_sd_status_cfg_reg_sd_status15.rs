#[doc = "Register `CR_REG_SD_STATUS_CFG_REG_SD_STATUS15` reader"]
pub type R = crate::R<CrRegSdStatusCfgRegSdStatus15Spec>;
#[doc = "Register `CR_REG_SD_STATUS_CFG_REG_SD_STATUS15` writer"]
pub type W = crate::W<CrRegSdStatusCfgRegSdStatus15Spec>;
#[doc = "Field `cfg_reg_sd_status15` reader - cr_reg_sd_status read/write control register"]
pub type CfgRegSdStatus15R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_sd_status15` writer - cr_reg_sd_status read/write control register"]
pub type CfgRegSdStatus15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_reg_sd_status read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_sd_status15(&self) -> CfgRegSdStatus15R {
        CfgRegSdStatus15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_reg_sd_status read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_sd_status15(
        &mut self,
    ) -> CfgRegSdStatus15W<'_, CrRegSdStatusCfgRegSdStatus15Spec> {
        CfgRegSdStatus15W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegSdStatusCfgRegSdStatus15Spec;
impl crate::RegisterSpec for CrRegSdStatusCfgRegSdStatus15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_sd_status_cfg_reg_sd_status15::R`](R) reader structure"]
impl crate::Readable for CrRegSdStatusCfgRegSdStatus15Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_sd_status_cfg_reg_sd_status15::W`](W) writer structure"]
impl crate::Writable for CrRegSdStatusCfgRegSdStatus15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_SD_STATUS_CFG_REG_SD_STATUS15 to value 0"]
impl crate::Resettable for CrRegSdStatusCfgRegSdStatus15Spec {}
