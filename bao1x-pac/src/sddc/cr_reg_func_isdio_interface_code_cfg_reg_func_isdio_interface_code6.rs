#[doc = "Register `CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE6` reader"]
pub type R = crate::R<CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6Spec>;
#[doc = "Register `CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE6` writer"]
pub type W = crate::W<CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6Spec>;
#[doc = "Field `cfg_reg_func_isdio_interface_code6` reader - cfg_reg_func_isdio_interface_code read/write control register"]
pub type CfgRegFuncIsdioInterfaceCode6R = crate::FieldReader;
#[doc = "Field `cfg_reg_func_isdio_interface_code6` writer - cfg_reg_func_isdio_interface_code read/write control register"]
pub type CfgRegFuncIsdioInterfaceCode6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cfg_reg_func_isdio_interface_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_isdio_interface_code6(&self) -> CfgRegFuncIsdioInterfaceCode6R {
        CfgRegFuncIsdioInterfaceCode6R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cfg_reg_func_isdio_interface_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_isdio_interface_code6(
        &mut self,
    ) -> CfgRegFuncIsdioInterfaceCode6W<
        '_,
        CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6Spec,
    > {
        CfgRegFuncIsdioInterfaceCode6W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6Spec;
impl crate::RegisterSpec for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6::R`](R) reader structure"]
impl crate::Readable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code6::W`](W) writer structure"]
impl crate::Writable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE6 to value 0"]
impl crate::Resettable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode6Spec {}
