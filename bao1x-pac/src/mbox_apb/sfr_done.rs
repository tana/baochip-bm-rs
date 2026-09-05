#[doc = "Register `SFR_DONE` reader"]
pub type R = crate::R<SfrDoneSpec>;
#[doc = "Register `SFR_DONE` writer"]
pub type W = crate::W<SfrDoneSpec>;
#[doc = "Field `sfr_done` reader - sfr_done performs action on write of value: 0x1"]
pub type SfrDoneR = crate::FieldReader<u32>;
#[doc = "Field `sfr_done` writer - sfr_done performs action on write of value: 0x1"]
pub type SfrDoneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_done performs action on write of value: 0x1"]
    #[inline(always)]
    pub fn sfr_done(&self) -> SfrDoneR {
        SfrDoneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_done performs action on write of value: 0x1"]
    #[inline(always)]
    pub fn sfr_done(&mut self) -> SfrDoneW<'_, SfrDoneSpec> {
        SfrDoneW::new(self, 0)
    }
}
#[doc = "See `mbox_v0.1.sv#L109 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L109>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDoneSpec;
impl crate::RegisterSpec for SfrDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_done::R`](R) reader structure"]
impl crate::Readable for SfrDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_done::W`](W) writer structure"]
impl crate::Writable for SfrDoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DONE to value 0"]
impl crate::Resettable for SfrDoneSpec {}
