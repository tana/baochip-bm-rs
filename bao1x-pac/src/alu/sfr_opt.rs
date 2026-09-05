#[doc = "Register `SFR_OPT` reader"]
pub type R = crate::R<SfrOptSpec>;
#[doc = "Register `SFR_OPT` writer"]
pub type W = crate::W<SfrOptSpec>;
#[doc = "Field `sfr_opt` reader - sfr_opt read/write control register"]
pub type SfrOptR = crate::FieldReader<u32>;
#[doc = "Field `sfr_opt` writer - sfr_opt read/write control register"]
pub type SfrOptW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_opt read/write control register"]
    #[inline(always)]
    pub fn sfr_opt(&self) -> SfrOptR {
        SfrOptR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_opt read/write control register"]
    #[inline(always)]
    pub fn sfr_opt(&mut self) -> SfrOptW<'_, SfrOptSpec> {
        SfrOptW::new(self, 0)
    }
}
#[doc = "See `alu.sv#L143 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L143>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrOptSpec;
impl crate::RegisterSpec for SfrOptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_opt::R`](R) reader structure"]
impl crate::Readable for SfrOptSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_opt::W`](W) writer structure"]
impl crate::Writable for SfrOptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_OPT to value 0"]
impl crate::Resettable for SfrOptSpec {}
