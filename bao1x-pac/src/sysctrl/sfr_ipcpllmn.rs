#[doc = "Register `SFR_IPCPLLMN` reader"]
pub type R = crate::R<SfrIpcpllmnSpec>;
#[doc = "Register `SFR_IPCPLLMN` writer"]
pub type W = crate::W<SfrIpcpllmnSpec>;
#[doc = "Field `sfr_ipcpllmn` reader - sfr_ipcpllmn read/write control register"]
pub type SfrIpcpllmnR = crate::FieldReader<u32>;
#[doc = "Field `sfr_ipcpllmn` writer - sfr_ipcpllmn read/write control register"]
pub type SfrIpcpllmnW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - sfr_ipcpllmn read/write control register"]
    #[inline(always)]
    pub fn sfr_ipcpllmn(&self) -> SfrIpcpllmnR {
        SfrIpcpllmnR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - sfr_ipcpllmn read/write control register"]
    #[inline(always)]
    pub fn sfr_ipcpllmn(&mut self) -> SfrIpcpllmnW<'_, SfrIpcpllmnSpec> {
        SfrIpcpllmnW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L814 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L814>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcpllmn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcpllmn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIpcpllmnSpec;
impl crate::RegisterSpec for SfrIpcpllmnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ipcpllmn::R`](R) reader structure"]
impl crate::Readable for SfrIpcpllmnSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ipcpllmn::W`](W) writer structure"]
impl crate::Writable for SfrIpcpllmnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IPCPLLMN to value 0"]
impl crate::Resettable for SfrIpcpllmnSpec {}
