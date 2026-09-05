#[doc = "Register `SFR_ABORT` reader"]
pub type R = crate::R<SfrAbortSpec>;
#[doc = "Register `SFR_ABORT` writer"]
pub type W = crate::W<SfrAbortSpec>;
#[doc = "Field `sfr_abort` reader - sfr_abort performs action on write of value: 0x1"]
pub type SfrAbortR = crate::FieldReader<u32>;
#[doc = "Field `sfr_abort` writer - sfr_abort performs action on write of value: 0x1"]
pub type SfrAbortW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_abort performs action on write of value: 0x1"]
    #[inline(always)]
    pub fn sfr_abort(&self) -> SfrAbortR {
        SfrAbortR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_abort performs action on write of value: 0x1"]
    #[inline(always)]
    pub fn sfr_abort(&mut self) -> SfrAbortW<'_, SfrAbortSpec> {
        SfrAbortW::new(self, 0)
    }
}
#[doc = "See `mbox_v0.1.sv#L108 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L108>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_abort::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_abort::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrAbortSpec;
impl crate::RegisterSpec for SfrAbortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_abort::R`](R) reader structure"]
impl crate::Readable for SfrAbortSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_abort::W`](W) writer structure"]
impl crate::Writable for SfrAbortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ABORT to value 0"]
impl crate::Resettable for SfrAbortSpec {}
