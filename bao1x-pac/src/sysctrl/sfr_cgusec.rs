#[doc = "Register `SFR_CGUSEC` reader"]
pub type R = crate::R<SfrCgusecSpec>;
#[doc = "Register `SFR_CGUSEC` writer"]
pub type W = crate::W<SfrCgusecSpec>;
#[doc = "Field `sfr_cgusec` reader - sfr_cgusec read/write control register"]
pub type SfrCgusecR = crate::FieldReader<u16>;
#[doc = "Field `sfr_cgusec` writer - sfr_cgusec read/write control register"]
pub type SfrCgusecW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_cgusec read/write control register"]
    #[inline(always)]
    pub fn sfr_cgusec(&self) -> SfrCgusecR {
        SfrCgusecR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_cgusec read/write control register"]
    #[inline(always)]
    pub fn sfr_cgusec(&mut self) -> SfrCgusecW<'_, SfrCgusecSpec> {
        SfrCgusecW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L768 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L768>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgusec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgusec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgusecSpec;
impl crate::RegisterSpec for SfrCgusecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgusec::R`](R) reader structure"]
impl crate::Readable for SfrCgusecSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgusec::W`](W) writer structure"]
impl crate::Writable for SfrCgusecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUSEC to value 0"]
impl crate::Resettable for SfrCgusecSpec {}
