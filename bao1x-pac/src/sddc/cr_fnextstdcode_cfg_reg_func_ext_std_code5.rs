#[doc = "Register `CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE5` reader"]
pub type R = crate::R<CrFnextstdcodeCfgRegFuncExtStdCode5Spec>;
#[doc = "Register `CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE5` writer"]
pub type W = crate::W<CrFnextstdcodeCfgRegFuncExtStdCode5Spec>;
#[doc = "Field `cfg_reg_func_ext_std_code5` reader - cfg_reg_func_ext_std_code read/write control register"]
pub type CfgRegFuncExtStdCode5R = crate::FieldReader;
#[doc = "Field `cfg_reg_func_ext_std_code5` writer - cfg_reg_func_ext_std_code read/write control register"]
pub type CfgRegFuncExtStdCode5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cfg_reg_func_ext_std_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_ext_std_code5(&self) -> CfgRegFuncExtStdCode5R {
        CfgRegFuncExtStdCode5R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cfg_reg_func_ext_std_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_ext_std_code5(
        &mut self,
    ) -> CfgRegFuncExtStdCode5W<'_, CrFnextstdcodeCfgRegFuncExtStdCode5Spec> {
        CfgRegFuncExtStdCode5W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fnextstdcode_cfg_reg_func_ext_std_code5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fnextstdcode_cfg_reg_func_ext_std_code5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrFnextstdcodeCfgRegFuncExtStdCode5Spec;
impl crate::RegisterSpec for CrFnextstdcodeCfgRegFuncExtStdCode5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_fnextstdcode_cfg_reg_func_ext_std_code5::R`](R) reader structure"]
impl crate::Readable for CrFnextstdcodeCfgRegFuncExtStdCode5Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_fnextstdcode_cfg_reg_func_ext_std_code5::W`](W) writer structure"]
impl crate::Writable for CrFnextstdcodeCfgRegFuncExtStdCode5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE5 to value 0"]
impl crate::Resettable for CrFnextstdcodeCfgRegFuncExtStdCode5Spec {}
