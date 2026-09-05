#[doc = "Register `SFR_SRDIVLEN` reader"]
pub type R = crate::R<SfrSrdivlenSpec>;
#[doc = "Register `SFR_SRDIVLEN` writer"]
pub type W = crate::W<SfrSrdivlenSpec>;
#[doc = "Field `sfr_srdivlen` reader - sfr_srdivlen read only status register"]
pub type SfrSrdivlenR = crate::FieldReader<u16>;
#[doc = "Field `sfr_srdivlen` writer - sfr_srdivlen read only status register"]
pub type SfrSrdivlenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_srdivlen read only status register"]
    #[inline(always)]
    pub fn sfr_srdivlen(&self) -> SfrSrdivlenR {
        SfrSrdivlenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_srdivlen read only status register"]
    #[inline(always)]
    pub fn sfr_srdivlen(&mut self) -> SfrSrdivlenW<'_, SfrSrdivlenSpec> {
        SfrSrdivlenW::new(self, 0)
    }
}
#[doc = "See `alu.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_srdivlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_srdivlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSrdivlenSpec;
impl crate::RegisterSpec for SfrSrdivlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_srdivlen::R`](R) reader structure"]
impl crate::Readable for SfrSrdivlenSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_srdivlen::W`](W) writer structure"]
impl crate::Writable for SfrSrdivlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SRDIVLEN to value 0"]
impl crate::Resettable for SfrSrdivlenSpec {}
