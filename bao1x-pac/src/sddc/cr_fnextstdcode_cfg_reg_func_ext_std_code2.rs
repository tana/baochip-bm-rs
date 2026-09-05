#[doc = "Register `CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE2` reader"]
pub type R = crate::R<CrFnextstdcodeCfgRegFuncExtStdCode2Spec>;
#[doc = "Register `CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE2` writer"]
pub type W = crate::W<CrFnextstdcodeCfgRegFuncExtStdCode2Spec>;
#[doc = "Field `cfg_reg_func_ext_std_code2` reader - cfg_reg_func_ext_std_code read/write control register"]
pub type CfgRegFuncExtStdCode2R = crate::FieldReader;
#[doc = "Field `cfg_reg_func_ext_std_code2` writer - cfg_reg_func_ext_std_code read/write control register"]
pub type CfgRegFuncExtStdCode2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cfg_reg_func_ext_std_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_ext_std_code2(&self) -> CfgRegFuncExtStdCode2R {
        CfgRegFuncExtStdCode2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cfg_reg_func_ext_std_code read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_ext_std_code2(
        &mut self,
    ) -> CfgRegFuncExtStdCode2W<'_, CrFnextstdcodeCfgRegFuncExtStdCode2Spec> {
        CfgRegFuncExtStdCode2W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L124 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L124>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fnextstdcode_cfg_reg_func_ext_std_code2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fnextstdcode_cfg_reg_func_ext_std_code2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrFnextstdcodeCfgRegFuncExtStdCode2Spec;
impl crate::RegisterSpec for CrFnextstdcodeCfgRegFuncExtStdCode2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_fnextstdcode_cfg_reg_func_ext_std_code2::R`](R) reader structure"]
impl crate::Readable for CrFnextstdcodeCfgRegFuncExtStdCode2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_fnextstdcode_cfg_reg_func_ext_std_code2::W`](W) writer structure"]
impl crate::Writable for CrFnextstdcodeCfgRegFuncExtStdCode2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE2 to value 0"]
impl crate::Resettable for CrFnextstdcodeCfgRegFuncExtStdCode2Spec {}
