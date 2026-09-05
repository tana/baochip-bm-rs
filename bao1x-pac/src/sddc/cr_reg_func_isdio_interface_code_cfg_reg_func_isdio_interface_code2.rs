#[doc = "Register `CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE2` reader"]
pub type R = crate::R<CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2Spec>;
#[doc = "Register `CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE2` writer"]
pub type W = crate::W<CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2Spec>;
#[doc = "Field `cfg_reg_func_isdio_interface_code2` reader - cfg_reg_func_isdio_interface_code read/write control register"]
pub type CfgRegFuncIsdioInterfaceCode2R = crate::FieldReader;
#[doc = "Field `cfg_reg_func_isdio_interface_code2` writer - cfg_reg_func_isdio_interface_code read/write control register"]
pub type CfgRegFuncIsdioInterfaceCode2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cfg_reg_func_isdio_interface_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_isdio_interface_code2(&self) -> CfgRegFuncIsdioInterfaceCode2R {
        CfgRegFuncIsdioInterfaceCode2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cfg_reg_func_isdio_interface_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_isdio_interface_code2(
        &mut self,
    ) -> CfgRegFuncIsdioInterfaceCode2W<
        '_,
        CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2Spec,
    > {
        CfgRegFuncIsdioInterfaceCode2W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2Spec;
impl crate::RegisterSpec for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2::R`](R) reader structure"]
impl crate::Readable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_isdio_interface_code_cfg_reg_func_isdio_interface_code2::W`](W) writer structure"]
impl crate::Writable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE2 to value 0"]
impl crate::Resettable for CrRegFuncIsdioInterfaceCodeCfgRegFuncIsdioInterfaceCode2Spec {}
