#[doc = "Register `CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE4` reader"]
pub type R = crate::R<CrFnextstdcodeCfgRegFuncExtStdCode4Spec>;
#[doc = "Register `CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE4` writer"]
pub type W = crate::W<CrFnextstdcodeCfgRegFuncExtStdCode4Spec>;
#[doc = "Field `cfg_reg_func_ext_std_code4` reader - cfg_reg_func_ext_std_code read/write control register"]
pub type CfgRegFuncExtStdCode4R = crate::FieldReader;
#[doc = "Field `cfg_reg_func_ext_std_code4` writer - cfg_reg_func_ext_std_code read/write control register"]
pub type CfgRegFuncExtStdCode4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cfg_reg_func_ext_std_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_ext_std_code4(&self) -> CfgRegFuncExtStdCode4R {
        CfgRegFuncExtStdCode4R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cfg_reg_func_ext_std_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_ext_std_code4(
        &mut self,
    ) -> CfgRegFuncExtStdCode4W<'_, CrFnextstdcodeCfgRegFuncExtStdCode4Spec> {
        CfgRegFuncExtStdCode4W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fnextstdcode_cfg_reg_func_ext_std_code4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fnextstdcode_cfg_reg_func_ext_std_code4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrFnextstdcodeCfgRegFuncExtStdCode4Spec;
impl crate::RegisterSpec for CrFnextstdcodeCfgRegFuncExtStdCode4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_fnextstdcode_cfg_reg_func_ext_std_code4::R`](R) reader structure"]
impl crate::Readable for CrFnextstdcodeCfgRegFuncExtStdCode4Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_fnextstdcode_cfg_reg_func_ext_std_code4::W`](W) writer structure"]
impl crate::Writable for CrFnextstdcodeCfgRegFuncExtStdCode4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE4 to value 0"]
impl crate::Resettable for CrFnextstdcodeCfgRegFuncExtStdCode4Spec {}
