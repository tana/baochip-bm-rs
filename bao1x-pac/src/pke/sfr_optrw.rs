#[doc = "Register `SFR_OPTRW` reader"]
pub type R = crate::R<SfrOptrwSpec>;
#[doc = "Register `SFR_OPTRW` writer"]
pub type W = crate::W<SfrOptrwSpec>;
#[doc = "Field `sfr_optrw` reader - sfr_optrw read/write control register"]
pub type SfrOptrwR = crate::FieldReader<u16>;
#[doc = "Field `sfr_optrw` writer - sfr_optrw read/write control register"]
pub type SfrOptrwW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sfr_optrw read/write control register"]
    #[inline(always)]
    pub fn sfr_optrw(&self) -> SfrOptrwR {
        SfrOptrwR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sfr_optrw read/write control register"]
    #[inline(always)]
    pub fn sfr_optrw(&mut self) -> SfrOptrwW<'_, SfrOptrwSpec> {
        SfrOptrwW::new(self, 0)
    }
}
#[doc = "See `pke.sv#L302 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L302>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optrw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optrw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrOptrwSpec;
impl crate::RegisterSpec for SfrOptrwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_optrw::R`](R) reader structure"]
impl crate::Readable for SfrOptrwSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_optrw::W`](W) writer structure"]
impl crate::Writable for SfrOptrwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_OPTRW to value 0"]
impl crate::Resettable for SfrOptrwSpec {}
