#[doc = "Register `CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO1` reader"]
pub type R = crate::R<CrRegFuncInfoCfgRegFuncInfo1Spec>;
#[doc = "Register `CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO1` writer"]
pub type W = crate::W<CrRegFuncInfoCfgRegFuncInfo1Spec>;
#[doc = "Field `cfg_reg_func_info1` reader - cfg_reg_func_info read/write control register"]
pub type CfgRegFuncInfo1R = crate::FieldReader<u16>;
#[doc = "Field `cfg_reg_func_info1` writer - cfg_reg_func_info read/write control register"]
pub type CfgRegFuncInfo1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cfg_reg_func_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_info1(&self) -> CfgRegFuncInfo1R {
        CfgRegFuncInfo1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cfg_reg_func_info read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_func_info1(&mut self) -> CfgRegFuncInfo1W<'_, CrRegFuncInfoCfgRegFuncInfo1Spec> {
        CfgRegFuncInfo1W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_func_info_cfg_reg_func_info1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_func_info_cfg_reg_func_info1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegFuncInfoCfgRegFuncInfo1Spec;
impl crate::RegisterSpec for CrRegFuncInfoCfgRegFuncInfo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_func_info_cfg_reg_func_info1::R`](R) reader structure"]
impl crate::Readable for CrRegFuncInfoCfgRegFuncInfo1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_func_info_cfg_reg_func_info1::W`](W) writer structure"]
impl crate::Writable for CrRegFuncInfoCfgRegFuncInfo1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO1 to value 0"]
impl crate::Resettable for CrRegFuncInfoCfgRegFuncInfo1Spec {}
