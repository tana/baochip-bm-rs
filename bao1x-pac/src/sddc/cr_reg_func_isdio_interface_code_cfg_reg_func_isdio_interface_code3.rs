#[doc = "Register `CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE3` reader"]
pub type R = crate::R<CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3Spec>;
#[doc = "Register `CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE3` writer"]
pub type W = crate::W<CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3Spec>;
#[doc = "Field `cfg_reg_func_isdio_interface_code3` reader - cfg_reg_func_isdio_interface_code read/write control register"]
pub type CfgRegFuncIsdioInterfaceCode3R = crate::FieldReader;
#[doc = "Field `cfg_reg_func_isdio_interface_code3` writer - cfg_reg_func_isdio_interface_code read/write control register"]
pub type CfgRegFuncIsdioInterfaceCode3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cfg_reg_func_isdio_interface_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_isdio_interface_code3(&self) -> CfgRegFuncIsdioInterfaceCode3R {
        CfgRegFuncIsdioInterfaceCode3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cfg_reg_func_isdio_interface_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_isdio_interface_code3(
        &mut self,
    ) -> CfgRegFuncIsdioInterfaceCode3W<
        '_,
        CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3Spec,
    > {
        CfgRegFuncIsdioInterfaceCode3W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3Spec;
impl crate::RegisterSpec for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3::R`](R) reader structure"]
impl crate::Readable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code3::W`](W) writer structure"]
impl crate::Writable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE3 to value 0"]
impl crate::Resettable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode3Spec {}
