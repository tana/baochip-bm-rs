#[doc = "Register `SFR_OPTNW` reader"]
pub type R = crate::R<SfrOptnwSpec>;
#[doc = "Register `SFR_OPTNW` writer"]
pub type W = crate::W<SfrOptnwSpec>;
#[doc = "Field `sfr_optnw` reader - sfr_optnw read/write control register"]
pub type SfrOptnwR = crate::FieldReader<u16>;
#[doc = "Field `sfr_optnw` writer - sfr_optnw read/write control register"]
pub type SfrOptnwW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - sfr_optnw read/write control register"]
    #[inline(always)]
    pub fn sfr_optnw(&self) -> SfrOptnwR {
        SfrOptnwR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - sfr_optnw read/write control register"]
    #[inline(always)]
    pub fn sfr_optnw(&mut self) -> SfrOptnwW<'_, SfrOptnwSpec> {
        SfrOptnwW::new(self, 0)
    }
}
#[doc = "See `pke.sv#L300 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L300>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optnw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optnw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrOptnwSpec;
impl crate::RegisterSpec for SfrOptnwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_optnw::R`](R) reader structure"]
impl crate::Readable for SfrOptnwSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_optnw::W`](W) writer structure"]
impl crate::Writable for SfrOptnwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_OPTNW to value 0"]
impl crate::Resettable for SfrOptnwSpec {}
