#[doc = "Register `CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE3` reader"]
pub type R = crate::R<CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3Spec>;
#[doc = "Register `CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE3` writer"]
pub type W = crate::W<CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3Spec>;
#[doc = "Field `cfg_reg_func_isdio_type_sup_code3` reader - cfg_reg_func_isdio_type_sup_code read/write control register"]
pub type CfgRegFuncIsdioTypeSupCode3R = crate::FieldReader;
#[doc = "Field `cfg_reg_func_isdio_type_sup_code3` writer - cfg_reg_func_isdio_type_sup_code read/write control register"]
pub type CfgRegFuncIsdioTypeSupCode3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cfg_reg_func_isdio_type_sup_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_isdio_type_sup_code3(&self) -> CfgRegFuncIsdioTypeSupCode3R {
        CfgRegFuncIsdioTypeSupCode3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cfg_reg_func_isdio_type_sup_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_isdio_type_sup_code3(
        &mut self,
    ) -> CfgRegFuncIsdioTypeSupCode3W<'_, CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3Spec>
    {
        CfgRegFuncIsdioTypeSupCode3W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3Spec;
impl crate::RegisterSpec for CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3::R`](R) reader structure"]
impl crate::Readable for CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_isdio_type_sup_code_cfg_reg_func_isdio_type_sup_code3::W`](W) writer structure"]
impl crate::Writable for CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE3 to value 0"]
impl crate::Resettable for CrRegFuncIsdioTypeSupCodeCfgRegFuncIsdioTypeSupCode3Spec {}
