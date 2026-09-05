#[doc = "Register `SFR_RDATA` reader"]
pub type R = crate::R<SfrRdataSpec>;
#[doc = "Register `SFR_RDATA` writer"]
pub type W = crate::W<SfrRdataSpec>;
#[doc = "Field `sfr_rdata` reader - sfr_rdata read only status register"]
pub type SfrRdataR = crate::FieldReader<u32>;
#[doc = "Field `sfr_rdata` writer - sfr_rdata read only status register"]
pub type SfrRdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_rdata read only status register"]
    #[inline(always)]
    pub fn sfr_rdata(&self) -> SfrRdataR {
        SfrRdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_rdata read only status register"]
    #[inline(always)]
    pub fn sfr_rdata(&mut self) -> SfrRdataW<'_, SfrRdataSpec> {
        SfrRdataW::new(self, 0)
    }
}
#[doc = "See `mbox_v0.1.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L106>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRdataSpec;
impl crate::RegisterSpec for SfrRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rdata::R`](R) reader structure"]
impl crate::Readable for SfrRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rdata::W`](W) writer structure"]
impl crate::Writable for SfrRdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RDATA to value 0"]
impl crate::Resettable for SfrRdataSpec {}
