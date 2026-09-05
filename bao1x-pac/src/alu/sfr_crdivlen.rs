#[doc = "Register `SFR_CRDIVLEN` reader"]
pub type R = crate::R<SfrCrdivlenSpec>;
#[doc = "Register `SFR_CRDIVLEN` writer"]
pub type W = crate::W<SfrCrdivlenSpec>;
#[doc = "Field `sfr_crdivlen` reader - sfr_crdivlen read/write control register"]
pub type SfrCrdivlenR = crate::FieldReader<u16>;
#[doc = "Field `sfr_crdivlen` writer - sfr_crdivlen read/write control register"]
pub type SfrCrdivlenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_crdivlen read/write control register"]
    #[inline(always)]
    pub fn sfr_crdivlen(&self) -> SfrCrdivlenR {
        SfrCrdivlenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_crdivlen read/write control register"]
    #[inline(always)]
    pub fn sfr_crdivlen(&mut self) -> SfrCrdivlenW<'_, SfrCrdivlenSpec> {
        SfrCrdivlenW::new(self, 0)
    }
}
#[doc = "See `alu.sv#L141 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L141>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crdivlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crdivlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCrdivlenSpec;
impl crate::RegisterSpec for SfrCrdivlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_crdivlen::R`](R) reader structure"]
impl crate::Readable for SfrCrdivlenSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_crdivlen::W`](W) writer structure"]
impl crate::Writable for SfrCrdivlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CRDIVLEN to value 0"]
impl crate::Resettable for SfrCrdivlenSpec {}
