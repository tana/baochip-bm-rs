#[doc = "Register `CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE2` reader"]
pub type R = crate::R<CrRegFuncManufactCodeCfgRegFuncManufactCode2Spec>;
#[doc = "Register `CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE2` writer"]
pub type W = crate::W<CrRegFuncManufactCodeCfgRegFuncManufactCode2Spec>;
#[doc = "Field `cfg_reg_func_manufact_code2` reader - cfg_reg_func_manufact_code read/write control register"]
pub type CfgRegFuncManufactCode2R = crate::FieldReader<u16>;
#[doc = "Field `cfg_reg_func_manufact_code2` writer - cfg_reg_func_manufact_code read/write control register"]
pub type CfgRegFuncManufactCode2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cfg_reg_func_manufact_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_manufact_code2(&self) -> CfgRegFuncManufactCode2R {
        CfgRegFuncManufactCode2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cfg_reg_func_manufact_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_manufact_code2(
        &mut self,
    ) -> CfgRegFuncManufactCode2W<'_, CrRegFuncManufactCodeCfgRegFuncManufactCode2Spec> {
        CfgRegFuncManufactCode2W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L151>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncManufactCodeCfgRegFuncManufactCode2Spec;
impl crate::RegisterSpec for CrRegFuncManufactCodeCfgRegFuncManufactCode2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code2::R`](R) reader structure"]
impl crate::Readable for CrRegFuncManufactCodeCfgRegFuncManufactCode2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_manufact_code_cfg_reg_func_manufact_code2::W`](W) writer structure"]
impl crate::Writable for CrRegFuncManufactCodeCfgRegFuncManufactCode2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE2 to value 0"]
impl crate::Resettable for CrRegFuncManufactCodeCfgRegFuncManufactCode2Spec {}
