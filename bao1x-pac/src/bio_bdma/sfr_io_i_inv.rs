#[doc = "Register `SFR_IO_I_INV` reader"]
pub type R = crate::R<SfrIoIInvSpec>;
#[doc = "Register `SFR_IO_I_INV` writer"]
pub type W = crate::W<SfrIoIInvSpec>;
#[doc = "Field `sfr_io_i_inv` reader - sfr_io_i_inv read/write control register"]
pub type SfrIoIInvR = crate::FieldReader<u32>;
#[doc = "Field `sfr_io_i_inv` writer - sfr_io_i_inv read/write control register"]
pub type SfrIoIInvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_io_i_inv read/write control register"]
    #[inline(always)]
    pub fn sfr_io_i_inv(&self) -> SfrIoIInvR {
        SfrIoIInvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_io_i_inv read/write control register"]
    #[inline(always)]
    pub fn sfr_io_i_inv(&mut self) -> SfrIoIInvW<'_, SfrIoIInvSpec> {
        SfrIoIInvW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L519 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L519>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_io_i_inv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_io_i_inv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIoIInvSpec;
impl crate::RegisterSpec for SfrIoIInvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_io_i_inv::R`](R) reader structure"]
impl crate::Readable for SfrIoIInvSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_io_i_inv::W`](W) writer structure"]
impl crate::Writable for SfrIoIInvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IO_I_INV to value 0"]
impl crate::Resettable for SfrIoIInvSpec {}
