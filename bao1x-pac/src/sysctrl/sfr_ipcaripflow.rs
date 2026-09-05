#[doc = "Register `SFR_IPCARIPFLOW` reader"]
pub type R = crate::R<SfrIpcaripflowSpec>;
#[doc = "Register `SFR_IPCARIPFLOW` writer"]
pub type W = crate::W<SfrIpcaripflowSpec>;
#[doc = "Field `sfr_ipcaripflow` reader - sfr_ipcaripflow performs action on write of value: 0x57"]
pub type SfrIpcaripflowR = crate::FieldReader<u32>;
#[doc = "Field `sfr_ipcaripflow` writer - sfr_ipcaripflow performs action on write of value: 0x57"]
pub type SfrIpcaripflowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_ipcaripflow performs action on write of value: 0x57"]
    #[inline(always)]
    pub fn sfr_ipcaripflow(&self) -> SfrIpcaripflowR {
        SfrIpcaripflowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_ipcaripflow performs action on write of value: 0x57"]
    #[inline(always)]
    pub fn sfr_ipcaripflow(&mut self) -> SfrIpcaripflowW<'_, SfrIpcaripflowSpec> {
        SfrIpcaripflowW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L810 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L810>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcaripflow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcaripflow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIpcaripflowSpec;
impl crate::RegisterSpec for SfrIpcaripflowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ipcaripflow::R`](R) reader structure"]
impl crate::Readable for SfrIpcaripflowSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ipcaripflow::W`](W) writer structure"]
impl crate::Writable for SfrIpcaripflowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IPCARIPFLOW to value 0"]
impl crate::Resettable for SfrIpcaripflowSpec {}
