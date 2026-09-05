#[doc = "Register `SFR_IO_O_INV` reader"]
pub type R = crate::R<SfrIoOInvSpec>;
#[doc = "Register `SFR_IO_O_INV` writer"]
pub type W = crate::W<SfrIoOInvSpec>;
#[doc = "Field `sfr_io_o_inv` reader - sfr_io_o_inv read/write control register"]
pub type SfrIoOInvR = crate::FieldReader<u32>;
#[doc = "Field `sfr_io_o_inv` writer - sfr_io_o_inv read/write control register"]
pub type SfrIoOInvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_io_o_inv read/write control register"]
    #[inline(always)]
    pub fn sfr_io_o_inv(&self) -> SfrIoOInvR {
        SfrIoOInvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_io_o_inv read/write control register"]
    #[inline(always)]
    pub fn sfr_io_o_inv(&mut self) -> SfrIoOInvW<'_, SfrIoOInvSpec> {
        SfrIoOInvW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L518 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L518>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_io_o_inv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_io_o_inv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIoOInvSpec;
impl crate::RegisterSpec for SfrIoOInvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_io_o_inv::R`](R) reader structure"]
impl crate::Readable for SfrIoOInvSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_io_o_inv::W`](W) writer structure"]
impl crate::Writable for SfrIoOInvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IO_O_INV to value 0"]
impl crate::Resettable for SfrIoOInvSpec {}
