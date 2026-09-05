#[doc = "Register `CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO4` reader"]
pub type R = crate::R<CrRegFuncManufactInfoCfgRegFuncManufactInfo4Spec>;
#[doc = "Register `CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO4` writer"]
pub type W = crate::W<CrRegFuncManufactInfoCfgRegFuncManufactInfo4Spec>;
#[doc = "Field `cfg_reg_func_manufact_info4` reader - cfg_reg_func_manufact_info read/write control register"]
pub type CfgRegFuncManufactInfo4R = crate::FieldReader<u16>;
#[doc = "Field `cfg_reg_func_manufact_info4` writer - cfg_reg_func_manufact_info read/write control register"]
pub type CfgRegFuncManufactInfo4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cfg_reg_func_manufact_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_manufact_info4(&self) -> CfgRegFuncManufactInfo4R {
        CfgRegFuncManufactInfo4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cfg_reg_func_manufact_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_manufact_info4(
        &mut self,
    ) -> CfgRegFuncManufactInfo4W<'_, CrRegFuncManufactInfoCfgRegFuncManufactInfo4Spec> {
        CfgRegFuncManufactInfo4W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncManufactInfoCfgRegFuncManufactInfo4Spec;
impl crate::RegisterSpec for CrRegFuncManufactInfoCfgRegFuncManufactInfo4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info4::R`](R) reader structure"]
impl crate::Readable for CrRegFuncManufactInfoCfgRegFuncManufactInfo4Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info4::W`](W) writer structure"]
impl crate::Writable for CrRegFuncManufactInfoCfgRegFuncManufactInfo4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO4 to value 0"]
impl crate::Resettable for CrRegFuncManufactInfoCfgRegFuncManufactInfo4Spec {}
