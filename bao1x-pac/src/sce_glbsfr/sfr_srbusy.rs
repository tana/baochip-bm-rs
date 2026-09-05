#[doc = "Register `SFR_SRBUSY` reader"]
pub type R = crate::R<SfrSrbusySpec>;
#[doc = "Register `SFR_SRBUSY` writer"]
pub type W = crate::W<SfrSrbusySpec>;
#[doc = "Field `sr_busy` reader - sr_busy read only status register"]
pub type SrBusyR = crate::FieldReader<u16>;
#[doc = "Field `sr_busy` writer - sr_busy read only status register"]
pub type SrBusyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sr_busy read only status register"]
    #[inline(always)]
    pub fn sr_busy(&self) -> SrBusyR {
        SrBusyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sr_busy read only status register"]
    #[inline(always)]
    pub fn sr_busy(&mut self) -> SrBusyW<'_, SfrSrbusySpec> {
        SrBusyW::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L79 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L79>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_srbusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_srbusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSrbusySpec;
impl crate::RegisterSpec for SfrSrbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_srbusy::R`](R) reader structure"]
impl crate::Readable for SfrSrbusySpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_srbusy::W`](W) writer structure"]
impl crate::Writable for SfrSrbusySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SRBUSY to value 0"]
impl crate::Resettable for SfrSrbusySpec {}
