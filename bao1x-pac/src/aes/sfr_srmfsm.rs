#[doc = "Register `SFR_SRMFSM` reader"]
pub type R = crate::R<SfrSrmfsmSpec>;
#[doc = "Register `SFR_SRMFSM` writer"]
pub type W = crate::W<SfrSrmfsmSpec>;
#[doc = "Field `sfr_srmfsm` reader - sfr_srmfsm read only status register"]
pub type SfrSrmfsmR = crate::FieldReader;
#[doc = "Field `sfr_srmfsm` writer - sfr_srmfsm read only status register"]
pub type SfrSrmfsmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sfr_srmfsm read only status register"]
    #[inline(always)]
    pub fn sfr_srmfsm(&self) -> SfrSrmfsmR {
        SfrSrmfsmR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - sfr_srmfsm read only status register"]
    #[inline(always)]
    pub fn sfr_srmfsm(&mut self) -> SfrSrmfsmW<'_, SfrSrmfsmSpec> {
        SfrSrmfsmW::new(self, 0)
    }
}
#[doc = "See `aes.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_srmfsm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_srmfsm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
