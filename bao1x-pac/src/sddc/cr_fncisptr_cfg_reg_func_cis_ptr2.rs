#[doc = "Register `CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR2` reader"]
pub type R = crate::R<CrFncisptrCfgRegFuncCisPtr2Spec>;
#[doc = "Register `CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR2` writer"]
pub type W = crate::W<CrFncisptrCfgRegFuncCisPtr2Spec>;
#[doc = "Field `cfg_reg_func_cis_ptr2` reader - cfg_reg_func_cis_ptr read/write control register"]
pub type CfgRegFuncCisPtr2R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_func_cis_ptr2` writer - cfg_reg_func_cis_ptr read/write control register"]
pub type CfgRegFuncCisPtr2W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - cfg_reg_func_cis_ptr read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_cis_ptr2(&self) -> CfgRegFuncCisPtr2R {
        CfgRegFuncCisPtr2R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - cfg_reg_func_cis_ptr read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_cis_ptr2(
        &mut self,
    ) -> CfgRegFuncCisPtr2W<'_, CrFncisptrCfgRegFuncCisPtr2Spec> {
        CfgRegFuncCisPtr2W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fncisptr_cfg_reg_func_cis_ptr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fncisptr_cfg_reg_func_cis_ptr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrFncisptrCfgRegFuncCisPtr2Spec;
impl crate::RegisterSpec for CrFncisptrCfgRegFuncCisPtr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_fncisptr_cfg_reg_func_cis_ptr2::R`](R) reader structure"]
impl crate::Readable for CrFncisptrCfgRegFuncCisPtr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_fncisptr_cfg_reg_func_cis_ptr2::W`](W) writer structure"]
impl crate::Writable for CrFncisptrCfgRegFuncCisPtr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR2 to value 0"]
impl crate::Resettable for CrFncisptrCfgRegFuncCisPtr2Spec {}
