#[doc = "Register `SFR_IPCEN` reader"]
pub type R = crate::R<SfrIpcenSpec>;
#[doc = "Register `SFR_IPCEN` writer"]
pub type W = crate::W<SfrIpcenSpec>;
#[doc = "Field `sfr_ipcen` reader - sfr_ipcen read/write control register"]
pub type SfrIpcenR = crate::FieldReader<u16>;
#[doc = "Field `sfr_ipcen` writer - sfr_ipcen read/write control register"]
pub type SfrIpcenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_ipcen read/write control register"]
    #[inline(always)]
    pub fn sfr_ipcen(&self) -> SfrIpcenR {
        SfrIpcenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_ipcen read/write control register"]
    #[inline(always)]
    pub fn sfr_ipcen(&mut self) -> SfrIpcenW<'_, SfrIpcenSpec> {
        SfrIpcenW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L811 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L811>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIpcenSpec;
impl crate::RegisterSpec for SfrIpcenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ipcen::R`](R) reader structure"]
impl crate::Readable for SfrIpcenSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ipcen::W`](W) writer structure"]
impl crate::Writable for SfrIpcenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IPCEN to value 0"]
impl crate::Resettable for SfrIpcenSpec {}
