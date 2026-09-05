#[doc = "Register `CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR7` reader"]
pub type R = crate::R<CrFncisptrCfgRegFuncCisPtr7Spec>;
#[doc = "Register `CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR7` writer"]
pub type W = crate::W<CrFncisptrCfgRegFuncCisPtr7Spec>;
#[doc = "Field `cfg_reg_func_cis_ptr7` reader - cfg_reg_func_cis_ptr read/write control register"]
pub type CfgRegFuncCisPtr7R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_func_cis_ptr7` writer - cfg_reg_func_cis_ptr read/write control register"]
pub type CfgRegFuncCisPtr7W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - cfg_reg_func_cis_ptr read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_cis_ptr7(&self) -> CfgRegFuncCisPtr7R {
        CfgRegFuncCisPtr7R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - cfg_reg_func_cis_ptr read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_cis_ptr7(
        &mut self,
    ) -> CfgRegFuncCisPtr7W<'_, CrFncisptrCfgRegFuncCisPtr7Spec> {
        CfgRegFuncCisPtr7W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L123 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L123>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_fncisptr_cfg_reg_func_cis_ptr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_fncisptr_cfg_reg_func_cis_ptr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrFncisptrCfgRegFuncCisPtr7Spec;
impl crate::RegisterSpec for CrFncisptrCfgRegFuncCisPtr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_fncisptr_cfg_reg_func_cis_ptr7::R`](R) reader structure"]
impl crate::Readable for CrFncisptrCfgRegFuncCisPtr7Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_fncisptr_cfg_reg_func_cis_ptr7::W`](W) writer structure"]
impl crate::Writable for CrFncisptrCfgRegFuncCisPtr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR7 to value 0"]
impl crate::Resettable for CrFncisptrCfgRegFuncCisPtr7Spec {}
