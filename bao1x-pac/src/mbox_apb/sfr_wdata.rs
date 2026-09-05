#[doc = "Register `SFR_WDATA` reader"]
pub type R = crate::R<SfrWdataSpec>;
#[doc = "Register `SFR_WDATA` writer"]
pub type W = crate::W<SfrWdataSpec>;
#[doc = "Field `sfr_wdata` reader - sfr_wdata read/write control register"]
pub type SfrWdataR = crate::FieldReader<u32>;
#[doc = "Field `sfr_wdata` writer - sfr_wdata read/write control register"]
pub type SfrWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_wdata read/write control register"]
    #[inline(always)]
    pub fn sfr_wdata(&self) -> SfrWdataR {
        SfrWdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_wdata read/write control register"]
    #[inline(always)]
    pub fn sfr_wdata(&mut self) -> SfrWdataW<'_, SfrWdataSpec> {
        SfrWdataW::new(self, 0)
    }
}
#[doc = "See `mbox_v0.1.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L105>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrWdataSpec;
impl crate::RegisterSpec for SfrWdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_wdata::R`](R) reader structure"]
impl crate::Readable for SfrWdataSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_wdata::W`](W) writer structure"]
impl crate::Writable for SfrWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_WDATA to value 0"]
impl crate::Resettable for SfrWdataSpec {}
