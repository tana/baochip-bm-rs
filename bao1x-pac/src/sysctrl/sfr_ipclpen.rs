#[doc = "Register `SFR_IPCLPEN` reader"]
pub type R = crate::R<SfrIpclpenSpec>;
#[doc = "Register `SFR_IPCLPEN` writer"]
pub type W = crate::W<SfrIpclpenSpec>;
#[doc = "Field `sfr_ipclpen` reader - sfr_ipclpen read/write control register"]
pub type SfrIpclpenR = crate::FieldReader<u16>;
#[doc = "Field `sfr_ipclpen` writer - sfr_ipclpen read/write control register"]
pub type SfrIpclpenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_ipclpen read/write control register"]
    #[inline(always)]
    pub fn sfr_ipclpen(&self) -> SfrIpclpenR {
        SfrIpclpenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_ipclpen read/write control register"]
    #[inline(always)]
    pub fn sfr_ipclpen(&mut self) -> SfrIpclpenW<'_, SfrIpclpenSpec> {
        SfrIpclpenW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L812 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L812>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipclpen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipclpen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIpclpenSpec;
impl crate::RegisterSpec for SfrIpclpenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ipclpen::R`](R) reader structure"]
impl crate::Readable for SfrIpclpenSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ipclpen::W`](W) writer structure"]
impl crate::Writable for SfrIpclpenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IPCLPEN to value 0"]
impl crate::Resettable for SfrIpclpenSpec {}
