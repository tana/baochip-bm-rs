#[doc = "Register `CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO6` reader"]
pub type R = crate::R<CrRegFuncInfoCfgRegFuncInfo6Spec>;
#[doc = "Register `CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO6` writer"]
pub type W = crate::W<CrRegFuncInfoCfgRegFuncInfo6Spec>;
#[doc = "Field `cfg_reg_func_info6` reader - cfg_reg_func_info read/write control register"]
pub type CfgRegFuncInfo6R = crate::FieldReader<u16>;
#[doc = "Field `cfg_reg_func_info6` writer - cfg_reg_func_info read/write control register"]
pub type CfgRegFuncInfo6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cfg_reg_func_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_info6(&self) -> CfgRegFuncInfo6R {
        CfgRegFuncInfo6R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cfg_reg_func_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_info6(&mut self) -> CfgRegFuncInfo6W<'_, CrRegFuncInfoCfgRegFuncInfo6Spec> {
        CfgRegFuncInfo6W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_info_cfg_reg_func_info6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_info_cfg_reg_func_info6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncInfoCfgRegFuncInfo6Spec;
impl crate::RegisterSpec for CrRegFuncInfoCfgRegFuncInfo6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_info_cfg_reg_func_info6::R`](R) reader structure"]
impl crate::Readable for CrRegFuncInfoCfgRegFuncInfo6Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_info_cfg_reg_func_info6::W`](W) writer structure"]
impl crate::Writable for CrRegFuncInfoCfgRegFuncInfo6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO6 to value 0"]
impl crate::Resettable for CrRegFuncInfoCfgRegFuncInfo6Spec {}
