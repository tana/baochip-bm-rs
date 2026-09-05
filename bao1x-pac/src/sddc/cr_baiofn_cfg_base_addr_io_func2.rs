#[doc = "Register `CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC2` reader"]
pub type R = crate::R<CrBaiofnCfgBaseAddrIoFunc2Spec>;
#[doc = "Register `CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC2` writer"]
pub type W = crate::W<CrBaiofnCfgBaseAddrIoFunc2Spec>;
#[doc = "Field `cfg_base_addr_io_func2` reader - cfg_base_addr_io_func read/write control register"]
pub type CfgBaseAddrIoFunc2R = crate::FieldReader<u32>;
#[doc = "Field `cfg_base_addr_io_func2` writer - cfg_base_addr_io_func read/write control register"]
pub type CfgBaseAddrIoFunc2W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - cfg_base_addr_io_func read/write control register"]
    #[inline(always)]
    pub fn cfg_base_addr_io_func2(&self) -> CfgBaseAddrIoFunc2R {
        CfgBaseAddrIoFunc2R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - cfg_base_addr_io_func read/write control register"]
    #[inline(always)]
    pub fn cfg_base_addr_io_func2(
        &mut self,
    ) -> CfgBaseAddrIoFunc2W<'_, CrBaiofnCfgBaseAddrIoFunc2Spec> {
        CfgBaseAddrIoFunc2W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L121 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L121>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_baiofn_cfg_base_addr_io_func2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_baiofn_cfg_base_addr_io_func2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrBaiofnCfgBaseAddrIoFunc2Spec;
impl crate::RegisterSpec for CrBaiofnCfgBaseAddrIoFunc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_baiofn_cfg_base_addr_io_func2::R`](R) reader structure"]
impl crate::Readable for CrBaiofnCfgBaseAddrIoFunc2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_baiofn_cfg_base_addr_io_func2::W`](W) writer structure"]
impl crate::Writable for CrBaiofnCfgBaseAddrIoFunc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC2 to value 0"]
impl crate::Resettable for CrBaiofnCfgBaseAddrIoFunc2Spec {}
