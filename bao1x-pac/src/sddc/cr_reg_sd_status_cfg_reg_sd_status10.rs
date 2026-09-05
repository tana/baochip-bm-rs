#[doc = "Register `CR_REG_SD_STATUS_CFG_REG_SD_STATUS10` reader"]
pub type R = crate::R<CrRegSdStatusCfgRegSdStatus10Spec>;
#[doc = "Register `CR_REG_SD_STATUS_CFG_REG_SD_STATUS10` writer"]
pub type W = crate::W<CrRegSdStatusCfgRegSdStatus10Spec>;
#[doc = "Field `cfg_reg_sd_status10` reader - cr_reg_sd_status read/write control register"]
pub type CfgRegSdStatus10R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_sd_status10` writer - cr_reg_sd_status read/write control register"]
pub type CfgRegSdStatus10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_reg_sd_status read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_sd_status10(&self) -> CfgRegSdStatus10R {
        CfgRegSdStatus10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_reg_sd_status read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_sd_status10(
        &mut self,
    ) -> CfgRegSdStatus10W<'_, CrRegSdStatusCfgRegSdStatus10Spec> {
        CfgRegSdStatus10W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegSdStatusCfgRegSdStatus10Spec;
impl crate::RegisterSpec for CrRegSdStatusCfgRegSdStatus10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_sd_status_cfg_reg_sd_status10::R`](R) reader structure"]
impl crate::Readable for CrRegSdStatusCfgRegSdStatus10Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_sd_status_cfg_reg_sd_status10::W`](W) writer structure"]
impl crate::Writable for CrRegSdStatusCfgRegSdStatus10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_SD_STATUS_CFG_REG_SD_STATUS10 to value 0"]
impl crate::Resettable for CrRegSdStatusCfgRegSdStatus10Spec {}
