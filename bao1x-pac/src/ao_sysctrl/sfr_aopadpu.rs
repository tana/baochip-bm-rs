#[doc = "Register `SFR_AOPADPU` reader"]
pub type R = crate::R<SfrAopadpuSpec>;
#[doc = "Register `SFR_AOPADPU` writer"]
pub type W = crate::W<SfrAopadpuSpec>;
#[doc = "Field `sfr_aopadpu` reader - sfr_aopadpu read/write control register"]
pub type SfrAopadpuR = crate::FieldReader<u16>;
#[doc = "Field `sfr_aopadpu` writer - sfr_aopadpu read/write control register"]
pub type SfrAopadpuW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sfr_aopadpu read/write control register"]
    #[inline(always)]
    pub fn sfr_aopadpu(&self) -> SfrAopadpuR {
        SfrAopadpuR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sfr_aopadpu read/write control register"]
    #[inline(always)]
    pub fn sfr_aopadpu(&mut self) -> SfrAopadpuW<'_, SfrAopadpuSpec> {
        SfrAopadpuW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L401 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L401>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_aopadpu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_aopadpu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrAopadpuSpec;
impl crate::RegisterSpec for SfrAopadpuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_aopadpu::R`](R) reader structure"]
impl crate::Readable for SfrAopadpuSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_aopadpu::W`](W) writer structure"]
impl crate::Writable for SfrAopadpuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_AOPADPU to value 0"]
impl crate::Resettable for SfrAopadpuSpec {}
