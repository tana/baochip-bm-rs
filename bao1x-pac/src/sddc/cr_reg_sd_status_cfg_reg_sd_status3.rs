#[doc = "Register `CR_REG_SD_STATUS_CFG_REG_SD_STATUS3` reader"]
pub type R = crate::R<CrRegSdStatusCfgRegSdStatus3Spec>;
#[doc = "Register `CR_REG_SD_STATUS_CFG_REG_SD_STATUS3` writer"]
pub type W = crate::W<CrRegSdStatusCfgRegSdStatus3Spec>;
#[doc = "Field `cfg_reg_sd_status3` reader - cr_reg_sd_status read/write control register"]
pub type CfgRegSdStatus3R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_sd_status3` writer - cr_reg_sd_status read/write control register"]
pub type CfgRegSdStatus3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_reg_sd_status read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_sd_status3(&self) -> CfgRegSdStatus3R {
        CfgRegSdStatus3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_reg_sd_status read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_sd_status3(&mut self) -> CfgRegSdStatus3W<'_, CrRegSdStatusCfgRegSdStatus3Spec> {
        CfgRegSdStatus3W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegSdStatusCfgRegSdStatus3Spec;
impl crate::RegisterSpec for CrRegSdStatusCfgRegSdStatus3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_sd_status_cfg_reg_sd_status3::R`](R) reader structure"]
impl crate::Readable for CrRegSdStatusCfgRegSdStatus3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_sd_status_cfg_reg_sd_status3::W`](W) writer structure"]
impl crate::Writable for CrRegSdStatusCfgRegSdStatus3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_SD_STATUS_CFG_REG_SD_STATUS3 to value 0"]
impl crate::Resettable for CrRegSdStatusCfgRegSdStatus3Spec {}
