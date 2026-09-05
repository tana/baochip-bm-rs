#[doc = "Register `CR_BACSA` reader"]
pub type R = crate::R<CrBacsaSpec>;
#[doc = "Register `CR_BACSA` writer"]
pub type W = crate::W<CrBacsaSpec>;
#[doc = "Field `cfg_base_addr_csa` reader - cfg_base_addr_csa read/write control register"]
pub type CfgBaseAddrCsaR = crate::FieldReader<u32>;
#[doc = "Field `cfg_base_addr_csa` writer - cfg_base_addr_csa read/write control register"]
pub type CfgBaseAddrCsaW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - cfg_base_addr_csa read/write control register"]
    #[inline(always)]
    pub fn cfg_base_addr_csa(&self) -> CfgBaseAddrCsaR {
        CfgBaseAddrCsaR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - cfg_base_addr_csa read/write control register"]
    #[inline(always)]
    pub fn cfg_base_addr_csa(&mut self) -> CfgBaseAddrCsaW<'_, CrBacsaSpec> {
        CfgBaseAddrCsaW::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L120 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L120>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_bacsa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_bacsa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrBacsaSpec;
impl crate::RegisterSpec for CrBacsaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_bacsa::R`](R) reader structure"]
impl crate::Readable for CrBacsaSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_bacsa::W`](W) writer structure"]
impl crate::Writable for CrBacsaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_BACSA to value 0"]
impl crate::Resettable for CrBacsaSpec {}
