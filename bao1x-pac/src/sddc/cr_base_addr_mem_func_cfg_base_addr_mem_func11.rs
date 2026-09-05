#[doc = "Register `CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC11` reader"]
pub type R = crate::R<CrBaseAddrMemFuncCfgBaseAddrMemFunc11Spec>;
#[doc = "Register `CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC11` writer"]
pub type W = crate::W<CrBaseAddrMemFuncCfgBaseAddrMemFunc11Spec>;
#[doc = "Field `cfg_base_addr_mem_func11` reader - cfg_base_addr_mem_func read/write control register"]
pub type CfgBaseAddrMemFunc11R = crate::FieldReader<u32>;
#[doc = "Field `cfg_base_addr_mem_func11` writer - cfg_base_addr_mem_func read/write control register"]
pub type CfgBaseAddrMemFunc11W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - cfg_base_addr_mem_func read/write control register"]
    #[inline(always)]
    pub fn cfg_base_addr_mem_func11(&self) -> CfgBaseAddrMemFunc11R {
        CfgBaseAddrMemFunc11R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - cfg_base_addr_mem_func read/write control register"]
    #[inline(always)]
    pub fn cfg_base_addr_mem_func11(
        &mut self,
    ) -> CfgBaseAddrMemFunc11W<'_, CrBaseAddrMemFuncCfgBaseAddrMemFunc11Spec> {
        CfgBaseAddrMemFunc11W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_base_addr_mem_func_cfg_base_addr_mem_func11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_base_addr_mem_func_cfg_base_addr_mem_func11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrBaseAddrMemFuncCfgBaseAddrMemFunc11Spec;
impl crate::RegisterSpec for CrBaseAddrMemFuncCfgBaseAddrMemFunc11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_base_addr_mem_func_cfg_base_addr_mem_func11::R`](R) reader structure"]
impl crate::Readable for CrBaseAddrMemFuncCfgBaseAddrMemFunc11Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_base_addr_mem_func_cfg_base_addr_mem_func11::W`](W) writer structure"]
impl crate::Writable for CrBaseAddrMemFuncCfgBaseAddrMemFunc11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC11 to value 0"]
impl crate::Resettable for CrBaseAddrMemFuncCfgBaseAddrMemFunc11Spec {}
