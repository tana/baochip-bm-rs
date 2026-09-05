#[doc = "Register `CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO1` reader"]
pub type R = crate::R<CrRegFuncManufactInfoCfgRegFuncManufactInfo1Spec>;
#[doc = "Register `CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO1` writer"]
pub type W = crate::W<CrRegFuncManufactInfoCfgRegFuncManufactInfo1Spec>;
#[doc = "Field `cfg_reg_func_manufact_info1` reader - cfg_reg_func_manufact_info read/write control register"]
pub type CfgRegFuncManufactInfo1R = crate::FieldReader<u16>;
#[doc = "Field `cfg_reg_func_manufact_info1` writer - cfg_reg_func_manufact_info read/write control register"]
pub type CfgRegFuncManufactInfo1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cfg_reg_func_manufact_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_manufact_info1(&self) -> CfgRegFuncManufactInfo1R {
        CfgRegFuncManufactInfo1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cfg_reg_func_manufact_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_manufact_info1(
        &mut self,
    ) -> CfgRegFuncManufactInfo1W<'_, CrRegFuncManufactInfoCfgRegFuncManufactInfo1Spec> {
        CfgRegFuncManufactInfo1W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L152 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L152>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncManufactInfoCfgRegFuncManufactInfo1Spec;
impl crate::RegisterSpec for CrRegFuncManufactInfoCfgRegFuncManufactInfo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info1::R`](R) reader structure"]
impl crate::Readable for CrRegFuncManufactInfoCfgRegFuncManufactInfo1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_manufact_info_cfg_reg_func_manufact_info1::W`](W) writer structure"]
impl crate::Writable for CrRegFuncManufactInfoCfgRegFuncManufactInfo1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO1 to value 0"]
impl crate::Resettable for CrRegFuncManufactInfoCfgRegFuncManufactInfo1Spec {}
