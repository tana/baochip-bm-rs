#[doc = "Register `CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO5` reader"]
pub type R = crate::R<CrRegFuncManufactInfoCfgRegFuncManufactInfo5Spec>;
#[doc = "Register `CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO5` writer"]
pub type W = crate::W<CrRegFuncManufactInfoCfgRegFuncManufactInfo5Spec>;
#[doc = "Field `cfg_reg_func_manufact_info5` reader - cfg_reg_func_manufact_info read/write control register"]
pub type CfgRegFuncManufactInfo5R = crate::FieldReader<u16>;
#[doc = "Field `cfg_reg_func_manufact_info5` writer - cfg_reg_func_manufact_info read/write control register"]
pub type CfgRegFuncManufactInfo5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cfg_reg_func_manufact_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_manufact_info5(&self) -> CfgRegFuncManufactInfo5R {
        CfgRegFuncManufactInfo5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cfg_reg_func_manufact_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_manufact_info5(
        &mut self,
    ) -> CfgRegFuncManufactInfo5W<'_, CrRegFuncManufactInfoCfgRegFuncManufactInfo5Spec> {
        CfgRegFuncManufactInfo5W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncManufactInfoCfgRegFuncManufactInfo5Spec;
impl crate::RegisterSpec for CrRegFuncManufactInfoCfgRegFuncManufactInfo5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info5::R`](R) reader structure"]
impl crate::Readable for CrRegFuncManufactInfoCfgRegFuncManufactInfo5Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info5::W`](W) writer structure"]
impl crate::Writable for CrRegFuncManufactInfoCfgRegFuncManufactInfo5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO5 to value 0"]
impl crate::Resettable for CrRegFuncManufactInfoCfgRegFuncManufactInfo5Spec {}
