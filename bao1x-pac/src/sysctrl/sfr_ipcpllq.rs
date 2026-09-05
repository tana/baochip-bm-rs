#[doc = "Register `SFR_IPCPLLQ` reader"]
pub type R = crate::R<SfrIpcpllqSpec>;
#[doc = "Register `SFR_IPCPLLQ` writer"]
pub type W = crate::W<SfrIpcpllqSpec>;
#[doc = "Field `sfr_ipcpllq` reader - sfr_ipcpllq read/write control register"]
pub type SfrIpcpllqR = crate::FieldReader<u16>;
#[doc = "Field `sfr_ipcpllq` writer - sfr_ipcpllq read/write control register"]
pub type SfrIpcpllqW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - sfr_ipcpllq read/write control register"]
    #[inline(always)]
    pub fn sfr_ipcpllq(&self) -> SfrIpcpllqR {
        SfrIpcpllqR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - sfr_ipcpllq read/write control register"]
    #[inline(always)]
    pub fn sfr_ipcpllq(&mut self) -> SfrIpcpllqW<'_, SfrIpcpllqSpec> {
        SfrIpcpllqW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L816 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L816>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcpllq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcpllq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIpcpllqSpec;
impl crate::RegisterSpec for SfrIpcpllqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ipcpllq::R`](R) reader structure"]
impl crate::Readable for SfrIpcpllqSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ipcpllq::W`](W) writer structure"]
impl crate::Writable for SfrIpcpllqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IPCPLLQ to value 0"]
impl crate::Resettable for SfrIpcpllqSpec {}
