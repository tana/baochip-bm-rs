#[doc = "Register `SFR_PMUCRPD` reader"]
pub type R = crate::R<SfrPmucrpdSpec>;
#[doc = "Register `SFR_PMUCRPD` writer"]
pub type W = crate::W<SfrPmucrpdSpec>;
#[doc = "Field `sfrpmucrpd` reader - sfrpmucrpd read/write control register"]
pub type SfrpmucrpdR = crate::FieldReader;
#[doc = "Field `sfrpmucrpd` writer - sfrpmucrpd read/write control register"]
pub type SfrpmucrpdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sfrpmucrpd read/write control register"]
    #[inline(always)]
    pub fn sfrpmucrpd(&self) -> SfrpmucrpdR {
        SfrpmucrpdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - sfrpmucrpd read/write control register"]
    #[inline(always)]
    pub fn sfrpmucrpd(&mut self) -> SfrpmucrpdW<'_, SfrPmucrpdSpec> {
        SfrpmucrpdW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L374 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L374>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmucrpd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmucrpd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPmucrpdSpec;
impl crate::RegisterSpec for SfrPmucrpdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pmucrpd::R`](R) reader structure"]
impl crate::Readable for SfrPmucrpdSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pmucrpd::W`](W) writer structure"]
impl crate::Writable for SfrPmucrpdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PMUCRPD to value 0"]
impl crate::Resettable for SfrPmucrpdSpec {}
