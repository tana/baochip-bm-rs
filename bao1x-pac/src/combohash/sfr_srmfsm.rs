#[doc = "Register `SFR_SRMFSM` reader"]
pub type R = crate::R<SfrSrmfsmSpec>;
#[doc = "Register `SFR_SRMFSM` writer"]
pub type W = crate::W<SfrSrmfsmSpec>;
#[doc = "Field `mfsm` reader - mfsm read only status register"]
pub type MfsmR = crate::FieldReader;
#[doc = "Field `mfsm` writer - mfsm read only status register"]
pub type MfsmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - mfsm read only status register"]
    #[inline(always)]
    pub fn mfsm(&self) -> MfsmR {
        MfsmR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - mfsm read only status register"]
    #[inline(always)]
    pub fn mfsm(&mut self) -> MfsmW<'_, SfrSrmfsmSpec> {
        MfsmW::new(self, 0)
    }
}
#[doc = "See `combohasha.sv#L210 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L210>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_srmfsm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_srmfsm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSrmfsmSpec;
impl crate::RegisterSpec for SfrSrmfsmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_srmfsm::R`](R) reader structure"]
impl crate::Readable for SfrSrmfsmSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_srmfsm::W`](W) writer structure"]
impl crate::Writable for SfrSrmfsmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SRMFSM to value 0"]
impl crate::Resettable for SfrSrmfsmSpec {}
