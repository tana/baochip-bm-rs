#[doc = "Register `CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO5` reader"]
pub type R = crate::R<CrRegFuncInfoCfgRegFuncInfo5Spec>;
#[doc = "Register `CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO5` writer"]
pub type W = crate::W<CrRegFuncInfoCfgRegFuncInfo5Spec>;
#[doc = "Field `cfg_reg_func_info5` reader - cfg_reg_func_info read/write control register"]
pub type CfgRegFuncInfo5R = crate::FieldReader<u16>;
#[doc = "Field `cfg_reg_func_info5` writer - cfg_reg_func_info read/write control register"]
pub type CfgRegFuncInfo5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cfg_reg_func_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_info5(&self) -> CfgRegFuncInfo5R {
        CfgRegFuncInfo5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cfg_reg_func_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_info5(&mut self) -> CfgRegFuncInfo5W<'_, CrRegFuncInfoCfgRegFuncInfo5Spec> {
        CfgRegFuncInfo5W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_info_cfg_reg_func_info5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_info_cfg_reg_func_info5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncInfoCfgRegFuncInfo5Spec;
impl crate::RegisterSpec for CrRegFuncInfoCfgRegFuncInfo5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_info_cfg_reg_func_info5::R`](R) reader structure"]
impl crate::Readable for CrRegFuncInfoCfgRegFuncInfo5Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_info_cfg_reg_func_info5::W`](W) writer structure"]
impl crate::Writable for CrRegFuncInfoCfgRegFuncInfo5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO5 to value 0"]
impl crate::Resettable for CrRegFuncInfoCfgRegFuncInfo5Spec {}
