#[doc = "Register `SFR_BUF` reader"]
pub type R = crate::R<SfrBufSpec>;
#[doc = "Register `SFR_BUF` writer"]
pub type W = crate::W<SfrBufSpec>;
#[doc = "Field `sfr_buf` reader - sfr_buf read only status register"]
pub type SfrBufR = crate::FieldReader<u32>;
#[doc = "Field `sfr_buf` writer - sfr_buf read only status register"]
pub type SfrBufW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_buf read only status register"]
    #[inline(always)]
    pub fn sfr_buf(&self) -> SfrBufR {
        SfrBufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_buf read only status register"]
    #[inline(always)]
    pub fn sfr_buf(&mut self) -> SfrBufW<'_, SfrBufSpec> {
        SfrBufW::new(self, 0)
    }
}
#[doc = "See `trng.sv#L242 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L242>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_buf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_buf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrBufSpec;
impl crate::RegisterSpec for SfrBufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_buf::R`](R) reader structure"]
impl crate::Readable for SfrBufSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_buf::W`](W) writer structure"]
impl crate::Writable for SfrBufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_BUF to value 0"]
impl crate::Resettable for SfrBufSpec {}
