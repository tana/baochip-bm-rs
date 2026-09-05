#[doc = "Register `CR_REG_SD_STATUS_CFG_REG_SD_STATUS8` reader"]
pub type R = crate::R<CrRegSdStatusCfgRegSdStatus8Spec>;
#[doc = "Register `CR_REG_SD_STATUS_CFG_REG_SD_STATUS8` writer"]
pub type W = crate::W<CrRegSdStatusCfgRegSdStatus8Spec>;
#[doc = "Field `cfg_reg_sd_status8` reader - cr_reg_sd_status read/write control register"]
pub type CfgRegSdStatus8R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_sd_status8` writer - cr_reg_sd_status read/write control register"]
pub type CfgRegSdStatus8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_reg_sd_status read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_sd_status8(&self) -> CfgRegSdStatus8R {
        CfgRegSdStatus8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_reg_sd_status read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_sd_status8(&mut self) -> CfgRegSdStatus8W<'_, CrRegSdStatusCfgRegSdStatus8Spec> {
        CfgRegSdStatus8W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_sd_status_cfg_reg_sd_status8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_sd_status_cfg_reg_sd_status8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegSdStatusCfgRegSdStatus8Spec;
impl crate::RegisterSpec for CrRegSdStatusCfgRegSdStatus8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_sd_status_cfg_reg_sd_status8::R`](R) reader structure"]
impl crate::Readable for CrRegSdStatusCfgRegSdStatus8Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_sd_status_cfg_reg_sd_status8::W`](W) writer structure"]
impl crate::Writable for CrRegSdStatusCfgRegSdStatus8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_SD_STATUS_CFG_REG_SD_STATUS8 to value 0"]
impl crate::Resettable for CrRegSdStatusCfgRegSdStatus8Spec {}
