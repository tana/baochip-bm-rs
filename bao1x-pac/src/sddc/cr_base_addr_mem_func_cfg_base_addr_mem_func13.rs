#[doc = "Register `CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC13` reader"]
pub type R = crate::R<CrBaseAddrMemFuncCfgBaseAddrMemFunc13Spec>;
#[doc = "Register `CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC13` writer"]
pub type W = crate::W<CrBaseAddrMemFuncCfgBaseAddrMemFunc13Spec>;
#[doc = "Field `cfg_base_addr_mem_func13` reader - cfg_base_addr_mem_func read/write control register"]
pub type CfgBaseAddrMemFunc13R = crate::FieldReader<u32>;
#[doc = "Field `cfg_base_addr_mem_func13` writer - cfg_base_addr_mem_func read/write control register"]
pub type CfgBaseAddrMemFunc13W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - cfg_base_addr_mem_func read/write control register"]
    #[inline(always)]
    pub fn cfg_base_addr_mem_func13(&self) -> CfgBaseAddrMemFunc13R {
        CfgBaseAddrMemFunc13R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - cfg_base_addr_mem_func read/write control register"]
    #[inline(always)]
    pub fn cfg_base_addr_mem_func13(
        &mut self,
    ) -> CfgBaseAddrMemFunc13W<'_, CrBaseAddrMemFuncCfgBaseAddrMemFunc13Spec> {
        CfgBaseAddrMemFunc13W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrBaseAddrMemFuncCfgBaseAddrMemFunc13Spec;
impl crate::RegisterSpec for CrBaseAddrMemFuncCfgBaseAddrMemFunc13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_base_addr_mem_func_cfg_base_addr_mem_func13::R`](R) reader structure"]
impl crate::Readable for CrBaseAddrMemFuncCfgBaseAddrMemFunc13Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_base_addr_mem_func_cfg_base_addr_mem_func13::W`](W) writer structure"]
impl crate::Writable for CrBaseAddrMemFuncCfgBaseAddrMemFunc13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC13 to value 0"]
impl crate::Resettable for CrBaseAddrMemFuncCfgBaseAddrMemFunc13Spec {}
