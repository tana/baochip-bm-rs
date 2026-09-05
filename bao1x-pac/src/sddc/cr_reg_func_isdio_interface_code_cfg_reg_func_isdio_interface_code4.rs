#[doc = "Register `CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE4` reader"]
pub type R = crate::R<CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4Spec>;
#[doc = "Register `CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE4` writer"]
pub type W = crate::W<CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4Spec>;
#[doc = "Field `cfg_reg_func_isdio_interface_code4` reader - cfg_reg_func_isdio_interface_code read/write control register"]
pub type CfgRegFuncIsdioInterfaceCode4R = crate::FieldReader;
#[doc = "Field `cfg_reg_func_isdio_interface_code4` writer - cfg_reg_func_isdio_interface_code read/write control register"]
pub type CfgRegFuncIsdioInterfaceCode4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cfg_reg_func_isdio_interface_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_isdio_interface_code4(&self) -> CfgRegFuncIsdioInterfaceCode4R {
        CfgRegFuncIsdioInterfaceCode4R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cfg_reg_func_isdio_interface_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_isdio_interface_code4(
        &mut self,
    ) -> CfgRegFuncIsdioInterfaceCode4W<
        '_,
        CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4Spec,
    > {
        CfgRegFuncIsdioInterfaceCode4W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4Spec;
impl crate::RegisterSpec for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4::R`](R) reader structure"]
impl crate::Readable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code4::W`](W) writer structure"]
impl crate::Writable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE4 to value 0"]
impl crate::Resettable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode4Spec {}
