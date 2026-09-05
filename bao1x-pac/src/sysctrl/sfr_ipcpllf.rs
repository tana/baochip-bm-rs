#[doc = "Register `SFR_IPCPLLF` reader"]
pub type R = crate::R<SfrIpcpllfSpec>;
#[doc = "Register `SFR_IPCPLLF` writer"]
pub type W = crate::W<SfrIpcpllfSpec>;
#[doc = "Field `sfr_ipcpllf` reader - sfr_ipcpllf read/write control register"]
pub type SfrIpcpllfR = crate::FieldReader<u32>;
#[doc = "Field `sfr_ipcpllf` writer - sfr_ipcpllf read/write control register"]
pub type SfrIpcpllfW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - sfr_ipcpllf read/write control register"]
    #[inline(always)]
    pub fn sfr_ipcpllf(&self) -> SfrIpcpllfR {
        SfrIpcpllfR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - sfr_ipcpllf read/write control register"]
    #[inline(always)]
    pub fn sfr_ipcpllf(&mut self) -> SfrIpcpllfW<'_, SfrIpcpllfSpec> {
        SfrIpcpllfW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L815 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L815>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcpllf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcpllf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIpcpllfSpec;
impl crate::RegisterSpec for SfrIpcpllfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ipcpllf::R`](R) reader structure"]
impl crate::Readable for SfrIpcpllfSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ipcpllf::W`](W) writer structure"]
impl crate::Writable for SfrIpcpllfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IPCPLLF to value 0"]
impl crate::Resettable for SfrIpcpllfSpec {}
