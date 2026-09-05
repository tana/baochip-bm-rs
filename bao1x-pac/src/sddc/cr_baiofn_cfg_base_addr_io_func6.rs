#[doc = "Register `CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC6` reader"]
pub type R = crate::R<CrBaiofnCfgBaseAddrIoFunc6Spec>;
#[doc = "Register `CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC6` writer"]
pub type W = crate::W<CrBaiofnCfgBaseAddrIoFunc6Spec>;
#[doc = "Field `cfg_base_addr_io_func6` reader - cfg_base_addr_io_func read/write control register"]
pub type CfgBaseAddrIoFunc6R = crate::FieldReader<u32>;
#[doc = "Field `cfg_base_addr_io_func6` writer - cfg_base_addr_io_func read/write control register"]
pub type CfgBaseAddrIoFunc6W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - cfg_base_addr_io_func read/write control register"]
    #[inline(always)]
    pub fn cfg_base_addr_io_func6(&self) -> CfgBaseAddrIoFunc6R {
        CfgBaseAddrIoFunc6R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - cfg_base_addr_io_func read/write control register"]
    #[inline(always)]
    pub fn cfg_base_addr_io_func6(
        &mut self,
    ) -> CfgBaseAddrIoFunc6W<'_, CrBaiofnCfgBaseAddrIoFunc6Spec> {
        CfgBaseAddrIoFunc6W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_baiofn_cfg_base_addr_io_func6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_baiofn_cfg_base_addr_io_func6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrBaiofnCfgBaseAddrIoFunc6Spec;
impl crate::RegisterSpec for CrBaiofnCfgBaseAddrIoFunc6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_baiofn_cfg_base_addr_io_func6::R`](R) reader structure"]
impl crate::Readable for CrBaiofnCfgBaseAddrIoFunc6Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_baiofn_cfg_base_addr_io_func6::W`](W) writer structure"]
impl crate::Writable for CrBaiofnCfgBaseAddrIoFunc6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC6 to value 0"]
impl crate::Resettable for CrBaiofnCfgBaseAddrIoFunc6Spec {}
