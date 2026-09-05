#[doc = "Register `CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR4` reader"]
pub type R = crate::R<CrFncisptrCfgRegFuncCisPtr4Spec>;
#[doc = "Register `CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR4` writer"]
pub type W = crate::W<CrFncisptrCfgRegFuncCisPtr4Spec>;
#[doc = "Field `cfg_reg_func_cis_ptr4` reader - cfg_reg_func_cis_ptr read/write control register"]
pub type CfgRegFuncCisPtr4R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_func_cis_ptr4` writer - cfg_reg_func_cis_ptr read/write control register"]
pub type CfgRegFuncCisPtr4W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - cfg_reg_func_cis_ptr read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_cis_ptr4(&self) -> CfgRegFuncCisPtr4R {
        CfgRegFuncCisPtr4R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - cfg_reg_func_cis_ptr read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_cis_ptr4(
        &mut self,
    ) -> CfgRegFuncCisPtr4W<'_, CrFncisptrCfgRegFuncCisPtr4Spec> {
        CfgRegFuncCisPtr4W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fncisptr_cfg_reg_func_cis_ptr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fncisptr_cfg_reg_func_cis_ptr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrFncisptrCfgRegFuncCisPtr4Spec;
impl crate::RegisterSpec for CrFncisptrCfgRegFuncCisPtr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_fncisptr_cfg_reg_func_cis_ptr4::R`](R) reader structure"]
impl crate::Readable for CrFncisptrCfgRegFuncCisPtr4Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_fncisptr_cfg_reg_func_cis_ptr4::W`](W) writer structure"]
impl crate::Writable for CrFncisptrCfgRegFuncCisPtr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR4 to value 0"]
impl crate::Resettable for CrFncisptrCfgRegFuncCisPtr4Spec {}
